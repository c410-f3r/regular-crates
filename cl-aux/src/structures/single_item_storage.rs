use core::slice;

/// A structure that holds one, and only one `T`.
///
/// Behaves like [T; 1] but has additional `Default` and `From` implementations.
#[derive(Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct SingleItemStorage<T>(
  // Single item
  pub T,
);

impl<T> AsMut<[T]> for SingleItemStorage<T> {
  #[inline]
  fn as_mut(&mut self) -> &mut [T] {
    slice::from_mut(&mut self.0)
  }
}

impl<T> AsRef<[T]> for SingleItemStorage<T> {
  #[inline]
  fn as_ref(&self) -> &[T] {
    slice::from_ref(&self.0)
  }
}

impl<T> From<T> for SingleItemStorage<T> {
  #[inline]
  fn from(from: T) -> Self {
    Self(from)
  }
}
