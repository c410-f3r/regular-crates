use alloc::{vec, vec::Vec};

/// Used to avoid syscalls.
///
/// `read_exact` can eliminate the necessity of this structure but the performance
/// impact of such approach is questionable.
///
/// ```txt
/// [                             buffer                            ]
/// [ antecedent frames | current frame | following frames | suffix ]
/// ```
#[derive(Debug)]
pub(crate) struct ReadBuffer {
  antecedent_frames_end_idx: usize,
  buffer: Vec<u8>,
  current_frame_end_idx: usize,
  following_frames_end_idx: usize,
}

impl ReadBuffer {
  pub(crate) fn with_capacity(len: usize) -> Self {
    Self {
      antecedent_frames_end_idx: 0,
      buffer: vec![0; len],
      current_frame_end_idx: 0,
      following_frames_end_idx: 0,
    }
  }

  pub(crate) fn antecedent_frames_end_idx(&self) -> usize {
    self.antecedent_frames_end_idx
  }

  pub(crate) fn after_current_frame_mut(&mut self) -> &mut [u8] {
    self.buffer.get_mut(self.current_frame_end_idx..).unwrap_or_default()
  }

  pub(crate) fn clear(&mut self) {
    self.antecedent_frames_end_idx = 0;
    self.current_frame_end_idx = 0;
    self.following_frames_end_idx = 0;
  }

  pub(crate) fn current_frame(&self) -> &[u8] {
    self.buffer.get(self.antecedent_frames_end_idx..self.current_frame_end_idx).unwrap_or_default()
  }

  pub(crate) fn current_frame_mut(&mut self) -> &mut [u8] {
    self
      .buffer
      .get_mut(self.antecedent_frames_end_idx..self.current_frame_end_idx)
      .unwrap_or_default()
  }

  pub(crate) fn expand_after_current_frame(&mut self, mut new_len: usize) {
    new_len = self.current_frame_end_idx.wrapping_add(new_len);
    if new_len > self.buffer.len() {
      self.buffer.resize(new_len, 0);
    }
  }

  pub(crate) fn expand_buffer(&mut self, new_len: usize) {
    if new_len > self.buffer.len() {
      self.buffer.resize(new_len, 0);
    }
  }

  pub(crate) fn following_frames_len(&self) -> usize {
    self.following_frames_end_idx.wrapping_sub(self.current_frame_end_idx)
  }

  pub(crate) fn has_following_frames(&self) -> bool {
    self.following_frames_end_idx > self.current_frame_end_idx
  }

  pub(crate) fn merge_current_frame_with_antecedent_frames(&mut self) {
    self.antecedent_frames_end_idx = self.current_frame_end_idx;
  }

  pub(crate) fn set_indices_through_expansion(
    &mut self,
    antecedent_frames_end_idx: usize,
    current_frame_end_idx: usize,
    following_frames_end_idx: usize,
  ) {
    self.antecedent_frames_end_idx = antecedent_frames_end_idx;
    self.current_frame_end_idx = self.antecedent_frames_end_idx.max(current_frame_end_idx);
    self.following_frames_end_idx = self.current_frame_end_idx.max(following_frames_end_idx);
    self.expand_buffer(self.following_frames_end_idx);
  }
}
