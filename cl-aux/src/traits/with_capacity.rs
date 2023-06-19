#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [WithCapacity::with_capacity] for more information.
pub trait WithCapacity
where
  Self: Sized,
{
  /// Error
  type Error;
  /// Input
  type Input;

  /// Creates a new instance based on an initial holding capacity provided by `Input`.
  fn with_capacity(input: Self::Input) -> Self;
}

/// ```rust
/// use cl_aux::Capacity;
/// let structure: [i32; 5];
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
impl<T, const N: usize> WithCapacity for [T; N]
where
  T: Default,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    [(); N].map(|_| T::default())
  }
}

/// ```rust
/// let structure: String = cl_aux::WithCapacity::with_capacity(2);
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl WithCapacity for String {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Self {
    String::with_capacity(input)
  }
}

/// ```rust
/// let structure: Vec<i32> = cl_aux::WithCapacity::with_capacity(2);
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl<T> WithCapacity for Vec<T> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Self {
    Vec::with_capacity(input)
  }
}

/// ```rust
/// let structure: arrayvec::ArrayString<5>;
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> WithCapacity for arrayvec::ArrayString<N> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    arrayvec::ArrayString::new()
  }
}

/// ```rust
/// let structure: arrayvec::ArrayVec<i32, 5>;
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> WithCapacity for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    arrayvec::ArrayVec::new()
  }
}

/// ```rust
/// let structure: smallvec::SmallVec<[i32; 5]>;
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "smallvec")]
impl<A> WithCapacity for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Self {
    smallvec::SmallVec::with_capacity(input)
  }
}

/// ```rust
/// let structure: staticvec::StaticVec<i32, 5>;
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> WithCapacity for staticvec::StaticVec<T, N> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    staticvec::StaticVec::new()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> WithCapacity for tinyvec::ArrayVec<A>
where
  A: Default + tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Self {
    tinyvec::ArrayVec::new()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// structure = cl_aux::WithCapacity::with_capacity(0);
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> WithCapacity for tinyvec::TinyVec<A>
where
  A: Default + tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Self {
    tinyvec::TinyVec::with_capacity(input)
  }
}
