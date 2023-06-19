#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// See [Retain::retain] for more information.
pub trait Retain {
  /// Input
  type Input;
  /// Output
  type Output;

  /// Retains only the elements specified by the `F` predicate.
  fn retain(&mut self, input: Self::Input) -> Self::Output;
}

impl<T> Retain for &mut T
where
  T: Retain,
{
  type Input = T::Input;
  type Output = T::Output;

  fn retain(&mut self, input: Self::Input) -> Self::Output {
    (*self).retain(input)
  }
}

/// ```rust
/// let mut opt = Some(1);
/// cl_aux::Retain::retain(&mut opt, |n| n % 2 == 0);
/// assert_eq!(opt, None);
/// ```
impl<T> Retain for Option<T> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    if let Some(elem) = self.as_mut() {
      if !input(elem) {
        *self = None;
      }
    }
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure, &[2]);
/// ```
#[cfg(feature = "alloc")]
impl<T> Retain for Vec<T> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::array_vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "arrayvec")]
impl<T, const N: usize> Retain for arrayvec::ArrayVec<T, N> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(|i| input(i));
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::small_vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "smallvec")]
impl<A> Retain for smallvec::SmallVec<A>
where
  A: smallvec::Array,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(|i| input(i));
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::static_vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "staticvec")]
impl<T, const N: usize> Retain for staticvec::StaticVec<T, N> {
  type Input = fn(&T) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec_array_vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Retain for tinyvec::ArrayVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input);
  }
}

/// ```rust
/// let mut structure = cl_aux::doc_tests::tiny_vec();
/// cl_aux::Retain::retain(&mut structure, |n| n % 2 == 0);
/// assert_eq!(&structure[..], &[2]);
/// ```
#[cfg(feature = "tinyvec")]
impl<A> Retain for tinyvec::TinyVec<A>
where
  A: tinyvec::Array,
  A::Item: Default,
{
  type Input = fn(&A::Item) -> bool;
  type Output = ();

  #[inline]
  fn retain(&mut self, input: Self::Input) {
    self.retain(input);
  }
}
