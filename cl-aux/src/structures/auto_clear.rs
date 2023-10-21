use crate::Clear;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
};

#[cfg(feature = "alloc")]
/// [AutoClear] with a [String].
pub type AutClearString<'item> = AutoClear<'item, String>;
#[cfg(feature = "alloc")]
/// [AutoClear] with a [Vec].
pub type AutClearVec<'item, T> = AutoClear<'item, Vec<T>>;

#[cfg(feature = "arrayvec")]
/// [AutoClear] with a [arrayvec::ArrayVec].
pub type AutClearAV<'item, T, const N: usize> = AutoClear<'item, arrayvec::ArrayVec<T, N>>;
#[cfg(feature = "arrayvec")]
/// [AutoClear] with a [arrayvec::ArrayString].
pub type AutClearAS<'item, const N: usize> = AutoClear<'item, arrayvec::ArrayString<N>>;

#[cfg(feature = "tinyvec")]
/// [AutoClear] with a [tinyvec::ArrayVec].
pub type AutClearTAV<'item, A> = AutoClear<'item, tinyvec::ArrayVec<A>>;
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
/// [AutoClear] with a [tinyvec::TinyVec].
pub type AutClearTV<'item, A> = AutoClear<'item, tinyvec::TinyVec<A>>;

/// Any mutable item wrapped in this structure is automatically cleaned when dropped.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AutoClear<'item, T>(&'item mut T)
where
  T: Clear;

impl<T> AsMut<T> for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> AsRef<T> for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_ref(&self) -> &T {
    self
  }
}

impl<T> Borrow<T> for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<T> BorrowMut<T> for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> Deref for AutoClear<'_, T>
where
  T: Clear,
{
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<T> DerefMut for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0
  }
}

impl<T> Drop for AutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn drop(&mut self) {
    self.0.clear();
  }
}

impl<'item, T> From<&'item mut T> for AutoClear<'item, T>
where
  T: Clear,
{
  #[inline]
  fn from(from: &'item mut T) -> Self {
    Self(from)
  }
}
