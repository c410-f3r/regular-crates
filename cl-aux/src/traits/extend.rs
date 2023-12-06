#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [Extend::extend] for more information.
pub trait Extend<IN> {
  /// Error
  type Error;

  /// Returns an mutable inner reference of a derived element.
  fn extend(&mut self, into_iter: impl IntoIterator<Item = IN>) -> Result<(), Self::Error>;
}

impl<IN, T> Extend<IN> for &mut T
where
  T: Extend<IN>,
{
  type Error = T::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = IN>) -> Result<(), Self::Error> {
    (*self).extend(into_iter)
  }
}

impl<T> Extend<T> for () {
  type Error = crate::Error;

  #[inline]
  fn extend(&mut self, _: impl IntoIterator<Item = T>) -> Result<(), Self::Error> {
    Ok(())
  }
}

/// ```rust
/// let mut opt = None;
/// cl_aux::Extend::extend(&mut opt, [3]).unwrap();
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Extend<T> for Option<T> {
  type Error = crate::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = T>) -> Result<(), Self::Error> {
    _check_capacity!(self);
    let err = || crate::Error::InsufficientCapacity(1);
    let mut iter = into_iter.into_iter();
    let next = iter.next().ok_or_else(err)?;
    *self = Some(next);
    if iter.next().is_some() {
      Err(err())
    } else {
      Ok(())
    }
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// cl_aux::Extend::extend(&mut structure, ['!']);
/// assert_eq!(structure.as_str(), "Hello!");
/// ```
#[cfg(feature = "alloc")]
impl Extend<char> for String {
  type Error = crate::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = char>) -> Result<(), Self::Error> {
    core::iter::Extend::extend(self, into_iter);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Extend::extend(&mut structure, [20]);
/// assert_eq!(&structure[..], &[1, 2, 3, 20][..]);
/// ```
#[cfg(feature = "alloc")]
impl<T> Extend<T> for Vec<T> {
  type Error = crate::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = T>) -> Result<(), Self::Error> {
    core::iter::Extend::extend(self, into_iter);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// cl_aux::Extend::extend(&mut structure, "!".chars());
/// assert_eq!(structure.as_str(), "Hello!");
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Extend<char> for arrayvec::ArrayString<N>
where
  Self: crate::Push<char>,
{
  type Error = <Self as crate::Push<char>>::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = char>) -> Result<(), Self::Error> {
    for elem in into_iter {
      crate::Push::push(self, elem)?;
    }
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Extend::extend(&mut structure, [20]);
/// assert_eq!(&structure[..], &[1, 2, 3, 20][..]);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Extend<T> for arrayvec::ArrayVec<T, N>
where
  Self: crate::Push<T>,
{
  type Error = <Self as crate::Push<T>>::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = T>) -> Result<(), Self::Error> {
    for elem in into_iter {
      crate::Push::push(self, elem)?;
    }
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Extend::extend(&mut structure, [20]);
/// assert_eq!(&structure[..], &[1, 2, 3, 20][..]);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Extend<A::Item> for smallvec::SmallVec<A>
where
  A: smallvec::Array,
  Self: crate::Push<A::Item>,
{
  type Error = <Self as crate::Push<A::Item>>::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = A::Item>) -> Result<(), Self::Error> {
    for elem in into_iter {
      crate::Push::push(self, elem)?;
    }
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Extend::extend(&mut structure, [20]);
/// assert_eq!(&structure[..], &[1, 2, 3, 20][..]);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Extend<A::Item> for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  Self: crate::Push<A::Item>,
{
  type Error = <Self as crate::Push<A::Item>>::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = A::Item>) -> Result<(), Self::Error> {
    for elem in into_iter {
      crate::Push::push(self, elem)?;
    }
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// cl_aux::Extend::extend(&mut structure, [20]);
/// assert_eq!(&structure[..], &[1, 2, 3, 20][..]);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> Extend<A::Item> for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
  Self: crate::Push<A::Item>,
{
  type Error = <Self as crate::Push<A::Item>>::Error;

  #[inline]
  fn extend(&mut self, into_iter: impl IntoIterator<Item = A::Item>) -> Result<(), Self::Error> {
    for elem in into_iter {
      crate::Push::push(self, elem)?;
    }
    Ok(())
  }
}
