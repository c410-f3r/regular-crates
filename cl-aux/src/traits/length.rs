#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  string::String,
  vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

use crate::SingleItemStorage;

/// See [Length::length] for more information.
pub trait Length {
  /// Holds a certain number of elements.
  fn length(&self) -> usize;
}

impl<T> Length for &T
where
  T: Length,
{
  #[inline]
  fn length(&self) -> usize {
    (*self).length()
  }
}

/// ```rust
/// assert_eq!(cl_aux::Length::length(&()), 0);
/// ```
impl Length for () {
  #[inline]
  fn length(&self) -> usize {
    0
  }
}

/// ```rust
/// let mut opt = Some(0);
/// assert_eq!(cl_aux::Length::length(&opt), 1);
/// opt.take();
/// assert_eq!(cl_aux::Length::length(&opt), 0);
/// ```
impl<T> Length for Option<T> {
  #[inline]
  fn length(&self) -> usize {
    usize::from(self.is_some())
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Length::length(&structure), 1);
/// ```
impl<T> Length for SingleItemStorage<T> {
  #[inline]
  fn length(&self) -> usize {
    1
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
impl<T> Length for &'_ [T] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::Length::length(&mut structure), 3);
/// ```
impl<T> Length for &'_ mut [T] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
impl<T, const N: usize> Length for [T; N] {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_map();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "alloc")]
impl<K, V> Length for BTreeMap<K, V> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_set();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "alloc")]
impl<V> Length for BTreeSet<V> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_map();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "std")]
impl<K, V, S> Length for HashMap<K, V, S>
where
  S: core::hash::BuildHasher,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_set();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "std")]
impl<V, S> Length for HashSet<V, S>
where
  S: core::hash::BuildHasher,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::Length::length(&structure), 5);
/// ```
#[cfg(feature = "alloc")]
impl Length for String {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "alloc")]
impl<T> Length for Vec<T> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::Length::length(&structure), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Length for arrayvec::ArrayString<N> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Length for arrayvec::ArrayVec<T, N> {
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Length for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Length for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> Length for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn length(&self) -> usize {
    self.len()
  }
}
