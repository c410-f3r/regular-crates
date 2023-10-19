use crate::csl::{Csl, CslError, CslMut, CslRef};
use cl_aux::{ArrayWrapper, Push};
use core::ops::Range;

macro_rules! create_sub_dim {
  (
    $trait:ident
    $trait_fn:ident
    $ref:ident
    $get:ident
    $line_fn:ident
    $sub_dim_fn:ident
    $([$mut:tt])?
  ) => {

#[inline]
pub(crate) fn $line_fn<'csl, 'ret, DATA, DS, IS, OS, const D: usize>(
  csl: &'csl $($mut)? Csl<DS, IS, OS, D>,
  indcs: [usize; D]
) -> Option<$ref<'ret, DATA, 1>>
where
  'csl: 'ret,
  DATA: 'csl,
  DS: $trait<[DATA]>,
  IS: AsRef<[usize]>,
  OS: AsRef<[usize]>,
{
  let last_dim = if let Some(r) = csl.dims.last() {
    *r
  }
  else {
    return None;
  };
  let [offs_indcs, offs_values] = line_offs(&csl.dims, &indcs, csl.offs.as_ref())?;
  Some($ref {
    data: csl.data.$trait_fn().$get(offs_values.clone())?,
    dims: [last_dim].into(),
    indcs: &csl.indcs.as_ref().get(offs_values)?,
    offs: &csl.offs.as_ref().get(offs_indcs)?,
  })
}

#[inline]
pub(crate) fn $sub_dim_fn<'csl, 'ret, DATA: 'csl, DS, IS, OS, const FD: usize, const TD: usize>(
  csl: &'csl $($mut)? Csl<DS, IS, OS, FD>,
  range: Range<usize>,
) -> Option<$ref<'ret, DATA, TD>>
where
  'csl: 'ret,
  DS: $trait<[DATA]>,
  IS: AsRef<[usize]>,
  OS: AsRef<[usize]>,
{
  if range.start > range.end || TD > FD {
    return None;
  }
  let data_ref = csl.data.$trait_fn();
  let dims_ref = &csl.dims;
  let indcs_ref = csl.indcs.as_ref();
  let offs_ref = csl.offs.as_ref();
  match TD {
    0 => None,
    1 => {
      let first = *offs_ref.first()?;
      let second = offs_ref.get(1)?;
      let [start_off_value, end_off_value] = [0, second.checked_sub(first)?];
      let indcs = indcs_ref.get(start_off_value..end_off_value)?;
      let start = indcs.binary_search(&range.start).unwrap_or_else(|x| x);
      let end = indcs.get(start..)?.binary_search(&range.end).unwrap_or_else(|x| x);
      let dims_ref_idx = FD.checked_sub(TD)?;
      let dims_array = ArrayWrapper::try_from_fn(|_| {
        dims_ref.get(dims_ref_idx).copied().ok_or(())
      }).ok()?;
      Some($ref {
        data: data_ref.$get(start..)?.$get(..end)?,
        dims: dims_array.into(),
        indcs: &indcs_ref.get(start..)?.get(..end)?,
        offs: &offs_ref.get(0..2)?
      })
    },
    _ => {
      let dims_ref_lower_bound = FD.checked_sub(TD)?;
      let mut dims = ArrayWrapper::try_from_fn(|idx| {
        let fun = || Some(*dims_ref.get(dims_ref_lower_bound..)?.get(idx)?);
        fun().ok_or(())
      }).ok()?;
      *dims.first_mut()? = range.end.checked_sub(range.start)?;
      let [offs_indcs, offs_values] = outermost_offs(&dims, offs_ref, range);
      Some($ref {
        data: data_ref.$get(offs_values.clone())?,
        dims,
        indcs: &indcs_ref.get(offs_values)?,
        offs: &offs_ref.get(offs_indcs)?,
      })
    },
  }
}

  };
}

create_sub_dim!(AsMut as_mut CslMut get_mut line_mut sub_dim_mut [mut]);
create_sub_dim!(AsRef as_ref CslRef get line sub_dim);

// Max offset length is usize::MAX - 1
#[inline]
pub(crate) fn correct_offs_len<const D: usize>(dims: &[usize; D]) -> crate::Result<usize> {
  match D {
    0 => Ok(1),
    1 => Ok(2),
    _ if dims == &ArrayWrapper::default().0 => Ok(1),
    _ => {
      let mut offs_len: usize = 1;
      for dim in dims.iter().copied().rev().skip(1).filter(|dim| dim != &0) {
        offs_len = offs_len.saturating_mul(dim);
      }
      offs_len.checked_add(1).ok_or_else(|| CslError::OffsLengthOverflow.into())
    }
  }
}

#[inline]
pub(crate) fn data_idx<DATA, DS, IS, OS, const D: usize>(
  csl: &Csl<DS, IS, OS, D>,
  indcs: [usize; D],
) -> Option<usize>
where
  DS: AsRef<[DATA]>,
  IS: AsRef<[usize]>,
  OS: AsRef<[usize]>,
{
  let innermost_idx = indcs.last()?;
  let [_, offs_values] = line_offs(&csl.dims, &indcs, csl.offs.as_ref())?;
  let start = offs_values.start;
  if let Ok(x) = csl.indcs.as_ref().get(offs_values)?.binary_search(innermost_idx) {
    start.checked_add(x)
  } else {
    None
  }
}

#[inline]
pub(crate) fn line_offs<const D: usize>(
  dims: &[usize; D],
  indcs: &[usize; D],
  offs: &[usize],
) -> Option<[Range<usize>; 2]> {
  match D {
    0 => None,
    1 => Some({
      let off_end = offs.get(1)?.saturating_sub(*offs.first()?);
      [0..2, 0..off_end]
    }),
    _ => {
      let diff = indcs.len().saturating_sub(2);
      let mut lines: usize = 0;
      for (idx, curr_idx) in indcs.iter().copied().enumerate().take(diff) {
        let skip = idx.checked_add(1)?;
        let product = dims.iter().skip(skip).rev().skip(1).product::<usize>();
        lines = lines.saturating_add(product.saturating_mul(curr_idx));
      }
      let line_idx = dims.len().checked_sub(2)?;
      lines = lines.saturating_add(*indcs.get(line_idx)?);
      if lines > usize::MAX.saturating_sub(2) {
        return None;
      }
      let first = *offs.first()?;
      let off_start = offs.get(lines)?.saturating_sub(first);
      let lines_plus_one = lines.checked_add(1)?;
      let off_end = offs.get(lines_plus_one)?.saturating_sub(first);
      Some([lines..lines.saturating_add(2), off_start..off_end])
    }
  }
}

#[inline]
pub(crate) fn outermost_offs<const D: usize>(
  dims: &[usize; D],
  offs: &[usize],
  range: Range<usize>,
) -> [Range<usize>; 2] {
  let outermost_stride = outermost_stride(dims);
  let start_off_idx = outermost_stride.saturating_mul(range.start);
  let end_off_idx = outermost_stride.saturating_mul(range.end);
  let off_start = *offs.get(start_off_idx).unwrap_or(&0);
  let off_end = *offs.get(end_off_idx).unwrap_or(&0);
  [start_off_idx..end_off_idx.saturating_add(1), off_start..off_end]
}

#[inline]
pub(crate) fn outermost_stride<const D: usize>(dims: &[usize; D]) -> usize {
  dims.iter().skip(1).rev().skip(1).product::<usize>()
}

#[inline]
pub(crate) fn manage_last_offset<OS>(offs: &mut OS) -> crate::Result<usize>
where
  OS: AsRef<[usize]> + Push<usize>,
{
  Ok(if let Some(rslt) = offs.as_ref().last() {
    *rslt
  } else {
    offs.push(0).map_err(|_err| crate::Error::InsufficientCapacity)?;
    0
  })
}
