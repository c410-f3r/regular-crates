use crate::{IterWrapper, Length, SingleItemStorage};
#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  string::String,
  vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// See [SizeHint::size_hint] for more information.
pub trait SizeHint {
  /// Has the same semantics of [Iterator::size_hint].
  fn size_hint(&self) -> (usize, Option<usize>);
}

impl<T> SizeHint for &T
where
  T: SizeHint,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (*self).size_hint()
  }
}

impl<T> SizeHint for IterWrapper<T>
where
  T: Iterator,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.0.size_hint()
  }
}

/// ```rust
/// assert_eq!(cl_aux::SizeHint::size_hint(&()), (0, Some(0)));
/// ```
impl SizeHint for () {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(0))
  }
}

/// ```rust
/// let mut opt = Some(0);
/// assert_eq!(cl_aux::SizeHint::size_hint(&opt), (1, Some(1)));
/// opt.take();
/// assert_eq!(cl_aux::SizeHint::size_hint(&opt), (0, Some(0)));
/// ```
impl<T> SizeHint for Option<T> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.length(), Some(self.length()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (1, Some(1)));
/// ```
impl<T> SizeHint for SingleItemStorage<T> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (1, Some(1))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
impl<T> SizeHint for [T] {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
impl<T> SizeHint for &'_ [T] {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::SizeHint::size_hint(&mut structure), (3, Some(3)));
/// ```
impl<T> SizeHint for &'_ mut [T] {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
impl<T, const N: usize> SizeHint for [T; N] {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_map();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "alloc")]
impl<K, V> SizeHint for BTreeMap<K, V> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_set();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "alloc")]
impl<V> SizeHint for BTreeSet<V> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_map();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "std")]
impl<K, V, S> SizeHint for HashMap<K, V, S>
where
  S: core::hash::BuildHasher,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_set();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "std")]
impl<V, S> SizeHint for HashSet<V, S>
where
  S: core::hash::BuildHasher,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (5, Some(5)));
/// ```
#[cfg(feature = "alloc")]
impl SizeHint for String {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "alloc")]
impl<T> SizeHint for Vec<T> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (5, Some(5)));
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> SizeHint for arrayvec::ArrayString<N> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> SizeHint for arrayvec::ArrayVec<T, N> {
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "smallvec")]
impl<A> SizeHint for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> SizeHint for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::SizeHint::size_hint(&structure), (3, Some(3)));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> SizeHint for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len(), Some(self.len()))
  }
}
