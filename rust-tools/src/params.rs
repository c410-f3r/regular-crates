use crate::TransformingParams;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Params {
  pub(crate) clippy_flags: Vec<String>,
  pub(crate) rust_flags: Vec<String>,
  pub(crate) rustfmt_flags: Vec<String>,
  pub(crate) toolchain: String,
}

impl Params {
  pub(crate) fn modify(&mut self, tp: &TransformingParams) {
    manage_flags(&mut self.clippy_flags, &tp.add_clippy_flags, &tp.rm_clippy_flags);
    manage_flags(&mut self.rust_flags, &tp.add_rust_flags, &tp.rm_rust_flags);
    manage_flags(&mut self.rustfmt_flags, &tp.add_rustfmt_flags, &tp.rm_rustfmt_flags);
    if !tp.toolchain.is_empty() {
      self.toolchain = tp.toolchain.clone();
    }
  }
}

fn manage_flags(flags: &mut Vec<String>, to_add: &[String], to_subtract: &[String]) {
  let iter = flags
    .iter()
    .chain(to_add)
    .filter(move |flag| !flag.is_empty() && to_subtract.iter().all(|s| s != *flag));
  *flags = iter.cloned().collect();
}
