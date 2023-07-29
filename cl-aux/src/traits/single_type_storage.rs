use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Anything that can hold a collection of items
pub trait SingleTypeStorage {
  /// Storage item
  type Item;
}

impl<T> SingleTypeStorage for &T
where
  T: SingleTypeStorage,
{
  type Item = T::Item;
}

impl<T> SingleTypeStorage for &mut T
where
  T: SingleTypeStorage,
{
  type Item = T::Item;
}

impl<T> SingleTypeStorage for Option<T> {
  type Item = T;
}

impl<T> SingleTypeStorage for SingleItemStorage<T> {
  type Item = T;
}

impl<T, const N: usize> SingleTypeStorage for [T; N] {
  type Item = T;
}

impl<T> SingleTypeStorage for &'_ [T] {
  type Item = T;
}

impl<T> SingleTypeStorage for &'_ mut [T] {
  type Item = T;
}

#[cfg(feature = "alloc")]
impl<T> SingleTypeStorage for Vec<T> {
  type Item = T;
}

#[cfg(feature = "arrayvec")]
impl<T, const N: usize> SingleTypeStorage for arrayvec::ArrayVec<T, N> {
  type Item = T;
}

#[cfg(feature = "smallvec")]
impl<A> SingleTypeStorage for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Item = A::Item;
}

#[cfg(feature = "tinyvec")]
impl<A> SingleTypeStorage for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Item = A::Item;
}

#[cfg(feature = "tinyvec")]
impl<A> SingleTypeStorage for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Item = A::Item;
}
