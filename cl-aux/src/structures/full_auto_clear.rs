use crate::Clear;
use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
};

/// Any mutable item wrapped in this structure is automatically cleaned when initialized and
/// dropped.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FullAutoClear<T>(T)
where
  T: Clear;

impl<T> AsMut<T> for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> AsRef<T> for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn as_ref(&self) -> &T {
    self
  }
}

impl<T> Borrow<T> for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<T> BorrowMut<T> for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> Deref for FullAutoClear<T>
where
  T: Clear,
{
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> Drop for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn drop(&mut self) {
    self.0.clear();
  }
}

impl<T> From<T> for FullAutoClear<T>
where
  T: Clear,
{
  #[inline]
  fn from(mut from: T) -> Self {
    from.clear();
    Self(from)
  }
}
