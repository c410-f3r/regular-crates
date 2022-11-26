use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [Capacity::capacity] for more information.
pub trait Capacity {
  /// The number of elements the implementation has pre-allocated as an internal buffer. Not
  /// necessarily the current number of inserted elements.
  fn capacity(&self) -> usize;
}

/// ```rust
/// assert_eq!(cl_aux::Capacity::capacity(&()), 0);
/// ```
impl Capacity for () {
  #[inline]
  fn capacity(&self) -> usize {
    0
  }
}

/// ```rust
/// assert_eq!(cl_aux::Capacity::capacity(&Some(0)), 1);
/// ```
impl<T> Capacity for Option<T> {
  #[inline]
  fn capacity(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 1);
/// ```
impl<T> Capacity for SingleItemStorage<T> {
  #[inline]
  fn capacity(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 3);
/// ```
impl<T, const N: usize> Capacity for [T; N] {
  #[inline]
  fn capacity(&self) -> usize {
    N
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::Length::length(&structure), 3);
/// ```
impl<T> Capacity for &'_ [T] {
  #[inline]
  fn capacity(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::Length::length(&mut structure), 3);
/// ```
impl<T> Capacity for &'_ mut [T] {
  #[inline]
  fn capacity(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "alloc")]
impl Capacity for String {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "alloc")]
impl<T> Capacity for Vec<T> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 10);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Capacity for arrayvec::ArrayString<N> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Capacity for arrayvec::ArrayVec<T, N> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Capacity for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Capacity for staticvec::StaticVec<T, N> {
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Capacity for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 5);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Capacity for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity(&self) -> usize {
    self.capacity()
  }
}
