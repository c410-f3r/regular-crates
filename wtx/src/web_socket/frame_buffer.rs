use crate::{
  misc::SingleTypeStorage,
  web_socket::{WebSocketError, DFLT_FRAME_BUFFER_VEC_LEN, MAX_CONTROL_FRAME_LEN, MAX_HDR_LEN_U8},
};
use alloc::{vec, vec::Vec};
use core::array;

/// Composed by an array with the maximum allowed size of a frame control.
pub type FrameBufferControlArray = FrameBuffer<[u8; MAX_CONTROL_FRAME_LEN]>;
/// Composed by an mutable array reference with the maximum allowed size of a frame control.
pub type FrameBufferControlArrayMut<'bytes> = FrameBuffer<&'bytes mut [u8; MAX_CONTROL_FRAME_LEN]>;
/// Composed by a sequence of mutable bytes.
pub type FrameBufferMut<'bytes> = FrameBuffer<&'bytes mut [u8]>;
/// Composed by an owned vector.
pub type FrameBufferVec = FrameBuffer<Vec<u8>>;
/// Composed by a mutable vector reference.
pub type FrameBufferVecMut<'bytes> = FrameBuffer<&'bytes mut Vec<u8>>;

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
  /// The underlying byte collection.
  #[inline]
  pub fn buffer(&self) -> &B {
    &self.buffer
  }

  /// The indices that represent all frame parts contained in the underlying byte collection.
  ///
  /// ```rust
  /// let fb = wtx::web_socket::FrameBufferVec::default();
  /// let (header_begin_idx, header_end_idx, payload_end_idx) = fb.indcs();
  /// assert_eq!(
  ///   fb.buffer().get(header_begin_idx.into()..header_end_idx.into()).unwrap_or_default(),
  ///   fb.header()
  /// );
  /// assert_eq!(
  ///   fb.buffer().get(header_end_idx.into()..payload_end_idx).unwrap_or_default(),
  ///   fb.payload()
  /// );
  /// ```
  #[inline]
  pub fn indcs(&self) -> (u8, u8, usize) {
    (self.header_begin_idx, self.header_end_idx, self.payload_end_idx)
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
  B: AsRef<[u8]>,
{
  /// Creates a new instance from the given `buffer`.
  #[inline]
  pub fn new(buffer: B) -> Self {
    Self { header_begin_idx: 0, header_end_idx: 0, payload_end_idx: 0, buffer }
  }

  /// Sequence of bytes that composes the frame header.
  #[inline]
  pub fn header(&self) -> &[u8] {
    self
      .buffer
      .as_ref()
      .get(self.header_begin_idx.into()..self.header_end_idx.into())
      .unwrap_or_default()
  }

  /// Sequence of bytes that composes the frame payload.
  #[inline]
  pub fn payload(&self) -> &[u8] {
    self.buffer.as_ref().get(self.header_end_idx.into()..self.payload_end_idx).unwrap_or_default()
  }

  pub(crate) fn frame(&self) -> &[u8] {
    self.buffer.as_ref().get(self.header_begin_idx.into()..self.payload_end_idx).unwrap_or_default()
  }

  pub(crate) fn set_header_indcs(&mut self, begin_idx: u8, len: u8) -> crate::Result<()> {
    let header_end_idx = Self::header_end_idx_from_parts(begin_idx, len);
    if len > MAX_HDR_LEN_U8 || usize::from(header_end_idx) > self.buffer.as_ref().len() {
      return Err(WebSocketError::InvalidFrameHeaderBounds.into());
    }
    self.header_begin_idx = begin_idx;
    self.header_end_idx = header_end_idx;
    self.payload_end_idx = usize::from(header_end_idx).max(self.payload_end_idx);
    Ok(())
  }

  pub(crate) fn set_payload_len(&mut self, payload_len: usize) -> crate::Result<()> {
    let payload_end_idx = Self::payload_end_idx_from_parts(self.header_end_idx, payload_len);
    if payload_end_idx > self.buffer.as_ref().len() {
      return Err(WebSocketError::InvalidPayloadBounds.into());
    }
    self.payload_end_idx = payload_end_idx;
    Ok(())
  }
}

impl<B> FrameBuffer<B>
where
  B: AsMut<[u8]>,
{
  pub(crate) fn header_mut(&mut self) -> &mut [u8] {
    self
      .buffer
      .as_mut()
      .get_mut(self.header_begin_idx.into()..self.header_end_idx.into())
      .unwrap_or_default()
  }

  pub(crate) fn payload_mut(&mut self) -> &mut [u8] {
    self
      .buffer
      .as_mut()
      .get_mut(self.header_end_idx.into()..self.payload_end_idx)
      .unwrap_or_default()
  }
}

impl<B> FrameBuffer<B>
where
  B: AsMut<Vec<u8>>,
{
  pub(crate) fn set_params_through_expansion(
    &mut self,
    header_begin_idx: u8,
    header_len: u8,
    mut payload_end_idx: usize,
  ) {
    let header_end_idx = Self::header_end_idx_from_parts(header_begin_idx, header_len);
    payload_end_idx = payload_end_idx.max(header_len.into());
    if payload_end_idx > self.buffer.as_mut().len() {
      self.buffer.as_mut().resize(payload_end_idx, 0);
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

impl<B> SingleTypeStorage for FrameBuffer<B> {
  type Item = B;
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
  B: AsMut<[u8]>,
{
  #[inline]
  fn from(from: &'fb mut FrameBuffer<B>) -> Self {
    Self {
      header_begin_idx: from.header_begin_idx,
      header_end_idx: from.header_end_idx,
      payload_end_idx: from.payload_end_idx,
      buffer: from.buffer.as_mut(),
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
      buffer: &mut from.buffer,
    }
  }
}

impl From<Vec<u8>> for FrameBufferVec {
  #[inline]
  fn from(from: Vec<u8>) -> Self {
    Self::new(from)
  }
}

impl<'bytes> From<&'bytes mut Vec<u8>> for FrameBufferVecMut<'bytes> {
  #[inline]
  fn from(from: &'bytes mut Vec<u8>) -> Self {
    Self::new(from)
  }
}
