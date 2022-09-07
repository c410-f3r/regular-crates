use core::fmt::{Debug, Display, Formatter};

/// Any error related to Csl operations
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CslError {
  /// Data or indices length is greater than the product of all dimensions length
  ///
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::csl::{CslError, CslVec};
  /// let csl = CslVec::new([3], vec![8, 9, 9, 9, 9], vec![0, 5, 5, 5, 5], vec![0, 2]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::DataIndcsLengthGreaterThanDimsLength)));
  /// ```
  DataIndcsLengthGreaterThanDimsLength,

  /// The data length is different than the indices length
  ///
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::csl::{ CslError, CslVec};
  /// let csl = CslVec::new([10], vec![8, 9], vec![0], vec![0, 2]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::DiffDataIndcsLength)));
  /// ```
  DiffDataIndcsLength,

  /// Duplicated indices in a line
  /// ```rust
  /// use ndstruct::csl::{CslArray, CslError};
  /// let csl = CslArray::new([10], [8, 9], [0, 0], [0, 2]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::DuplicatedIndices)));
  /// ```
  DuplicatedIndices,

  /// A index is greater or equal to the innermost dimension length
  ///
  /// ```rust
  /// use ndstruct::csl::{CslArray, CslError};
  /// let csl = CslArray::new([10], [8, 9], [0, 10], [0, 2]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::IndcsGreaterThanEqualDimLength)));
  /// ```
  IndcsGreaterThanEqualDimLength,

  /// Some innermost dimension length is equal to zero
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::csl::{CslError, CslVec};
  /// let csl: ndstruct::Result<CslVec<i32, 5>>;
  /// csl = CslVec::new([1, 2, 3, 0, 5], vec![], vec![], vec![]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::InnermostDimsZero)));
  /// ```
  InnermostDimsZero,

  /// Line iterator must deal with non-empty dimensions
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::csl::{CslVec, CslError};
  /// let csl = CslVec::<i32, 0>::default();
  /// assert_eq!(csl.outermost_line_iter(), Err(ndstruct::Error::Csl(CslError::InvalidIterDim)));
  /// ```
  InvalidIterDim,

  /// Offsets length is different than the dimensions product
  /// (without the innermost dimension) plus one.
  /// This rule doesn't not apply to an empty dimension.
  #[cfg_attr(feature = "alloc", doc = "```rust")]
  #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
  /// use ndstruct::csl::{CslError, CslVec};
  /// let csl = CslVec::new([10], vec![8, 9], vec![0, 5], vec![0, 2, 4]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::InvalidOffsetsLength)));
  /// ```
  InvalidOffsetsLength,

  /// Offsets aren't in ascending order
  ///
  /// ```rust
  /// use ndstruct::csl::{CslArray, CslError};
  /// let csl = CslArray::new([10], [8, 9], [0, 5], [2, 0]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::InvalidOffsetsOrder)));
  /// ```
  InvalidOffsetsOrder,

  /// Last offset is not equal to the nnz
  ///
  /// ```rust
  /// use ndstruct::csl::{CslArray, CslError};
  /// let csl = CslArray::new([10], [8, 9], [0, 5], [0, 4]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::LastOffsetDifferentNnz)));
  /// ```
  LastOffsetDifferentNnz,

  /// nnz is greater than the maximum permitted number of nnz
  #[cfg_attr(all(feature = "alloc", feature = "rand"), doc = "```rust")]
  #[cfg_attr(not(all(feature = "alloc", feature = "rand")), doc = "```ignore")]
  /// use ndstruct::csl::CslVec;
  /// use rand::{Rng, rngs::mock::StepRng};
  /// let mut rng = StepRng::new(0, 1);
  /// let dims = [1, 2, 3]; // Max of 6 elements (1 * 2 * 3)
  /// let csl: ndstruct::Result<CslVec<i32, 3>>;
  /// csl = CslVec::new_controlled_random_rand(dims, 7, &mut rng, |r, _| r.gen());
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(ndstruct::csl::CslError::NnzGreaterThanMaximumNnz)));
  /// ```
  #[cfg(feature = "rand")]
  NnzGreaterThanMaximumNnz,

  /// It isn't possible to have more lines than usize::MAX - 2
  ///
  /// ```rust
  /// use ndstruct::csl::{CslArray, CslError};
  /// let csl = CslArray::new([18446744073709551295, 255, 3026418949592973312], [0], [0], [0, 1]);
  /// assert_eq!(csl, Err(ndstruct::Error::Csl(CslError::OffsLengthOverflow)));
  /// ```
  OffsLengthOverflow,
}

impl Display for CslError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for CslError {}
