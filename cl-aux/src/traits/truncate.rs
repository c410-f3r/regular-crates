#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [Truncate::truncate] for more information.
pub trait Truncate {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Truncates the storage, delimiting its length by `Input`.
  fn truncate(&mut self, input: Self::Input) -> Self::Output;
}

impl<T> Truncate for &mut T
where
  T: Truncate,
{
  type Input = T::Input;
  type Output = T::Output;

  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    (*self).truncate(input)
  }
}

/// ```rust
/// let mut structure = Some(1);
/// cl_aux::Truncate::truncate(&mut structure, 0);
/// assert_eq!(structure, None);
/// ```
impl<T> Truncate for Option<T> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    if input == 0 {
      *self = None;
    }
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "alloc")]
impl Truncate for String {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "alloc")]
impl<T> Truncate for Vec<T> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Truncate for arrayvec::ArrayString<N> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Truncate for arrayvec::ArrayVec<T, N> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Truncate for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Truncate for staticvec::StaticVec<T, N> {
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Truncate for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Truncate for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;
  type Output = ();

  #[inline]
  fn truncate(&mut self, input: Self::Input) -> Self::Output {
    self.truncate(input);
  }
}
