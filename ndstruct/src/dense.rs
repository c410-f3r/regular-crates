//! Dense
//!
//! A fully filled contiguous space of memory. If 1d, then this structure is the same as a vector;
//! if 2d, then this structure is the same as a matrix; if 3d, then this structure is the same as
//! a cube and so on for higher dimensions.

mod dense_error;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use cl_aux::{ArrayWrapper, SingleTypeStorage};
pub use dense_error::*;

/// Dense backed by a static array.
pub type DenseArray<DATA, const D: usize, const DN: usize> = Dense<[DATA; DN], D>;
/// Dense backed by a mutable slice
pub type DenseMut<'data, DATA, const D: usize> = Dense<&'data mut [DATA], D>;
/// Dense backed by a slice
pub type DenseRef<'data, DATA, const D: usize> = Dense<&'data [DATA], D>;
#[cfg(feature = "alloc")]
/// Dense backed by a dynamic vector.
pub type DenseVec<DATA, const D: usize> = Dense<Vec<DATA>, D>;

/// Base structure for all [Dense] variants.
///
/// # Types
///
/// * `D`: Number of dimensions
/// * `DS`: Data Storage
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dense<DS, const D: usize> {
  pub(crate) data: DS,
  pub(crate) dims: ArrayWrapper<usize, D>,
}

impl<DS, const D: usize> Dense<DS, D> {
  /// The definitions of all dimensions.
  ///
  /// # Example
  ///
  /// ```rust
  /// use ndstruct::doc_tests::dense_array_3;
  /// assert_eq!(dense_array_3().dims(), &[4, 3, 3]);
  /// ```
  #[inline]
  pub fn dims(&self) -> &[usize; D] {
    &self.dims
  }
}

impl<DATA, DS, const D: usize> Dense<DS, D>
where
  DS: AsRef<[<DS as SingleTypeStorage>::Item]> + SingleTypeStorage<Item = DATA>,
{
  /// Creates a valid [Dense] instance.
  ///
  /// # Arguments
  ///
  /// * `dims`: Array of dimensions
  /// * `data`: Data collection
  ///
  /// # Example
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::dense::{DenseArray, DenseVec};
  /// // Matrix ([1, 2, 3], [4, 5, 6])
  /// let mut _matrix = DenseArray::new([2, 3], [1, 2, 3, 4, 5, 6]);
  /// // A bunch of nothing for your overflow needs
  /// let _over_nine: ndstruct::Result<DenseVec<(), 9001>>;
  /// _over_nine = DenseVec::new([0; 9001], vec![]);
  /// ```
  #[inline]
  pub fn new(dims: [usize; D], data: DS) -> crate::Result<Self> {
    if dims.iter().copied().any(|elem| elem >= D) {
      return Err(DenseError::InvalidIndcs.into());
    }
    Ok(Self { data, dims: dims.into() })
  }

  /// The data that is being stored.
  ///
  /// # Example
  ///
  /// ```rust
  /// use ndstruct::doc_tests::dense_array_3;
  /// assert_eq!(dense_array_3().data(), &[
  ///    1,  2,  3,  4,  5,  6,  7,  8,  9,
  ///   10, 11, 12, 13, 14, 15, 16, 17, 18,
  ///   19, 20, 21, 22, 23, 24, 25, 26, 27,
  ///   28, 29, 30, 31, 32, 33, 34, 35, 36,
  /// ]);
  /// ```
  #[inline]
  pub fn data(&self) -> &[DATA] {
    self.data.as_ref()
  }

  /// If any, retrieves an immutable data reference of a given set of indices.
  ///
  /// # Arguments
  ///
  /// * `indcs`: Indices of the desired data location
  ///
  /// # Example
  ///
  /// ```rust
  /// use ndstruct::doc_tests::dense_array_3;
  /// let elem = dense_array_3();
  /// assert_eq!(elem.value([0, 0, 0]), Some(&1));
  /// assert_eq!(elem.value([0, 0, 2]), Some(&3));
  /// assert_eq!(elem.value([0, 2, 2]), Some(&9));
  /// assert_eq!(elem.value([3, 2, 2]), Some(&36));
  /// assert_eq!(elem.value([3, 2, 3]), None);
  /// ```
  #[inline]
  pub fn value(&self, indcs: [usize; D]) -> Option<&DATA> {
    self.data().get(self.idx(indcs))
  }

  // 1 * rows * cols * z
  // 1 * rows        * y
  // 1               * x
  #[inline]
  fn dim_stride(&self, dim_idx: usize, target_dim_idx: usize) -> usize {
    self.dims.iter().copied().skip(1).rev().skip(dim_idx).chain([target_dim_idx]).product()
  }

  #[inline]
  fn idx(&self, indcs: [usize; D]) -> usize {
    let mut rslt: usize = 0;
    for (dim_idx, target_dim_idx) in (0..self.dims.len()).zip(indcs) {
      rslt = rslt.wrapping_add(self.dim_stride(dim_idx, target_dim_idx));
    }
    rslt
  }
}
