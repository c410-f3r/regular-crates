use core::fmt::{Debug, Display, Formatter};

/// Groups all possible crate errors
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
  /// It is not possible to insert an already existing element
  AlreadyExistingElement,
  /// Structure can't store more elements
  InsufficientCapacity(u32),
  /// Index is out of structure bounds
  OutOfBounds(u32),
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    Debug::fmt(self, f)
  }
}
