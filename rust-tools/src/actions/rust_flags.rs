use crate::Actions;
use std::io::{stdout, Write};

impl Actions {
  pub(crate) fn rust_flags(&self) -> crate::Result<()> {
    let mut iter = self.params.rust_flags.iter();
    let mut stdout = stdout();
    if let Some(first) = iter.next() {
      stdout.write_all(first.as_bytes())?;
    }
    for element in iter {
      stdout.write_all(br#" "#)?;
      stdout.write_all(element.as_bytes())?;
    }
    stdout.flush()?;
    Ok(())
  }
}
