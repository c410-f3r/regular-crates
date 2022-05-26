use crate::Actions;
use std::{fs::File, io::Write};

impl Actions {
  pub(crate) fn set_up(&self) -> crate::Result<()> {
    if !self.params.toolchain.is_empty() {
      let mut file = File::create("rust-toolchain")?;
      file.write_all(self.params.toolchain.as_bytes())?;
    }

    if !self.params.rustfmt_flags.is_empty() {
      let mut file = File::create("rustfmt.toml")?;
      for rustfmt_flag in &self.params.rustfmt_flags {
        file.write_all(rustfmt_flag.as_bytes())?;
        file.write_all(b"\n")?;
      }
    }
    Ok(())
  }
}
