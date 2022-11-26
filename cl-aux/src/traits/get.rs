use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [Get::get] for more information.
pub trait Get {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Output
  type Output<'output>
  where
    Self: 'output;

  /// Returns an immutable inner reference of a derived element.
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error>;
}

/// ```rust
/// let structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
impl<T> Get for SingleItemStorage<T> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, _: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), 0)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
impl<T, const N: usize> Get for [T; N] {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
impl<T> Get for &'_ [T] {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::slice_mut!();
/// assert_eq!(cl_aux::Get::get(&mut structure, 0), Ok(&1));
/// ```
impl<T> Get for &'_ mut [T] {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "alloc")]
impl<T> Get for Vec<T> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_slice(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Get for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "smallvec")]
impl<A> Get for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output A::Item
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::static_vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Get for staticvec::StaticVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output T
  where
    Self: 'output,
    T: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Get for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output A::Item
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::Get::get(&structure, 0), Ok(&1));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Get for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;
  type Output<'output> = &'output A::Item
  where
    Self: 'output;

  #[inline]
  fn get(&self, input: Self::Input) -> Result<Self::Output<'_>, Self::Error> {
    _get!(self.as_ref(), input)
  }
}
