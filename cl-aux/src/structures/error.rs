use core::fmt::{Debug, Display, Formatter};

/// Groups all possible crate errors
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
  /// It is not possible to insert an already existing element
  AlreadyExistingElement,
  /// Structure can't store more elements
  InsufficientCapacity(usize),
  /// Index is out of structure bounds
  OutOfBounds(usize),
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}
