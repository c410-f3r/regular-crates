use core::{borrow::Borrow, ops::Deref, slice::Iter};

/// Immutable array reference wrapper similar to [crate::ArrayWrapper].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArrayWrapperRef<'array, T, const N: usize>(
  /// The actual array reference.
  pub &'array [T; N],
);

impl<T, const N: usize> AsRef<[T; N]> for ArrayWrapperRef<'_, T, N> {
  #[inline]
  fn as_ref(&self) -> &[T; N] {
    self
  }
}

impl<T, const N: usize> Borrow<[T; N]> for ArrayWrapperRef<'_, T, N> {
  #[inline]
  fn borrow(&self) -> &[T; N] {
    self
  }
}

impl<T, const N: usize> Deref for ArrayWrapperRef<'_, T, N> {
  type Target = [T; N];

  #[inline]
  fn deref(&self) -> &[T; N] {
    self.0
  }
}

impl<'array, T, const N: usize> From<&'array [T; N]> for ArrayWrapperRef<'array, T, N> {
  #[inline]
  fn from(from: &'array [T; N]) -> Self {
    Self(from)
  }
}

impl<'array, T, const N: usize> IntoIterator for &'array ArrayWrapperRef<'array, T, N> {
  type IntoIter = Iter<'array, T>;
  type Item = &'array T;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

#[cfg(feature = "serde")]
mod serde {
  use crate::ArrayWrapperRef;
  use serde::{ser::SerializeTuple, Serialize, Serializer};

  impl<T, const N: usize> Serialize for ArrayWrapperRef<'_, T, N>
  where
    T: Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut seq = serializer.serialize_tuple(N)?;
      for elem in self.0 {
        seq.serialize_element(elem)?;
      }
      seq.end()
    }
  }
}
