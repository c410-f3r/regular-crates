use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [GetMut::get_mut] for more information.
pub trait GetMut {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Output
  type Output<'output>
  where
    Self: 'output;

  /// Returns an mutable inner reference of a derived element.
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error>;
}

impl<T> GetMut for &mut T
where
  T: GetMut,
{
  type Error = T::Error;
  type Input = T::Input;
  type Output<'output> = T::Output<'output>
  where
    Self: 'output;

  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    (*self).get_mut(input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
impl<T> GetMut for SingleItemStorage<T> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
impl<T, const N: usize> GetMut for [T; N] {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
impl<T> GetMut for &'_ mut [T] {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "alloc")]
impl<T> GetMut for Vec<T> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut_slice(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> GetMut for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "smallvec")]
impl<A> GetMut for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut A::Item
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> GetMut for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut A::Item
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> GetMut for staticvec::StaticVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut T
  where
    Self: 'output,
    T: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::GetMut::get_mut(&mut structure, 0), Ok(&mut 1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> GetMut for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output mut A::Item
  where
    Self: 'output;

  #[inline]
  fn get_mut(&mut self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get_mut!(self.as_mut(), input)
  }
}
