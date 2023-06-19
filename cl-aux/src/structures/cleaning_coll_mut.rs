use crate::Clear;
use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
};

#[cfg(feature = "alloc")]
pub type CleaningVecMut<'any, T> = CleaningCollMut<'any, T>;

/// A mutable collection reference that clears its internal data when dropped.
pub struct CleaningCollMut<'any, T>(&'any mut T)
where
  T: Clear;

impl<'any, T> AsMut<T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  fn as_mut(&mut self) -> &mut T {
    self
  }
}

impl<'any, T> AsRef<T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  fn as_ref(&self) -> &T {
    self
  }
}

impl<'any, T> Borrow<T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<'any, T> BorrowMut<T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    self
  }
}

impl<'any, T> Deref for CleaningCollMut<'any, T>
where
  T: Clear,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'any, T> DerefMut for CleaningCollMut<'any, T>
where
  T: Clear,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<'any, T> Drop for CleaningCollMut<'any, T>
where
  T: Clear,
{
  fn drop(&mut self) {
    self.0.clear();
  }
}

impl<'any, T> From<&'any mut T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  fn from(from: &'any mut T) -> Self {
    Self(from)
  }
}
