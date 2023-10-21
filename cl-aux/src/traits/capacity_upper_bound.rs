use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [CapacityUpperBound::capacity_upper_bound] for more information.
pub trait CapacityUpperBound {
  /// The maximum theoretical number of elements a type implementation is able to store.
  const CAPACITY_UPPER_BOUND: usize;

  /// Instance method representing [Self::CAPACITY_UPPER_BOUND].
  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    Self::CAPACITY_UPPER_BOUND
  }
}

impl<T> CapacityUpperBound for &T
where
  T: CapacityUpperBound,
{
  const CAPACITY_UPPER_BOUND: usize = T::CAPACITY_UPPER_BOUND;

  #[inline]
  fn capacity_upper_bound(&self) -> usize {
    (*self).capacity_upper_bound()
  }
}

/// ```rust
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&()), 0);
/// ```
impl CapacityUpperBound for () {
  const CAPACITY_UPPER_BOUND: usize = 0;
}

/// ```rust
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&Some(0)), 1);
/// ```
impl<T> CapacityUpperBound for Option<T> {
  const CAPACITY_UPPER_BOUND: usize = 1;
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Capacity::capacity(&structure), 1);
/// ```
impl<T> CapacityUpperBound for SingleItemStorage<T> {
  const CAPACITY_UPPER_BOUND: usize = 1;
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 3);
/// ```
impl<T, const N: usize> CapacityUpperBound for [T; N] {
  const CAPACITY_UPPER_BOUND: usize = N;
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
impl<T> CapacityUpperBound for &'_ [T] {
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<T>();
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&mut structure), 2305843009213693951);
/// ```
impl<T> CapacityUpperBound for &'_ mut [T] {
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<T>();
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 9223372036854775807);
/// ```
#[cfg(feature = "alloc")]
impl CapacityUpperBound for String {
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<u8>();
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(feature = "alloc")]
impl<T> CapacityUpperBound for Vec<T> {
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<T>();
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 10);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> CapacityUpperBound for arrayvec::ArrayString<N> {
  const CAPACITY_UPPER_BOUND: usize = N;
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> CapacityUpperBound for arrayvec::ArrayVec<T, N> {
  const CAPACITY_UPPER_BOUND: usize = N;
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
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<A::Item>();
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
  const CAPACITY_UPPER_BOUND: usize = A::CAPACITY;
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// assert_eq!(cl_aux::CapacityUpperBound::capacity_upper_bound(&structure), 2305843009213693951);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> CapacityUpperBound for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  const CAPACITY_UPPER_BOUND: usize = _capacity_upper_bound_of_type::<A::Item>();
}

#[inline]
const fn _capacity_upper_bound_of_type<T>() -> usize {
  let size_of_t = core::mem::size_of::<T>();
  let isize_max_usize = isize::MAX.unsigned_abs();
  if let Some(elem) = isize_max_usize.checked_div(size_of_t) {
    elem
  } else {
    0
  }
}
