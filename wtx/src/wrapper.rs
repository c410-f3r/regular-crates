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

impl Borrow<Vec<u8>> for Wrapper<&mut Vec<u8>> {
  #[inline]
  fn borrow(&self) -> &Vec<u8> {
    self.0
  }
}

impl BorrowMut<[u8]> for Wrapper<&mut Vec<u8>> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [u8] {
    self.0.as_mut_slice()
  }
}

impl BorrowMut<Vec<u8>> for Wrapper<&mut Vec<u8>> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Vec<u8> {
    self.0
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

#[cfg(test)]
mod tests {
  use crate::{
    web_socket::{FrameBufferVec, FrameBufferVecMut, WebSocketClientOwned},
    DummyStream,
  };

  #[test]
  fn reads_accept_frame_buffers_composed_by_different_types() {
    let mut ws = WebSocketClientOwned::new(<_>::default(), DummyStream);

    let _ = ws.read_frame(&mut FrameBufferVecMut::from(&mut Vec::new()));
    let _ = ws.read_msg(&mut FrameBufferVecMut::from(&mut Vec::new()));

    let _ = ws.read_frame(&mut FrameBufferVec::new(Vec::new()));
    let _ = ws.read_msg(&mut FrameBufferVec::new(Vec::new()));
  }
}
