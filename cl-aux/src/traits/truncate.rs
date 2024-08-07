#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [`Truncate::truncate`] for more information.
pub trait Truncate {
  /// Input
  type Input;

  /// Truncates the storage, delimiting its length by `Input`.
  fn truncate(&mut self, input: Self::Input);
}

impl<T> Truncate for &mut T
where
  T: Truncate,
{
  type Input = T::Input;

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    (*self).truncate(input);
  }
}

/// ```rust
/// let mut structure = Some(1);
/// cl_aux::Truncate::truncate(&mut structure, 0);
/// assert_eq!(structure, None);
/// ```
impl<T> Truncate for Option<T> {
  type Input = usize;

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
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

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// cl_aux::Truncate::truncate(&mut structure, 1);
/// assert_eq!(structure.len(), 1);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> Truncate for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = usize;

  #[inline]
  fn truncate(&mut self, input: Self::Input) {
    self.truncate(input);
  }
}
