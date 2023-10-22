use crate::Clear;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{
  borrow::{Borrow, BorrowMut},
  ops::{Deref, DerefMut},
};

#[cfg(feature = "alloc")]
/// [FullAutoClear] with a [String].
pub type FullAutoClearString<'item> = FullAutoClear<'item, String>;
#[cfg(feature = "alloc")]
/// [FullAutoClear] with a [Vec].
pub type FullAutoClearVec<'item, T> = FullAutoClear<'item, Vec<T>>;

#[cfg(feature = "arrayvec")]
/// [FullAutoClear] with a [arrayvec::ArrayVec].
pub type FullAutoClearAV<'item, T, const N: usize> = FullAutoClear<'item, arrayvec::ArrayVec<T, N>>;
#[cfg(feature = "arrayvec")]
/// [FullAutoClear] with a [arrayvec::ArrayString].
pub type FullAutoClearAS<'item, const N: usize> = FullAutoClear<'item, arrayvec::ArrayString<N>>;

#[cfg(feature = "tinyvec")]
/// [FullAutoClear] with a [tinyvec::ArrayVec].
pub type FullAutoClearTAV<'item, A> = FullAutoClear<'item, tinyvec::ArrayVec<A>>;
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
/// [FullAutoClear] with a [tinyvec::TinyVec].
pub type FullAutoClearTV<'item, A> = FullAutoClear<'item, tinyvec::TinyVec<A>>;

/// Any mutable item wrapped in this structure is automatically cleaned when initialized and
/// dropped.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FullAutoClear<'item, T>(&'item mut T)
where
  T: Clear;

impl<T> AsMut<T> for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> AsRef<T> for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn as_ref(&self) -> &T {
    self
  }
}

impl<T> Borrow<T> for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow(&self) -> &T {
    self
  }
}

impl<T> BorrowMut<T> for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    self
  }
}

impl<T> Deref for FullAutoClear<'_, T>
where
  T: Clear,
{
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<T> DerefMut for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0
  }
}

impl<T> Drop for FullAutoClear<'_, T>
where
  T: Clear,
{
  #[inline]
  fn drop(&mut self) {
    self.0.clear();
  }
}

impl<'item, T> From<&'item mut T> for FullAutoClear<'item, T>
where
  T: Clear,
{
  #[inline]
  fn from(from: &'item mut T) -> Self {
    from.clear();
    Self(from)
  }
}
