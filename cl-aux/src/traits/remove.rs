#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [Remove::remove] for more information.
pub trait Remove {
  /// Error
  type Error;
  /// Input
  type Input;
  /// Output
  type Output;

  /// Removes an element referenced by `Input`.
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error>;
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "alloc")]
impl<T> Remove for Vec<T> {
  type Error = crate::Error;
  type Input = usize;
  type Output = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Remove for arrayvec::ArrayVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "smallvec")]
impl<A> Remove for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Error = crate::Error;
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Remove for staticvec::StaticVec<T, N> {
  type Error = crate::Error;
  type Input = usize;
  type Output = T;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Remove for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// cl_aux::Remove::remove(&mut structure, 0);
/// assert_eq!(structure.get(0), Some(&2));
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Remove for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Error = crate::Error;
  type Input = usize;
  type Output = A::Item;

  #[inline]
  fn remove(&mut self, idx: Self::Input) -> Result<Self::Output, Self::Error> {
    _check_indcs!(self, idx);
    Ok(self.remove(idx))
  }
}
