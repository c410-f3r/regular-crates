#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [`WithCapacity::with_capacity`] for more information.
pub trait WithCapacity
where
  Self: Sized,
{
  /// Error
  type Error;
  /// Input
  type Input;

  /// Creates a new instance based on an initial holding capacity provided by `Input`.
  fn with_capacity(input: Self::Input) -> Result<Self, Self::Error>;
}

/// ```rust
/// use cl_aux::Capacity;
/// let structure: [i32; 5];
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
/// assert_eq!(structure.capacity(), 5);
/// ```
impl<T, const N: usize> WithCapacity for [T; N]
where
  T: Default,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Result<Self, Self::Error> {
    Ok([(); N].map(|_| T::default()))
  }
}

/// ```rust
/// let structure: String = cl_aux::WithCapacity::with_capacity(2).unwrap();
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl WithCapacity for String {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Result<Self, Self::Error> {
    Ok(String::with_capacity(input))
  }
}

/// ```rust
/// let structure: Vec<i32> = cl_aux::WithCapacity::with_capacity(2).unwrap();
/// assert_eq!(structure.capacity(), 2);
/// ```
#[cfg(feature = "alloc")]
impl<T> WithCapacity for Vec<T> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Result<Self, Self::Error> {
    Ok(Vec::with_capacity(input))
  }
}

/// ```rust
/// let structure: arrayvec::ArrayString<5>;
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> WithCapacity for arrayvec::ArrayString<N> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Result<Self, Self::Error> {
    Ok(arrayvec::ArrayString::new())
  }
}

/// ```rust
/// let structure: arrayvec::ArrayVec<i32, 5>;
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> WithCapacity for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(_: Self::Input) -> Result<Self, Self::Error> {
    Ok(arrayvec::ArrayVec::new())
  }
}

/// ```rust
/// let structure: smallvec::SmallVec<[i32; 5]>;
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
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
  fn with_capacity(input: Self::Input) -> Result<Self, Self::Error> {
    Ok(smallvec::SmallVec::with_capacity(input))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
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
  fn with_capacity(_: Self::Input) -> Result<Self, Self::Error> {
    Ok(tinyvec::ArrayVec::new())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// structure = cl_aux::WithCapacity::with_capacity(0).unwrap();
/// assert_eq!(structure.capacity(), 5);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> WithCapacity for tinyvec::TinyVec<A>
where
  A: Default + tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;

  #[inline]
  fn with_capacity(input: Self::Input) -> Result<Self, Self::Error> {
    Ok(tinyvec::TinyVec::with_capacity(input))
  }
}
