use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [CapacityUpperBound::capacity_upper_bound] for more information.
pub trait CapacityUpperBound {
  /// The maximum theoretical number of elements a type implementation is able to store.
  fn capacity_upper_bound(&self) -> usize;
}

/// ```rust
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&Some(0)), 1);
/// ```
impl<T> CapacityUpperBound for Option<T> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 1);
/// ```
impl<T> CapacityUpperBound for SingleItemStorage<T> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    1
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T, const N: usize> CapacityUpperBound for [T; N] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    N
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T> CapacityUpperBound for &'_ [T] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(structure), 3);
/// ```
impl<T> CapacityUpperBound for &'_ mut [T] {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    self.len()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 9223372036854775807);
/// ```
#[cfg(feature = "alloc")]
impl CapacityUpperBound for String {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    _capacity_upper_bound_for_heap::<u8>()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "alloc")]
impl<T> CapacityUpperBound for Vec<T> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    _capacity_upper_bound_for_heap::<T>()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 10);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> CapacityUpperBound for arrayvec::ArrayString<N> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    N
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> CapacityUpperBound for arrayvec::ArrayVec<T, N> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    N
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "smallvec")]
impl<A> CapacityUpperBound for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    _capacity_upper_bound_for_heap::<A::Item>()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> CapacityUpperBound for staticvec::StaticVec<T, N> {
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    N
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> CapacityUpperBound for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    A::CAPACITY
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> CapacityUpperBound for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    _capacity_upper_bound_for_heap::<A::Item>()
  }
}

#[allow(
  // `E0080` does not allow `T`s larger than `isize::MAX`
  clippy::unwrap_used,
)]
#[inline]
fn _capacity_upper_bound_for_heap<T>() -> usize {
  let size_of_t = core::mem::size_of::<T>();
  let isize_max_usize = isize::MAX.unsigned_abs();
  isize_max_usize.checked_div(size_of_t).unwrap()
}
