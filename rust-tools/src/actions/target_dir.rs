use crate::Actions;
use std::io::{stdout, Write};

const TARGET_DIR: &str = "target/rust-tools-target";

impl Actions {
  pub(crate) fn target_dir(&self) -> crate::Result<()> {
    let mut stdout = stdout();
    stdout.write_all(TARGET_DIR.as_bytes())?;
    stdout.flush()?;
    Ok(())
  }
}
