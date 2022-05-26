#![allow(unused_lifetimes)]

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [Push::push] for more information.
pub trait Push<I> {
  /// Error
  type Error;
  /// Output
  type Output;

  /// Pushes an element, increasing the storage length.
  fn push(&mut self, input: I) -> Result<Self::Output, Self::Error>;
}

/// ```rust
/// let mut opt = None;
/// cl_aux::Push::push(&mut opt, 3);
/// assert_eq!(opt, Some(3));
/// ```
impl<T> Push<T> for Option<T> {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: T) -> Result<Self::Output, Self::Error> {
    if self.is_some() {
      Err(crate::Error::InsufficientCapacity(stringify!(self), 1))
    } else {
      *self = Some(input);
      Ok(())
    }
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// cl_aux::Push::push(&mut structure, '!');
/// assert_eq!(structure.as_str(), "Hello!");
/// ```
#[cfg(feature = "alloc")]
impl Push<char> for String {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: char) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// cl_aux::Push::push(&mut structure, "!!");
/// assert_eq!(structure.as_str(), "Hello!!");
/// ```
#[cfg(feature = "alloc")]
impl<'input> Push<&'input str> for String {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: &'input str) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push_str(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "alloc")]
impl<T> Push<T> for Vec<T> {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: T) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// cl_aux::Push::push(&mut structure, '!');
/// assert_eq!(structure.as_str(), "Hello!");
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Push<char> for arrayvec::ArrayString<N> {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: char) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// cl_aux::Push::push(&mut structure, "!!");
/// assert_eq!(structure.as_str(), "Hello!!");
/// ```
#[cfg(feature = "arrayvec")]
impl<'input, const N: usize> Push<&'input str> for arrayvec::ArrayString<N> {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: &'input str) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push_str(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Push<T> for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: T) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "smallvec")]
impl<A> Push<A::Item> for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: A::Item) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Push<T> for staticvec::StaticVec<T, N> {
  type Error = T;
  type Output = ();

  #[inline]
  fn push(&mut self, input: T) -> Result<Self::Output, Self::Error> {
    self.try_push(input).map_err(|e| e.into_value())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Push<A::Item> for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: A::Item) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// cl_aux::Push::push(&mut structure, 20);
/// assert_eq!(structure.get(3), Some(&20));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Push<A::Item> for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Output = ();

  #[inline]
  fn push(&mut self, input: A::Item) -> Result<Self::Output, Self::Error> {
    _check_capacity!(self);
    self.push(input);
    Ok(())
  }
}
