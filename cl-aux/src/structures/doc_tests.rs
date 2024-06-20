//! Instances for documentation tests

/// Mutable slice with three elements
#[doc(hidden)]
#[macro_export]
macro_rules! slice_mut {
  () => {
    &mut [1i32, 2, 3][..]
  };
}

use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  string::String,
  vec::Vec,
};
pub use slice_mut;
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// Array with three elements
#[inline]
#[must_use]
pub fn array() -> [i32; 3] {
  [1, 2, 3]
}

/// [`arrayvec::ArrayString`] filled with "Hello"
#[cfg(feature = "arrayvec")]
#[inline]
#[must_use]
pub fn array_string() -> arrayvec::ArrayString<10> {
  let mut s = arrayvec::ArrayString::new();
  s.push_str("Hello");
  s
}

/// `ArrayVec` with three elements
#[cfg(feature = "arrayvec")]
#[inline]
#[must_use]
pub fn array_vec() -> arrayvec::ArrayVec<i32, 5> {
  let mut vec = arrayvec::ArrayVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

/// `BTreeMap` with three elements
#[cfg(feature = "alloc")]
#[inline]
#[must_use]
pub fn b_tree_map() -> BTreeMap<i32, i32> {
  [(0, 1), (1, 2), (2, 3)].iter().copied().collect()
}

/// `BTreeSet` with three elements
#[cfg(feature = "alloc")]
#[inline]
#[must_use]
pub fn b_tree_set() -> BTreeSet<i32> {
  [1, 2, 3].iter().copied().collect()
}

/// `HashMap` with three elements
#[cfg(feature = "std")]
#[inline]
#[must_use]
pub fn hash_map() -> HashMap<i32, i32> {
  [(0, 1), (1, 2), (2, 3)].iter().copied().collect()
}

/// `HashSet` with three elements
#[cfg(feature = "std")]
#[inline]
#[must_use]
pub fn hash_set() -> HashSet<i32> {
  [1, 2, 3].iter().copied().collect()
}

#[inline]
#[must_use]
/// `SingleItemStorage` containing `'1i32`
pub fn single_item_storage() -> SingleItemStorage<i32> {
  1i32.into()
}

#[inline]
#[must_use]
/// Slice with three elements
pub fn slice() -> &'static [i32] {
  &[1, 2, 3]
}

#[cfg(feature = "smallvec")]
#[inline]
#[must_use]
/// `SmallVec` with three elements
pub fn small_vec() -> smallvec::SmallVec<[i32; 5]> {
  let mut vec = smallvec::SmallVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

/// [String] filled with "Hello"
#[cfg(feature = "alloc")]
#[inline]
#[must_use]
pub fn string() -> String {
  String::from("Hello")
}

#[cfg(feature = "tinyvec")]
#[inline]
#[must_use]
/// `ArrayVec` with three elements
pub fn tiny_vec_array_vec() -> tinyvec::ArrayVec<[i32; 5]> {
  let mut vec = tinyvec::ArrayVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(all(feature = "alloc", feature = "tinyvec"))]
#[inline]
#[must_use]
/// `TinyVec` with three elements
pub fn tiny_vec_tiny_vec() -> tinyvec::TinyVec<[i32; 5]> {
  let mut vec = tinyvec::TinyVec::new();
  vec.extend([1, 2, 3].iter().copied());
  vec
}

#[cfg(feature = "alloc")]
#[inline]
#[must_use]
/// `Vec` with three elements
pub fn vec() -> Vec<i32> {
  let mut vec = Vec::with_capacity(5);
  vec.extend([1, 2, 3].iter().copied());
  vec
}
