//! A computer communications protocol, providing full-duplex communication channels over a single
//! TCP connection.
//
// # Mask bytes
//
// Client: Creates masked frames, receives unmasked frames and writes masked frames
// Server: Creates unmasked frames, receives masked frames and writes unmasked frames

mod close_code;
mod frame;
mod frame_buffer;
mod http;
mod mask;
mod op_code;
mod read_buffer;

pub use crate::stream::{DummyStream, Stream};
use crate::{
  misc::{from_utf8_ext_rslt, from_utf8_opt, CompleteErr, ExtUtf8Error, Rng},
  web_socket::{
    close_code::CloseCode,
    frame::{FrameControlArray, FrameMut},
  },
};
use core::borrow::BorrowMut;
pub use frame::Frame;
pub use frame_buffer::{FrameBuffer, FrameBufferMut, FrameBufferVec};
#[cfg(feature = "http-client")]
pub use http::http_client::{
  UpgradeFutHttpClient, WebSocketHandshakeHttpClient, WebSocketUpgradeHttpClient,
};
#[cfg(feature = "hyper")]
pub use http::hyper::{UpgradeFutHyper, WebSocketHandshakeHyper, WebSocketUpgradeHyper};
pub use http::{WebSocketHandshake, WebSocketUpgrade};
pub use mask::unmask;
pub use op_code::OpCode;
use read_buffer::ReadBuffer;

pub(crate) const DFLT_FRAME_BUFFER_VEC_LEN: usize = 32 * 1024 * 1024;
pub(crate) const DFLT_READ_BUFFER_LEN: usize = 128 * 1024 * 1024;
pub(crate) const MAX_CONTROL_FRAME_LEN: usize =
  MAX_HEADER_LEN_USIZE + MAX_CONTROL_FRAME_PAYLOAD_LEN;
pub(crate) const MAX_CONTROL_FRAME_PAYLOAD_LEN: usize = 125;
pub(crate) const MAX_HEADER_LEN_U8: u8 = 14;
pub(crate) const MAX_HEADER_LEN_USIZE: usize = 14;
pub(crate) const MIN_HEADER_LEN_USIZE: usize = 2;

/// Always masks the payload before sending.
pub type WebSocketClient<S> = WebSocket<S, true>;
/// Always decode the payload after receiving.
pub type WebSocketServer<S> = WebSocket<S, false>;

/// WebSocket protocol implementation over an asynchronous stream.
#[derive(Debug)]
pub struct WebSocket<S, const IS_CLIENT: bool> {
  auto_close: bool,
  auto_pong: bool,
  is_stream_closed: bool,
  max_payload_size: usize,
  rb: ReadBuffer,
  rng: Rng,
  stream: S,
}

impl<S, const IS_CLIENT: bool> WebSocket<S, IS_CLIENT> {
  /// Sets whether to automatically close the connection when a close frame is received. Defaults
  /// to `true`.
  #[inline]
  pub fn set_auto_close(&mut self, auto_close: bool) {
    self.auto_close = auto_close;
  }

  /// Sets whether to automatically send a pong frame when a ping frame is received. Defaults
  /// to `true`.
  #[inline]
  pub fn set_auto_pong(&mut self, auto_pong: bool) {
    self.auto_pong = auto_pong;
  }

  /// Sets whether to automatically close the connection when a received frame payload length
  /// exceeds `max_payload_size`. Defaults to `64 * 1024 * 1024` bytes (64 MiB).
  #[inline]
  pub fn set_max_payload_size(&mut self, max_payload_size: usize) {
    self.max_payload_size = max_payload_size;
  }
}

impl<S, const IS_CLIENT: bool> WebSocket<S, IS_CLIENT>
where
  S: Stream,
{
  /// Creates a new instance from a stream that supposedly has already completed the WebSocket
  /// handshake.
  #[inline]
  pub fn new(stream: S) -> Self {
    Self {
      auto_close: true,
      auto_pong: true,
      is_stream_closed: false,
      max_payload_size: 32 * 1024 * 1024,
      rb: ReadBuffer::with_capacity(DFLT_READ_BUFFER_LEN),
      rng: Rng::default(),
      stream,
    }
  }

  /// Reads a frame from the stream unmasking and validating its payload.
  #[inline]
  pub async fn read_frame<'fb>(
    &mut self,
    fb: &'fb mut FrameBufferVec,
  ) -> crate::Result<FrameMut<'fb, IS_CLIENT>> {
    let rbfi = self.do_read_frame::<true>().await?;
    Self::copy_from_rb_to_fb(CopyType::Normal, fb, &self.rb, &rbfi);
    if !self.rb.has_following_frames() {
      self.rb.clear();
    }
    Frame::from_fb(fb.into())
  }

  /// Collects frames and returns the completed message once all fragments have been received.
  #[inline]
  pub async fn read_msg<'fb>(
    &mut self,
    fb: &'fb mut FrameBufferVec,
  ) -> crate::Result<FrameMut<'fb, IS_CLIENT>> {
    let mut iuc_opt = None;
    let mut is_binary = true;
    let rbfi = self.do_read_frame::<false>().await?;
    if rbfi.op_code.is_continuation() {
      return Err(crate::Error::InvalidContinuationFrame);
    }
    let should_stop_at_the_first_frame = match rbfi.op_code {
      OpCode::Binary => rbfi.fin,
      OpCode::Text => {
        let curr_payload = self.rb.current_frame().get(rbfi.header_end_idx..).unwrap_or_default();
        if rbfi.fin {
          if from_utf8_opt(curr_payload).is_none() {
            return Err(crate::Error::InvalidUTF8);
          }
          true
        } else {
          is_binary = false;
          match from_utf8_ext_rslt(curr_payload) {
            Err(ExtUtf8Error::Incomplete { incomplete_ending_char, .. }) => {
              iuc_opt = Some(incomplete_ending_char);
              false
            }
            Err(ExtUtf8Error::Invalid { .. }) => {
              return Err(crate::Error::InvalidUTF8);
            }
            Ok(_) => false,
          }
        }
      }
      OpCode::Continuation | OpCode::Close | OpCode::Ping | OpCode::Pong => true,
    };
    if should_stop_at_the_first_frame {
      Self::copy_from_rb_to_fb(CopyType::Normal, fb, &self.rb, &rbfi);
      if !self.rb.has_following_frames() {
        self.rb.clear();
      }
      return Frame::from_fb(fb.into());
    }
    let mut total_frame_len = msg_header_placeholder::<IS_CLIENT>().into();
    Self::copy_from_rb_to_fb(CopyType::Msg(&mut total_frame_len), fb, &self.rb, &rbfi);
    if is_binary {
      self.manage_read_msg_loop(fb, rbfi.op_code, &mut total_frame_len, |_| Ok(())).await?;
    } else {
      self
        .manage_read_msg_loop(fb, rbfi.op_code, &mut total_frame_len, |payload| {
          let tail = if let Some(mut incomplete) = iuc_opt.take() {
            let (rslt, remaining) = incomplete.complete(payload);
            match rslt {
              Err(CompleteErr::HasInvalidBytes) => {
                return Err(crate::Error::InvalidUTF8);
              }
              Err(CompleteErr::InsufficientInput) => {
                let _ = iuc_opt.replace(incomplete);
                &[]
              }
              Ok(_) => remaining,
            }
          } else {
            payload
          };
          match from_utf8_ext_rslt(tail) {
            Err(ExtUtf8Error::Incomplete { incomplete_ending_char, .. }) => {
              iuc_opt = Some(incomplete_ending_char);
            }
            Err(ExtUtf8Error::Invalid { .. }) => {
              return Err(crate::Error::InvalidUTF8);
            }
            Ok(_) => {}
          }
          Ok(())
        })
        .await?;
    };
    Frame::from_fb(fb.into())
  }

  /// Writes a frame to the stream without masking its payload.
  #[inline]
  pub async fn write_frame<B>(
    &mut self,
    frame: Frame<FrameBuffer<B>, IS_CLIENT>,
  ) -> crate::Result<()>
  where
    B: BorrowMut<[u8]> + core::fmt::Debug,
  {
    Self::do_write_frame(frame, &mut self.is_stream_closed, &mut self.rng, &mut self.stream).await
  }

  fn copy_from_rb_to_fb(
    ct: CopyType<'_>,
    fb: &mut FrameBufferVec,
    rb: &ReadBuffer,
    rbfi: &ReadBufferFrameInfo,
  ) {
    let current_frame = rb.current_frame();
    let range = match ct {
      CopyType::Msg(total_frame_len) => {
        let prev = *total_frame_len;
        *total_frame_len = total_frame_len.wrapping_add(rbfi.payload_len);
        fb.set_params_through_expansion(0, msg_header_placeholder::<IS_CLIENT>(), *total_frame_len);
        prev..*total_frame_len
      }
      CopyType::Normal => {
        let mask_placeholder = if IS_CLIENT { 4 } else { 0 };
        let header_len_total = rbfi.header_len.wrapping_add(mask_placeholder);
        let header_len_total_usize = rbfi.header_len.wrapping_add(mask_placeholder).into();
        fb.set_params_through_expansion(
          0,
          header_len_total,
          rbfi.payload_len.wrapping_add(header_len_total_usize),
        );
        fb.buffer_mut().get_mut(..rbfi.header_len.into()).unwrap_or_default().copy_from_slice(
          current_frame.get(rbfi.header_begin_idx..rbfi.header_end_idx).unwrap_or_default(),
        );
        let start = header_len_total_usize;
        let end = current_frame
          .len()
          .wrapping_sub(rbfi.header_begin_idx)
          .wrapping_add(mask_placeholder.into());
        start..end
      }
    };
    fb.buffer_mut()
      .get_mut(range)
      .unwrap_or_default()
      .copy_from_slice(current_frame.get(rbfi.header_end_idx..).unwrap_or_default());
  }

  #[inline]
  async fn do_read_frame<const CHECK_TEXT_UTF8: bool>(
    &mut self,
  ) -> crate::Result<ReadBufferFrameInfo> {
    loop {
      let mut rbfi = self.fill_rb_from_stream().await?;
      let curr_frame = self.rb.current_frame_mut();
      if !IS_CLIENT {
        unmask(
          curr_frame.get_mut(rbfi.header_end_idx..).unwrap_or_default(),
          rbfi.mask.ok_or(crate::Error::NoFrameMask)?,
        );
        let n = remove_mask(
          curr_frame.get_mut(rbfi.header_begin_idx..rbfi.header_end_idx).unwrap_or_default(),
        );
        let n_usize = n.into();
        rbfi.frame_len = rbfi.frame_len.wrapping_sub(n_usize);
        rbfi.header_begin_idx = rbfi.header_begin_idx.wrapping_add(n_usize);
        rbfi.header_len = rbfi.header_len.wrapping_sub(n);
      }
      let payload = curr_frame.get(rbfi.header_end_idx..).unwrap_or_default();
      match rbfi.op_code {
        OpCode::Close if self.auto_close && !self.is_stream_closed => {
          match payload {
            [] => {}
            [_] => return Err(crate::Error::InvalidCloseFrame),
            [a, b, rest @ ..] => {
              if from_utf8_opt(rest).is_none() {
                return Err(crate::Error::InvalidUTF8);
              };
              let is_not_allowed = !CloseCode::from(u16::from_be_bytes([*a, *b])).is_allowed();
              if is_not_allowed || rest.len() > MAX_CONTROL_FRAME_PAYLOAD_LEN - 2 {
                Self::write_control_frame(
                  Frame::close_from_params(1002, <_>::default(), rest)?,
                  &mut self.is_stream_closed,
                  &mut self.rng,
                  &mut self.stream,
                )
                .await?;
                return Err(crate::Error::InvalidCloseFrame);
              }
            }
          }
          Self::write_control_frame(
            Frame::new_fin(<_>::default(), OpCode::Close, payload)?,
            &mut self.is_stream_closed,
            &mut self.rng,
            &mut self.stream,
          )
          .await?;
          break Ok(rbfi);
        }
        OpCode::Ping if self.auto_pong => {
          Self::write_control_frame(
            Frame::new_fin(<_>::default(), OpCode::Pong, payload)?,
            &mut self.is_stream_closed,
            &mut self.rng,
            &mut self.stream,
          )
          .await?;
        }
        OpCode::Text => {
          if CHECK_TEXT_UTF8 && from_utf8_opt(payload).is_none() {
            return Err(crate::Error::InvalidUTF8);
          }
          break Ok(rbfi);
        }
        OpCode::Continuation | OpCode::Binary | OpCode::Close | OpCode::Ping | OpCode::Pong => {
          break Ok(rbfi);
        }
      }
    }
  }

  async fn do_write_frame<B>(
    mut frame: Frame<FrameBuffer<B>, IS_CLIENT>,
    is_stream_closed: &mut bool,
    rng: &mut Rng,
    stream: &mut S,
  ) -> crate::Result<()>
  where
    B: BorrowMut<[u8]> + core::fmt::Debug,
  {
    if IS_CLIENT {
      let mut mask_opt = None;
      if let [_, second_byte, .., a, b, c, d] = frame.fb_mut().header_mut() {
        if !has_masked_frame(*second_byte) {
          *second_byte |= 0b1000_0000;
          let mask = rng.random_u8_4();
          *a = mask[0];
          *b = mask[1];
          *c = mask[2];
          *d = mask[3];
          mask_opt = Some(mask);
        }
      }
      if let Some(mask) = mask_opt {
        unmask(frame.fb_mut().payload_mut(), mask);
      }
    }
    if frame.op_code() == OpCode::Close {
      *is_stream_closed = true;
    }
    stream.write_all(frame.fb().frame()).await?;
    Ok(())
  }

  async fn fill_initial_rb_from_stream(
    buffer: &mut [u8],
    max_payload_size: usize,
    read: &mut usize,
    stream: &mut S,
  ) -> crate::Result<ReadBufferFrameInfo>
  where
    S: Stream,
  {
    async fn read_until<S, const LEN: usize>(
      buffer: &mut [u8],
      read: &mut usize,
      start: usize,
      stream: &mut S,
    ) -> crate::Result<[u8; LEN]>
    where
      [u8; LEN]: Default,
      S: Stream,
    {
      let until = start.wrapping_add(LEN);
      while *read < until {
        let actual_buffer = buffer.get_mut(*read..).unwrap_or_default();
        let local_read = stream.read(actual_buffer).await?;
        if local_read == 0 {
          return Err(crate::Error::UnexpectedEOF);
        }
        *read = read.wrapping_add(local_read);
      }
      Ok(buffer.get(start..until).and_then(|el| el.try_into().ok()).unwrap_or_default())
    }

    let first_two = read_until::<_, 2>(buffer, read, 0, stream).await?;

    let fin = first_two[0] & 0b1000_0000 != 0;
    let rsv1 = first_two[0] & 0b0100_0000 != 0;
    let rsv2 = first_two[0] & 0b0010_0000 != 0;
    let rsv3 = first_two[0] & 0b0001_0000 != 0;

    if rsv1 || rsv2 || rsv3 {
      return Err(crate::Error::ReservedBitsAreNotZero);
    }

    let is_masked = has_masked_frame(first_two[1]);
    let length_code = first_two[1] & 0b0111_1111;
    let op_code = op_code(first_two[0])?;

    let (mut header_len, payload_len) = match length_code {
      126 => (4, u16::from_be_bytes(read_until::<_, 2>(buffer, read, 2, stream).await?).into()),
      127 => {
        let payload_len = read_until::<_, 8>(buffer, read, 2, stream).await?;
        (10, u64::from_be_bytes(payload_len).try_into()?)
      }
      _ => (2, length_code.into()),
    };

    let mut mask = None;
    if is_masked {
      mask = Some(read_until::<_, 4>(buffer, read, header_len, stream).await?);
      header_len = header_len.wrapping_add(4);
    }

    if op_code.is_control() && !fin {
      return Err(crate::Error::FragmentedControlFrame);
    }
    if op_code == OpCode::Ping && payload_len > MAX_CONTROL_FRAME_PAYLOAD_LEN {
      return Err(crate::Error::VeryLargeControlFrame);
    }
    if payload_len >= max_payload_size {
      return Err(crate::Error::VeryLargePayload);
    }

    Ok(ReadBufferFrameInfo {
      fin,
      frame_len: header_len.wrapping_add(payload_len),
      header_begin_idx: 0,
      header_end_idx: header_len,
      header_len: header_len.try_into().unwrap_or_default(),
      mask,
      op_code,
      payload_len,
    })
  }

  async fn fill_rb_from_stream(&mut self) -> crate::Result<ReadBufferFrameInfo> {
    let mut read = self.rb.following_frames_len();
    self.rb.merge_current_frame_with_antecedent_frames();
    self.rb.expand_after_current_frame(MAX_HEADER_LEN_USIZE);
    let rbfi = Self::fill_initial_rb_from_stream(
      self.rb.after_current_frame_mut(),
      self.max_payload_size,
      &mut read,
      &mut self.stream,
    )
    .await?;
    if self.is_stream_closed && rbfi.op_code != OpCode::Close {
      return Err(crate::Error::ConnectionClosed);
    }
    loop {
      if read >= rbfi.frame_len {
        break;
      }
      self.rb.expand_after_current_frame(rbfi.frame_len);
      let local_read = self
        .stream
        .read(self.rb.after_current_frame_mut().get_mut(read..).unwrap_or_default())
        .await?;
      read = read.wrapping_add(local_read);
    }
    self.rb.set_indices_through_expansion(
      self.rb.antecedent_frames_end_idx(),
      self.rb.antecedent_frames_end_idx().wrapping_add(rbfi.frame_len),
      self.rb.antecedent_frames_end_idx().wrapping_add(read),
    );
    Ok(rbfi)
  }

  async fn manage_read_msg_loop(
    &mut self,
    fb: &mut FrameBufferVec,
    first_frame_op_code: OpCode,
    total_frame_len: &mut usize,
    mut cb: impl FnMut(&[u8]) -> crate::Result<()>,
  ) -> crate::Result<()>
  where
    S: Stream,
  {
    loop {
      let rbfi = self.do_read_frame::<false>().await?;
      Self::copy_from_rb_to_fb(CopyType::Msg(total_frame_len), fb, &self.rb, &rbfi);
      match rbfi.op_code {
        OpCode::Continuation => {
          cb(self.rb.current_frame().get(rbfi.header_end_idx..).unwrap_or_default())?;
          if rbfi.fin {
            let mut buffer = [0; MAX_HEADER_LEN_USIZE];
            let header_len = copy_header_params_to_buffer::<IS_CLIENT>(
              &mut buffer,
              true,
              first_frame_op_code,
              fb.payload().len(),
            )?;
            let start_idx = msg_header_placeholder::<IS_CLIENT>().wrapping_sub(header_len);
            fb.header_mut()
              .get_mut(start_idx.into()..)
              .unwrap_or_default()
              .copy_from_slice(buffer.get(..header_len.into()).unwrap_or_default());
            fb.set_params_through_expansion(start_idx, header_len, *total_frame_len);
            if !self.rb.has_following_frames() {
              self.rb.clear();
            }
            break;
          }
        }
        OpCode::Binary | OpCode::Close | OpCode::Ping | OpCode::Pong | OpCode::Text => {
          return Err(crate::Error::InvalidMsgFrame);
        }
      }
    }
    Ok(())
  }

  async fn write_control_frame(
    frame: FrameControlArray<IS_CLIENT>,
    is_stream_closed: &mut bool,
    rng: &mut Rng,
    stream: &mut S,
  ) -> crate::Result<()> {
    Self::do_write_frame(frame, is_stream_closed, rng, stream).await?;
    Ok(())
  }
}

#[derive(Debug)]
enum CopyType<'read> {
  Msg(&'read mut usize),
  Normal,
}

#[derive(Debug)]
struct ReadBufferFrameInfo {
  fin: bool,
  frame_len: usize,
  header_begin_idx: usize,
  header_end_idx: usize,
  header_len: u8,
  mask: Option<[u8; 4]>,
  op_code: OpCode,
  payload_len: usize,
}

pub(crate) fn copy_header_params_to_buffer<const IS_CLIENT: bool>(
  buffer: &mut [u8],
  fin: bool,
  op_code: OpCode,
  payload_len: usize,
) -> crate::Result<u8> {
  fn first_header_byte(fin: bool, op_code: OpCode) -> u8 {
    u8::from(fin) << 7 | u8::from(op_code)
  }

  fn manage_mask<const IS_CLIENT: bool, const N: u8>(
    rest: &mut [u8],
    second_byte: &mut u8,
  ) -> crate::Result<u8> {
    Ok(if IS_CLIENT {
      *second_byte &= 0b0111_1111;
      let [a, b, c, d, ..] = rest else { return Err(crate::Error::InvalidHeaderBounds); };
      *a = 0;
      *b = 0;
      *c = 0;
      *d = 0;
      N.wrapping_add(4)
    } else {
      N
    })
  }

  match payload_len {
    0..=125 => {
      if let ([a, b, rest @ ..], Ok(u8_len)) = (buffer, u8::try_from(payload_len)) {
        *a = first_header_byte(fin, op_code);
        *b = u8_len;
        return manage_mask::<IS_CLIENT, 2>(rest, b);
      }
    }
    126..=0xFFFF => {
      let rslt = u16::try_from(payload_len).map(u16::to_be_bytes);
      if let ([a, b, c, d, rest @ ..], Ok([len_c, len_d])) = (buffer, rslt) {
        *a = first_header_byte(fin, op_code);
        *b = 126;
        *c = len_c;
        *d = len_d;
        return manage_mask::<IS_CLIENT, 4>(rest, b);
      }
    }
    _ => {
      if let (
        [a, b, c, d, e, f, g, h, i, j, rest @ ..],
        Ok([len_c, len_d, len_e, len_f, len_g, len_h, len_i, len_j]),
      ) = (buffer, u64::try_from(payload_len).map(u64::to_be_bytes))
      {
        *a = first_header_byte(fin, op_code);
        *b = 127;
        *c = len_c;
        *d = len_d;
        *e = len_e;
        *f = len_f;
        *g = len_g;
        *h = len_h;
        *i = len_i;
        *j = len_j;
        return manage_mask::<IS_CLIENT, 10>(rest, b);
      }
    }
  }

  Err(crate::Error::InvalidHeaderBounds)
}

pub(crate) fn has_masked_frame(second_header_byte: u8) -> bool {
  second_header_byte & 0b1000_0000 != 0
}

pub(crate) fn op_code(first_header_byte: u8) -> crate::Result<OpCode> {
  OpCode::try_from(first_header_byte & 0b0000_1111)
}
const fn msg_header_placeholder<const IS_CLIENT: bool>() -> u8 {
  if IS_CLIENT {
    MAX_HEADER_LEN_U8
  } else {
    MAX_HEADER_LEN_U8 - 4
  }
}

fn remove_mask(header: &mut [u8]) -> u8 {
  let Some(second_header_byte) = header.get_mut(1) else {
    return 0;
  };
  if !has_masked_frame(*second_header_byte) {
    return 0;
  }
  *second_header_byte &= 0b0111_1111;
  let prev_header_len = header.len();
  let until_mask = header.get_mut(..prev_header_len.wrapping_sub(4)).unwrap_or_default();
  let mut buffer = [0u8; MAX_HEADER_LEN_USIZE - 4];
  let swap_bytes = buffer.get_mut(..until_mask.len()).unwrap_or_default();
  swap_bytes.copy_from_slice(until_mask);
  let new_header = header.get_mut(4..prev_header_len).unwrap_or_default();
  new_header.copy_from_slice(swap_bytes);
  4
}
