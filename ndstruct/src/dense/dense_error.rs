use core::fmt::{Debug, Display, Formatter};

/// Any error related to `Coo` operations
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DenseError {
  /// Some index overflows the maximum number of dimensions.
  ///
  /// ```rust
  /// use ndstruct::dense::{DenseArray, DenseError};
  /// let dense = DenseArray::new([2, 3], [1, 2, 3, 4]);
  /// assert_eq!(dense, Err(ndstruct::Error::Dense(DenseError::InvalidIndcs)));
  /// ```
  InvalidIndcs,
}

impl Display for DenseError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}

impl core::error::Error for DenseError {}
