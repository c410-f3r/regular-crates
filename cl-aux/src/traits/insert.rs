#![allow(
  // `_manage_hash` is also used by BTreeMap
  clippy::map_entry
)]

macro_rules! _manage_hash {
  ($hash:expr, $key:expr, $value:expr) => {{
    if $hash.contains_key(&$key) {
      Err(crate::Error::AlreadyExistingElement)
    } else {
      let _maybe_discarded = $hash.insert($key, $value);
      Ok(())
    }
  }};
}

macro_rules! _manage_set {
  ($set:expr, $value:expr) => {{
    if $set.contains(&$value) {
      Err(crate::Error::AlreadyExistingElement)
    } else {
      let _ = $set.insert($value);
      Ok(())
    }
  }};
}

#[cfg(feature = "alloc")]
use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// See [`Insert::insert`] for more information.
pub trait Insert {
  /// Error
  type Error;
  /// Input
  type Input;

  /// Inserts an `Input` element.
  fn insert(&mut self, input: Self::Input) -> Result<(), Self::Error>;
}

impl<T> Insert for &mut T
where
  T: Insert,
{
  type Error = T::Error;
  type Input = T::Input;

  #[inline]
  fn insert(&mut self, input: Self::Input) -> Result<(), Self::Error> {
    (*self).insert(input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_map();
/// cl_aux::Insert::insert(&mut structure, (10, 100));
/// assert_eq!(structure.iter().find(|(k, v)| **k == 10), Some((&10, &100)));
/// ```
#[cfg(feature = "alloc")]
impl<K, V> Insert for BTreeMap<K, V>
where
  K: Ord,
{
  type Error = crate::Error;
  type Input = (K, V);

  #[inline]
  fn insert(&mut self, (key, val): Self::Input) -> Result<(), Self::Error> {
    _manage_hash!(self, key, val)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::b_tree_set();
/// cl_aux::Insert::insert(&mut structure, 10);
/// assert_eq!(structure.iter().find(|&&e| e == 10), Some(&10));
/// ```
#[cfg(feature = "alloc")]
impl<V> Insert for BTreeSet<V>
where
  V: Ord,
{
  type Error = crate::Error;
  type Input = V;

  #[inline]
  fn insert(&mut self, input: Self::Input) -> Result<(), Self::Error> {
    _manage_set!(self, input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_map();
/// cl_aux::Insert::insert(&mut structure, (10, 100));
/// assert_eq!(structure.iter().find(|(k, v)| **k == 10), Some((&10, &100)));
/// ```
#[cfg(feature = "std")]
impl<K, V, S> Insert for HashMap<K, V, S>
where
  K: Eq + core::hash::Hash,
  S: core::hash::BuildHasher,
{
  type Error = crate::Error;
  type Input = (K, V);

  #[inline]
  fn insert(&mut self, (k, v): Self::Input) -> Result<(), Self::Error> {
    _manage_hash!(self, k, v)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::hash_set();
/// cl_aux::Insert::insert(&mut structure, 10);
/// assert_eq!(structure.iter().find(|&&e| e == 10), Some(&10));
/// ```
#[cfg(feature = "std")]
impl<V, S> Insert for HashSet<V, S>
where
  V: core::hash::Hash + Eq,
  S: core::hash::BuildHasher,
{
  type Error = crate::Error;
  type Input = V;

  #[inline]
  fn insert(&mut self, v: Self::Input) -> Result<(), Self::Error> {
    _manage_set!(self, v)
  }
}

/// ```rust
/// let mut opt = None;
/// cl_aux::Insert::insert(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Insert for Option<T> {
  type Error = crate::Error;
  type Input = T;

  #[inline]
  fn insert(&mut self, input: Self::Input) -> Result<(), Self::Error> {
    if self.is_some() {
      Err(crate::Error::InsufficientCapacity(1))
    } else {
      *self = Some(input);
      Ok(())
    }
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "alloc")]
impl<T> Insert for Vec<T> {
  type Error = crate::Error;
  type Input = (usize, T);

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<(), Self::Error> {
    _check_indcs!(self, idx);
    self.insert(idx, elem);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Insert for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = (usize, T);

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<(), Self::Error> {
    _check_indcs!(self, idx);
    self.insert(idx, elem);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "smallvec")]
impl<A> Insert for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = (usize, A::Item);

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<(), Self::Error> {
    _check_indcs!(self, idx);
    self.insert(idx, elem);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Insert for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = (usize, A::Item);

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<(), Self::Error> {
    _check_indcs!(self, idx);
    self.insert(idx, elem);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// cl_aux::Insert::insert(&mut structure, (0, 10));
/// assert_eq!(structure.get(0), Some(&10));
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> Insert for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = (usize, A::Item);

  #[inline]
  fn insert(&mut self, (idx, elem): Self::Input) -> Result<(), Self::Error> {
    _check_indcs!(self, idx);
    self.insert(idx, elem);
    Ok(())
  }
}
