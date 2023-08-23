use crate::misc::Expand;
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};

/// Internal utility used to workaround coherence. Doesn't have any public meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct Wrapper<T>(
  /// Anything
  pub T,
);

impl Borrow<[u8]> for Wrapper<&mut Vec<u8>> {
  #[inline]
  fn borrow(&self) -> &[u8] {
    self.0.as_slice()
  }
}

impl BorrowMut<[u8]> for Wrapper<&mut Vec<u8>> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [u8] {
    self.0.as_mut_slice()
  }
}

impl<T> Expand for Wrapper<T>
where
  T: Expand,
{
  #[inline]
  fn expand(&mut self, len: usize) {
    self.0.expand(len);
  }
}
