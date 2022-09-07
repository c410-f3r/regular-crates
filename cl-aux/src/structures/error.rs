use core::fmt::{Debug, Display, Formatter};

/// Groups all possible crate errors
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
  /// It is not possible to insert an already existing element
  AlreadyExistingElement(&'static str, &'static str),
  /// Structure can't store more elements
  InsufficientCapacity(&'static str, usize),
  /// Index is out of structure bounds
  OutOfBounds(&'static str, usize),
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}
