/// Unmasks a sequence of bytes using the given 4-byte `mask`.
#[inline]
pub fn unmask(bytes: &mut [u8], mask: [u8; 4]) {
  unmask_aligned_slice(bytes, mask);
}

fn unmask_aligned_slice(bytes: &mut [u8], mask: [u8; 4]) {
  let mut mask_u32 = u32::from_ne_bytes(mask);
  #[allow(unsafe_code)]
  // SAFETY: Changing a sequence of `u8` to `u32` should be fine
  let (prefix, words, suffix) = unsafe { bytes.align_to_mut::<u32>() };
  unmask_chunks_of_slice(prefix, mask);
  let mut shift = u32::try_from(prefix.len() & 3).unwrap_or_default();
  if shift > 0 {
    shift = shift.wrapping_mul(8);
    if cfg!(target_endian = "big") {
      mask_u32 = mask_u32.rotate_left(shift);
    } else {
      mask_u32 = mask_u32.rotate_right(shift);
    }
  }
  for word in words.iter_mut() {
    *word ^= mask_u32;
  }
  unmask_chunks_of_slice(suffix, mask_u32.to_ne_bytes());
}

fn unmask_chunks_of_slice(bytes: &mut [u8], mask: [u8; 4]) {
  let mut bytes_skip: usize = 0;
  #[cfg(feature = "async-trait")]
  {
    let mut iter = bytes.chunks_exact_mut(4);
    while let Some([a, b, c, d]) = iter.next() {
      *a ^= mask[0];
      *b ^= mask[1];
      *c ^= mask[2];
      *d ^= mask[3];
      bytes_skip = bytes_skip.wrapping_add(1);
    }
  }
  #[cfg(not(feature = "async-trait"))]
  for [a, b, c, d] in bytes.array_chunks_mut::<4>() {
    *a ^= mask[0];
    *b ^= mask[1];
    *c ^= mask[2];
    *d ^= mask[3];
    bytes_skip = bytes_skip.wrapping_add(1);
  }
  bytes_skip = bytes_skip.wrapping_mul(4);
  bytes.get_mut(bytes_skip..).into_iter().flatten().zip(mask).for_each(|(byte, mask_el)| {
    *byte ^= mask_el;
  });
}

#[cfg(test)]
mod tests {
  use crate::{misc::Rng, web_socket::mask::unmask};
  use alloc::{vec, vec::Vec};

  #[test]
  fn test_unmask() {
    let mut payload = [0u8; 33];
    let mask = [1, 2, 3, 4];
    unmask(&mut payload, mask);
    assert_eq!(
      &payload,
      &[
        1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
        3, 4, 1
      ]
    );
  }

  #[test]
  fn length_variation_unmask() {
    for len in &[0, 2, 3, 8, 16, 18, 31, 32, 40] {
      let mut payload = vec![0u8; *len];
      let mask = [1, 2, 3, 4];
      unmask(&mut payload, mask);

      let expected = (0..*len).map(|i| (i & 3) as u8 + 1).collect::<Vec<_>>();
      assert_eq!(payload, expected);
    }
  }

  #[test]
  fn length_variation_unmask_2() {
    for len in &[0, 2, 3, 8, 16, 18, 31, 32, 40] {
      let mut payload = vec![0u8; *len];
      let mask = Rng::default().random_u8_4();
      unmask(&mut payload, mask);
      let expected = (0..*len).map(|i| mask[i & 3]).collect::<Vec<_>>();
      assert_eq!(payload, expected);
    }
  }
}
