use crate::csl::{csl_utils::manage_last_offset, Csl};
use cl_aux::{Push, SingleTypeStorage};
use core::fmt::{Debug, Display, Formatter};

/// Constructs valid lines in a easy and interactive manner, abstracting away the complexity
/// of the compressed sparse format.
#[derive(Debug, Eq, PartialEq)]
pub struct CslLineConstructor<'csl, DS, IS, OS, const D: usize> {
  csl: &'csl mut Csl<DS, IS, OS, D>,
  curr_dim_idx: usize,
  last_off: usize,
}

impl<'csl, DATA, DS, IS, OS, const D: usize> CslLineConstructor<'csl, DS, IS, OS, D>
where
  DS: AsRef<[DATA]> + Push<DATA, Output = ()> + SingleTypeStorage<Item = DATA>,
  IS: AsRef<[usize]> + Push<usize, Output = ()>,
  OS: AsRef<[usize]> + Push<usize, Output = ()>,
{
  #[inline]
  pub(crate) fn new(csl: &'csl mut Csl<DS, IS, OS, D>) -> crate::Result<Self> {
    if D == 0 {
      return Err(CslLineConstructorError::EmptyDimension.into());
    }
    let curr_dim_idx = if let Some(idx) = csl.dims.iter().copied().position(|x| x != 0) {
      idx
    } else {
      csl.dims.len()
    };
    let last_off = manage_last_offset(&mut csl.offs)?;
    Ok(Self { csl, curr_dim_idx, last_off })
  }

  /// Jumps to the next outermost dimension, i.e., from right to left.
  ///
  /// # Example
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// # fn main() -> ndstruct::Result<()> {
  /// use ndstruct::csl::{CslRef, CslVec};
  /// let mut csl = CslVec::<i32, 3>::default();
  /// csl
  ///   .constructor()?
  ///   .next_outermost_dim(3)?
  ///   .push_line([(0, 1)].iter().copied())?
  ///   .next_outermost_dim(4)?
  ///   .push_line([(1, 2)].iter().copied())?
  ///   .push_empty_line()?
  ///   .push_line([(0, 3), (1,4)].iter().copied())?;
  /// assert_eq!(
  ///   csl.sub_dim(0..4),
  ///   CslRef::new([4, 3], &[1, 2, 3, 4][..], &[0, 1, 0, 1][..], &[0, 1, 2, 2, 4][..]).ok()
  /// );
  /// # Ok(()) }
  #[inline]
  pub fn next_outermost_dim(mut self, len: usize) -> crate::Result<Self> {
    self.curr_dim_idx =
      self.curr_dim_idx.checked_sub(1).ok_or(CslLineConstructorError::DimsOverflow)?;
    *self.curr_dim() = len;
    Ok(self)
  }

  /// This is the same as `push_line([].iter(), [].iter())`.
  ///
  /// # Example
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// # fn main() -> ndstruct::Result<()> {
  /// use ndstruct::csl::{CslRef, CslVec};
  /// let mut csl = CslVec::<i32, 3>::default();
  /// let constructor = csl.constructor()?.next_outermost_dim(3)?;
  /// constructor.push_empty_line()?.next_outermost_dim(2)?.push_empty_line()?;
  /// assert_eq!(csl.line([0, 0, 0]), CslRef::new([3], &[][..], &[][..], &[0, 0][..]).ok());
  /// # Ok(()) }
  #[inline]
  pub fn push_empty_line(self) -> crate::Result<Self> {
    self.csl.offs.push(self.last_off).map_err(|_err| crate::Error::InsufficientCapacity)?;
    Ok(self)
  }

  /// Pushes a new compressed line, modifying the internal structure and if applicable,
  /// increases the current dimension length.
  ///
  /// The iterator will be truncated to (usize::Max - last offset value + 1) or (last dimension value)
  /// and it can lead to a situation where no values will be inserted.
  ///
  /// # Arguments
  ///
  /// * `data`:  Iterator of cloned items.
  /// * `indcs`: Iterator of the respective indices of each item.
  ///
  /// # Example
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// # fn main() -> ndstruct::Result<()> {
  /// use ndstruct::csl::{CslRef, CslVec};
  /// let mut csl = CslVec::<i32, 3>::default();
  /// csl.constructor()?.next_outermost_dim(50)?.push_line([(1, 1), (40, 2)].iter().copied())?;
  /// let line = csl.line([0, 0, 0]);
  /// assert_eq!(line, CslRef::new([50], &[1, 2][..], &[1, 40][..], &[0, 2][..]).ok());
  /// # Ok(()) }
  #[inline]
  pub fn push_line(mut self, di: impl Iterator<Item = (usize, DATA)>) -> crate::Result<Self> {
    let nnz_iter = 1..self.last_dim().saturating_add(1);
    let off_iter = self.last_off.saturating_add(1)..;
    let mut iter = off_iter.zip(nnz_iter.zip(di));
    let mut last_off = self.last_off;
    let mut nnz = 0;

    let mut push = |curr_last_off, curr_nnz, idx, value| {
      self.csl.indcs.push(idx).map_err(|_err| crate::Error::InsufficientCapacity)?;
      self.csl.data.push(value).map_err(|_err| crate::Error::InsufficientCapacity)?;
      nnz = curr_nnz;
      last_off = curr_last_off;
      Ok::<(), crate::Error>(())
    };

    let mut last_line_idx = if let Some((curr_last_off, (curr_nnz, (idx, value)))) = iter.next() {
      push(curr_last_off, curr_nnz, idx, value)?;
      idx
    } else {
      return self.push_empty_line();
    };

    for (curr_last_off, (curr_nnz, (idx, value))) in iter {
      if idx <= last_line_idx {
        return Err(CslLineConstructorError::UnsortedIndices.into());
      }
      push(curr_last_off, curr_nnz, idx, value)?;
      last_line_idx = idx;
    }

    if nnz == 0 {
      return self.push_empty_line();
    }
    self.csl.offs.push(last_off).map_err(|_err| crate::Error::InsufficientCapacity)?;
    self.last_off = last_off;
    Ok(self)
  }

  #[allow(
    // self.curr_dim_idx always points to a valid reference
    clippy::unwrap_used
  )]
  fn curr_dim(&mut self) -> &mut usize {
    self.csl.dims.get_mut(self.curr_dim_idx).unwrap()
  }

  #[allow(
    // Constructor doesn't contain empty dimensions
    clippy::unwrap_used
  )]
  fn last_dim(&mut self) -> usize {
    *self.csl.dims.last().unwrap()
  }
}

/// Contains all errors related to CslLineConstructor.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CslLineConstructorError {
  /// The maximum number of dimenstions has been reached
  DimsOverflow,
  /// All indices must be in ascending order
  UnsortedIndices,
  /// It isn't possible to construct new elements in an empty dimension
  EmptyDimension,
  /// The maximum number of lines for the currention dimension has been reached
  MaxNumOfLines,
}

impl Display for CslLineConstructorError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for CslLineConstructorError {}
