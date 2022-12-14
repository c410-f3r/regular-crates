use crate::csl::{csl_utils::outermost_offs, CslError, CslMut, CslRef};
use core::mem;

macro_rules! impl_iter {
  ($csl_iter:ident, $data_type:ty, $split_at:ident, $ref:ident) => {
    /// Iterator of a CSL dimension.
    #[derive(Debug, Eq, PartialEq)]
    pub struct $csl_iter<'slices, T, const D: usize> {
      curr_idx: usize,
      data: $data_type,
      dims: [usize; D],
      indcs: &'slices [usize],
      max_idx: usize,
      offs: &'slices [usize],
    }

    impl<'slices, T, const D: usize> $csl_iter<'slices, T, D> {
      pub(crate) fn new(
        mut dims: [usize; D],
        data: $data_type,
        indcs: &'slices [usize],
        offs: &'slices [usize],
      ) -> crate::Result<Self> {
        if let Some(r) = dims.first_mut() {
          let max_idx = *r;
          *r = 1;
          Ok($csl_iter { curr_idx: 0, data, dims, indcs, max_idx, offs })
        } else {
          Err(CslError::InvalidIterDim.into())
        }
      }

      #[cfg(feature = "rayon")]
      pub(crate) fn split_at(self, idx: usize) -> Option<[Self; 2]> {
        let cut_point = self.curr_idx.checked_add(idx)?;
        let [_, values] = outermost_offs(&self.dims, self.offs, self.curr_idx..cut_point);
        let values_diff = values.end.checked_sub(values.start)?;
        let (data_head, data_tail) = self.data.$split_at(values_diff);
        let (indcs_head, indcs_tail) = self.indcs.split_at(values_diff);
        Some([
          $csl_iter {
            curr_idx: self.curr_idx,
            data: data_head,
            dims: self.dims,
            indcs: indcs_head,
            max_idx: cut_point,
            offs: self.offs,
          },
          $csl_iter {
            curr_idx: cut_point,
            data: data_tail,
            dims: self.dims,
            indcs: indcs_tail,
            max_idx: self.max_idx,
            offs: self.offs,
          },
        ])
      }
    }

    impl<T, const D: usize> DoubleEndedIterator for $csl_iter<'_, T, D> {
      #[inline]
      fn next_back(&mut self) -> Option<Self::Item> {
        if self.curr_idx == 0 {
          return None;
        }
        let curr_idx_less_one = self.curr_idx.checked_sub(1)?;
        let range = curr_idx_less_one..self.curr_idx;
        self.curr_idx = curr_idx_less_one;
        let [indcs, values] = outermost_offs(&self.dims, self.offs, range);
        let data = mem::take(&mut self.data);
        let values_diff = values.end.checked_sub(values.start)?;
        let (data_head, data_tail) = data.$split_at(values_diff);
        let (indcs_head, indcs_tail) = self.indcs.split_at(values_diff);
        self.data = data_tail;
        self.indcs = indcs_tail;
        Some($ref {
          data: data_head,
          dims: self.dims.into(),
          indcs: indcs_head,
          offs: self.offs.get(indcs)?,
        })
      }
    }

    impl<T, const D: usize> ExactSizeIterator for $csl_iter<'_, T, D> {}

    impl<'slices, T, const D: usize> Iterator for $csl_iter<'slices, T, D> {
      type Item = $ref<'slices, T, D>;

      #[inline]
      fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx >= self.max_idx {
          return None;
        }
        let curr_idx_plus_one = self.curr_idx.checked_add(1)?;
        let range = self.curr_idx..curr_idx_plus_one;
        self.curr_idx = curr_idx_plus_one;
        let [indcs, values] = outermost_offs(&self.dims, self.offs, range);
        let data = mem::take(&mut self.data);
        let values_diff = values.end.checked_sub(values.start)?;
        let (data_head, data_tail) = data.$split_at(values_diff);
        let (indcs_head, indcs_tail) = self.indcs.split_at(values_diff);
        self.data = data_tail;
        self.indcs = indcs_tail;
        Some($ref {
          data: data_head,
          dims: self.dims.into(),
          indcs: indcs_head,
          offs: self.offs.get(indcs)?,
        })
      }

      #[inline]
      fn size_hint(&self) -> (usize, Option<usize>) {
        (self.max_idx, Some(self.max_idx))
      }
    }
  };
}

impl_iter!(CslLineIterMut, &'slices mut [T], split_at_mut, CslMut);
impl_iter!(CslLineIterRef, &'slices [T], split_at, CslRef);
