use crate::utils::windows2;

macro_rules! create_value {
  ($get:ident $fn_name:ident $([$mut:tt])?) => {
    #[inline]
    pub(crate) fn $fn_name<DATA, const D: usize>(
      indcs: [usize; D],
      data: &$($mut)? [([usize; D], DATA)],
    ) -> Option<&$($mut)? DATA> {
      if let Ok(idx) = data.binary_search_by(|value| value.0.cmp(&indcs)) {
        Some(&$($mut)? data.$get(idx)?.1)
      } else {
        None
      }
    }
  }
}

create_value!(get value);
create_value!(get_mut value_mut [mut]);

#[inline]
pub(crate) fn does_not_have_duplicates_sorted<F, T>(slice: &[T], mut cb: F) -> bool
where
  F: FnMut(&T, &T) -> bool,
{
  windows2(slice).all(|[a, b]| cb(a, b))
}
