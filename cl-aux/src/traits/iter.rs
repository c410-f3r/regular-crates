use crate::SingleItemStorage;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::{option, slice};

/// See [Iter::iter] for more information.
pub trait Iter {
  /// Iterator
  type Output<'iter>: Iterator
  where
    Self: 'iter;

  /// Returns a new iterator that refers inner elements.
  fn iter(&self) -> Self::Output<'_>;
}

impl<T> Iter for &T
where
  T: Iter,
{
  type Output<'iter> = T::Output<'iter>
  where
    Self: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    (*self).iter()
  }
}

/// ```rust
/// assert_eq!(cl_aux::Iter::iter(&()).next(), None);
/// ```
impl Iter for () {
  type Output<'iter> = slice::Iter<'iter, ()>;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    [].as_ref().iter()
  }
}

/// ```rust
/// assert_eq!(cl_aux::Iter::iter(&Some(0)).next().unwrap(), &0);
/// ```
impl<T> Iter for Option<T> {
  type Output<'iter> = option::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.iter()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::single_item_storage();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
impl<T> Iter for SingleItemStorage<T> {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    slice::from_ref(&self.0).iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
impl<T, const N: usize> Iter for [T; N] {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_ref().iter()
  }
}

/// ```rust
/// let structure = cl_aux::doc_tests::slice();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
impl<T> Iter for &'_ [T] {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    Self: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_ref().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), 'H');
/// ```
#[cfg(feature = "alloc")]
impl Iter for String {
  type Output<'iter> = core::str::Chars<'iter>;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.chars()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "alloc")]
impl<T> Iter for Vec<T> {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), 'H');
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Iter for arrayvec::ArrayString<N> {
  type Output<'iter> = core::str::Chars<'iter>;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.chars()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Iter for arrayvec::ArrayVec<T, N> {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Iter for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Output<'iter> = slice::Iter<'iter, A::Item>
  where
    A: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Iter for staticvec::StaticVec<T, N> {
  type Output<'iter> = slice::Iter<'iter, T>
  where
    T: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Iter for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output<'iter> = slice::Iter<'iter, A::Item>
  where
    A: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// assert_eq!(cl_aux::Iter::iter(&structure).next().unwrap(), &1);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Iter for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Output<'iter> = slice::Iter<'iter, A::Item>
  where
    A: 'iter;

  #[inline]
  fn iter(&self) -> Self::Output<'_> {
    self.as_slice().iter()
  }
}
