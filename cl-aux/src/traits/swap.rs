#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [Swap::swap] for more information.
pub trait Swap {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Output
  type Output;

  /// Swaps two elements referencied by `Input`.
  fn swap(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

impl<T> Swap for &mut T
where
  T: Swap,
{
  type Error = T::Error;
  type Input = T::Input;
  type Output = T::Output;

  fn swap(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
    (*self).swap(input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<T, const N: usize> Swap for [T; N] {
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
impl<T> Swap for &'_ mut [T] {
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "alloc")]
impl<T> Swap for Vec<T> {
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Swap for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "smallvec")]
impl<A> Swap for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Swap for staticvec::StaticVec<T, N> {
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Swap for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// cl_aux::Swap::swap(&mut structure, [0, 2]);
/// assert_eq!(structure.get(0), Some(&3));
/// assert_eq!(structure.get(2), Some(&1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Swap for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = [usize; 2];
  type Output = ();

  #[inline]
  fn swap(&mut self, [a, b]: Self::Input) -> Result<Self::Output, Self::Error> {
    manage_slice(self, a, b)
  }
}

#[inline]
fn manage_slice<T>(slice: &mut [T], a: usize, b: usize) -> crate::Result<()> {
  _check_indcs!(&slice, a, b);
  slice.as_mut().swap(a, b);
  Ok(())
}
