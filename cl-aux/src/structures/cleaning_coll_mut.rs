use crate::Clear;
use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
};

#[cfg(feature = "alloc")]
/// [CleaningCollMut] with the std vector.
pub type CleaningVecMut<'any, T> = CleaningCollMut<'any, T>;

/// A mutable collection reference that clears its internal data when dropped.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CleaningCollMut<'any, T>(&'any mut T)
where
  T: Clear;

impl<T> AsMut<T> for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> AsRef<T> for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_ref(&self) -> &T {
    self
  }
}

impl<T> Borrow<T> for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<T> BorrowMut<T> for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> Deref for CleaningCollMut<'_, T>
where
  T: Clear,
{
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<T> DerefMut for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0
  }
}

impl<T> Drop for CleaningCollMut<'_, T>
where
  T: Clear,
{
  #[inline]
  fn drop(&mut self) {
    self.0.clear();
  }
}

impl<'any, T> From<&'any mut T> for CleaningCollMut<'any, T>
where
  T: Clear,
{
  #[inline]
  fn from(from: &'any mut T) -> Self {
    Self(from)
  }
}
