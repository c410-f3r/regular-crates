use crate::{
  web_socket::{
    WebSocketError, DFLT_FRAME_BUFFER_VEC_LEN, MAX_CONTROL_FRAME_LEN, MAX_HEADER_LEN_U8,
  },
  Wrapper,
};
use alloc::{vec, vec::Vec};
use core::{
  array,
  borrow::{Borrow, BorrowMut},
};

/// Composed by a sequence of mutable bytes
pub type FrameBufferMut<'bytes> = FrameBuffer<&'bytes mut [u8]>;
/// Composed by a vector
pub type FrameBufferVec = FrameBuffer<Vec<u8>>;
pub(crate) type FrameBufferControlArray = FrameBuffer<[u8; MAX_CONTROL_FRAME_LEN]>;
pub(crate) type FrameBufferVecMut<'bytes> = FrameBuffer<Wrapper<&'bytes mut Vec<u8>>>;

/// Concentrates all data necessary to read or write to a stream.
//
// ```
// [ prefix | header | payload | suffix ]
// ```
#[derive(Debug)]
#[repr(C)]
pub struct FrameBuffer<B> {
  header_begin_idx: u8,
  header_end_idx: u8,
  payload_end_idx: usize,
  // Tail field to hopefully help transforms
  buffer: B,
}

impl<B> FrameBuffer<B> {
  pub(crate) fn _buffer(&self) -> &B {
    &self.buffer
  }

  pub(crate) fn buffer_mut(&mut self) -> &mut B {
    &mut self.buffer
  }

  pub(crate) fn clear(&mut self) {
    self.header_begin_idx = 0;
    self.header_end_idx = 0;
    self.payload_end_idx = 0;
  }

  fn header_end_idx_from_parts(header_begin_idx: u8, header_len: u8) -> u8 {
    header_begin_idx.saturating_add(header_len)
  }

  fn payload_end_idx_from_parts(header_end: u8, payload_len: usize) -> usize {
    usize::from(header_end).wrapping_add(payload_len)
  }
}

impl<B> FrameBuffer<B>
where
  B: Borrow<[u8]>,
{
  /// Creates a new instance that has at least 14 bytes.
  #[inline]
  pub fn new(buffer: B) -> Self {
    Self { header_begin_idx: 0, header_end_idx: 0, payload_end_idx: 0, buffer }
  }

  /// Sequence of bytes that composes the frame payload.
  #[inline]
  pub fn payload(&self) -> &[u8] {
    self.buffer.borrow().get(self.header_end_idx.into()..self.payload_end_idx).unwrap_or_default()
  }

  pub(crate) fn frame(&self) -> &[u8] {
    self.buffer.borrow().get(self.header_begin_idx.into()..self.payload_end_idx).unwrap_or_default()
  }

  pub(crate) fn header(&self) -> &[u8] {
    self
      .buffer
      .borrow()
      .get(self.header_begin_idx.into()..self.header_end_idx.into())
      .unwrap_or_default()
  }

  pub(crate) fn set_header_indcs(&mut self, begin_idx: u8, len: u8) -> crate::Result<()> {
    let header_end_idx = Self::header_end_idx_from_parts(begin_idx, len);
    if len > MAX_HEADER_LEN_U8 || usize::from(header_end_idx) > self.buffer.borrow().len() {
      return Err(WebSocketError::InvalidFrameHeaderBounds.into());
    }
    self.header_begin_idx = begin_idx;
    self.header_end_idx = header_end_idx;
    self.payload_end_idx = usize::from(header_end_idx).max(self.payload_end_idx);
    Ok(())
  }

  pub(crate) fn set_payload_len(&mut self, payload_len: usize) -> crate::Result<()> {
    let payload_end_idx = Self::payload_end_idx_from_parts(self.header_end_idx, payload_len);
    if payload_end_idx > self.buffer.borrow().len() {
      return Err(WebSocketError::InvalidPayloadBounds.into());
    }
    self.payload_end_idx = payload_end_idx;
    Ok(())
  }
}

impl<B> FrameBuffer<B>
where
  B: BorrowMut<[u8]>,
{
  pub(crate) fn header_mut(&mut self) -> &mut [u8] {
    self
      .buffer
      .borrow_mut()
      .get_mut(self.header_begin_idx.into()..self.header_end_idx.into())
      .unwrap_or_default()
  }

  pub(crate) fn payload_mut(&mut self) -> &mut [u8] {
    self
      .buffer
      .borrow_mut()
      .get_mut(self.header_end_idx.into()..self.payload_end_idx)
      .unwrap_or_default()
  }
}

impl<B> FrameBuffer<B>
where
  B: BorrowMut<Vec<u8>>,
{
  pub(crate) fn set_params_through_expansion(
    &mut self,
    header_begin_idx: u8,
    header_len: u8,
    mut payload_end_idx: usize,
  ) {
    let header_end_idx = Self::header_end_idx_from_parts(header_begin_idx, header_len);
    payload_end_idx = payload_end_idx.max(header_len.into());
    if payload_end_idx > self.buffer.borrow_mut().len() {
      self.buffer.borrow_mut().resize(payload_end_idx, 0);
    }
    self.header_begin_idx = header_begin_idx;
    self.header_end_idx = header_end_idx;
    self.payload_end_idx = payload_end_idx;
  }
}

impl FrameBufferVec {
  /// Creates a new instance with pre-allocated bytes.
  #[inline]
  pub fn with_capacity(n: usize) -> Self {
    Self { header_begin_idx: 0, header_end_idx: 0, payload_end_idx: 0, buffer: vec![0; n] }
  }
}

impl Default for FrameBufferControlArray {
  #[inline]
  fn default() -> Self {
    Self {
      header_begin_idx: 0,
      header_end_idx: 0,
      payload_end_idx: 0,
      buffer: array::from_fn(|_| 0),
    }
  }
}

impl Default for FrameBufferVec {
  #[inline]
  fn default() -> Self {
    Self {
      header_begin_idx: 0,
      header_end_idx: 0,
      payload_end_idx: 0,
      buffer: vec![0; DFLT_FRAME_BUFFER_VEC_LEN],
    }
  }
}

impl<'fb, B> From<&'fb mut FrameBuffer<B>> for FrameBufferMut<'fb>
where
  B: BorrowMut<[u8]>,
{
  #[inline]
  fn from(from: &'fb mut FrameBuffer<B>) -> Self {
    Self {
      header_begin_idx: from.header_begin_idx,
      header_end_idx: from.header_end_idx,
      payload_end_idx: from.payload_end_idx,
      buffer: from.buffer.borrow_mut(),
    }
  }
}

impl<'bytes, 'fb> From<&'fb mut FrameBufferVec> for FrameBufferVecMut<'bytes>
where
  'fb: 'bytes,
{
  #[inline]
  fn from(from: &'fb mut FrameBufferVec) -> Self {
    Self {
      header_begin_idx: from.header_begin_idx,
      header_end_idx: from.header_end_idx,
      payload_end_idx: from.payload_end_idx,
      buffer: Wrapper(&mut from.buffer),
    }
  }
}
