#[derive(Default)]
pub(crate) struct TransformingParams {
  pub(crate) add_clippy_flags: Vec<String>,
  pub(crate) add_rust_flags: Vec<String>,
  pub(crate) add_rustfmt_flags: Vec<String>,
  pub(crate) rm_clippy_flags: Vec<String>,
  pub(crate) rm_rust_flags: Vec<String>,
  pub(crate) rm_rustfmt_flags: Vec<String>,
  pub(crate) toolchain: String,
}
