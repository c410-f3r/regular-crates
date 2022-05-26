use core::fmt::{Debug, Display, Formatter};

/// Any error related to `Coo` operations
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum CooError {
  /// Some index isn't in asceding order
  ///
  /// ```rust
  /// use ndstruct::coo::{CooArray, CooError};
  /// let coo = CooArray::new([2, 2], [([1, 1], 8), ([0, 0], 9)]);
  /// assert_eq!(coo, Err(ndstruct::Error::Coo(CooError::InvalidIndcsOrder)));
  /// ```
  InvalidIndcsOrder,

  /// Some index is greater than the defined dimensions
  ///
  /// ```rust
  /// use ndstruct::coo::{CooArray, CooError};
  /// let coo = CooArray::new([2, 2], [([0, 1], 8), ([9, 9], 9)]);
  /// assert_eq!(coo, Err(ndstruct::Error::Coo(CooError::InvalidIndcs)));
  /// ```
  InvalidIndcs,

  /// There are duplicated indices
  ///
  /// ```rust
  /// use ndstruct::coo::{CooArray, CooError};
  /// let coo = CooArray::new([2, 2], [([0, 0], 8), ([0, 0], 9)]);
  /// assert_eq!(coo, Err(ndstruct::Error::Coo(CooError::DuplicatedIndices)));
  DuplicatedIndices,

  /// nnz is greater than the maximum permitted number of nnz
  ///
  #[cfg_attr(all(feature = "alloc", feature = "rand"), doc = "```rust")]
  #[cfg_attr(not(all(feature = "alloc", feature = "rand")), doc = "```ignore")]
  /// use ndstruct::coo::{CooError, CooVec};
  /// use rand::{Rng, rngs::mock::StepRng};
  /// let mut rng = StepRng::new(0, 1);
  /// let dims = [1, 2, 3]; // Max of 6 elements (1 * 2 * 3)
  /// let coo: ndstruct::Result<CooVec<u8, 3>>;
  /// coo = CooVec::new_controlled_random_rand(dims, 10, &mut rng, |r, _| r.gen());
  /// assert_eq!(coo, Err(ndstruct::Error::Coo(CooError::NnzGreaterThanMaximumNnz)));
  /// ```
  #[cfg(feature = "rand")]
  NnzGreaterThanMaximumNnz,
}

impl Display for CooError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for CooError {}
