mod cargo;
mod rust_flags;
mod set_up;
mod target_dir;
mod tools;

use crate::Params;

pub(crate) struct Actions {
  pub(crate) params: Params,
}

impl Actions {
  #[inline]
  pub(crate) fn new(params: Params) -> Self {
    Self { params }
  }
}
