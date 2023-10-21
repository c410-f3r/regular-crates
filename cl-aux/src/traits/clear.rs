#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// See [Clear::clear] for more information.
pub trait Clear {
  /// "Clears" the internal buffer, "erasing" all elements.
  fn clear(&mut self);
}

impl<T> Clear for &mut T
where
  T: Clear,
{
  #[inline]
  fn clear(&mut self) {
    (*self).clear();
  }
}

impl Clear for () {
  #[inline]
  fn clear(&mut self) {}
}

/// ```rust
/// let mut opt = Some(0);
/// cl_aux::Clear::clear(&mut opt);
/// assert_eq!(opt, None);
/// ```
impl<T> Clear for Option<T> {
  #[inline]
  fn clear(&mut self) {
    *self = None;
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::string();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "alloc")]
impl Clear for String {
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "alloc")]
impl<T> Clear for Vec<T> {
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_string();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "arrayvec")]
impl<const N: usize> Clear for arrayvec::ArrayString<N> {
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Clear for arrayvec::ArrayVec<T, N> {
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Clear for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Clear for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_tiny_vec();
/// cl_aux::Clear::clear(&mut structure);
/// assert_eq!(structure.len(), 0);
/// ```
#[cfg(all(feature = "alloc", feature = "tinyvec"))]
impl<A> Clear for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  #[inline]
  fn clear(&mut self) {
    self.clear();
  }
}
