use crate::{
  coo::CooError,
  csl::{CslError, CslLineConstructorError},
  dense::DenseError,
};
use core::fmt::{Debug, Display, Formatter};

/// Contains all errors related to ndstruct
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
  /// See [cl_aux::Error].
  ClAux(cl_aux::Error),
  /// [CooError]
  Coo(CooError),
  /// [CslError]
  Csl(CslError),
  /// CslLineConstructorError
  CslLineConstructor(CslLineConstructorError),
  /// [DenseError]
  Dense(DenseError),
  /// The internal buffer can't store all necessary data
  InsufficientCapacity,
  /// An Unknown that probably shouldn't have happened
  UnknownError,
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<cl_aux::Error> for Error {
  #[inline]
  fn from(from: cl_aux::Error) -> Self {
    Self::ClAux(from)
  }
}

impl From<CooError> for Error {
  #[inline]
  fn from(from: CooError) -> Self {
    Self::Coo(from)
  }
}

impl From<CslError> for Error {
  #[inline]
  fn from(from: CslError) -> Self {
    Self::Csl(from)
  }
}

impl From<CslLineConstructorError> for Error {
  #[inline]
  fn from(from: CslLineConstructorError) -> Self {
    Self::CslLineConstructor(from)
  }
}

impl From<DenseError> for Error {
  #[inline]
  fn from(from: DenseError) -> Self {
    Self::Dense(from)
  }
}
