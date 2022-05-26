use crate::{CfgOption, Params, TransformingParams};
use std::io::{BufRead, BufReader, Read};

pub(crate) fn parse_cfg<R>(read: R) -> crate::Result<(Params, TransformingParams)>
where
  R: Read,
{
  let mut br = BufReader::new(read);
  let mut overall_buffer = String::with_capacity(16);
  let mut params = Params::default();
  let mut tp = TransformingParams::default();

  iterations(&mut overall_buffer, &mut br, |_| false)?;
  macro_rules! push {
    (vec: $name:ident) => {
      push!($name, |rslt: &str| {
        for value in rslt.split(',') {
          tp.$name.push(value.trim().into());
        }
        Ok(())
      })
    };
    ($name:ident, $cb:expr) => {
      let name = stringify!($name);
      if let Some(rslt) = overall_buffer.split(name).nth(1) {
        let cb: crate::Result<()> = $cb(rslt);
        cb?;
        iterations(&mut overall_buffer, &mut br, |_| false)?;
      }
    };
  }

  push!(vec: add_clippy_flags);
  push!(vec: rm_clippy_flags);
  push!(vec: add_rust_flags);
  push!(vec: rm_rust_flags);
  push!(vec: add_rustfmt_flags);
  push!(vec: rm_rustfmt_flags);
  push!(template, |rslt: &str| {
    params = rslt.trim().parse::<CfgOption>()?.into_params();
    Ok(())
  });
  push!(toolchain, |rslt: &str| {
    tp.toolchain = rslt.trim().into();
    Ok(())
  });
  Ok((params, tp))
}

#[inline]
fn iterations<F, R>(
  overall_buffer: &mut String,
  br: &mut BufReader<R>,
  mut cb: F,
) -> crate::Result<()>
where
  F: FnMut(&str) -> bool,
  R: Read,
{
  overall_buffer.clear();
  let mut bytes_read = 0;

  loop {
    let curr_bytes_read = br.read_line(overall_buffer)?;

    if curr_bytes_read == 0 {
      break;
    }

    let str_read = if let Some(rslt) = overall_buffer.get(bytes_read..) {
      rslt
    } else {
      break;
    };
    let trimmed = str_read.trim();

    bytes_read = bytes_read.saturating_add(curr_bytes_read);

    if trimmed.is_empty() || trimmed.starts_with("//") {
      continue;
    }

    if !cb(trimmed) {
      break;
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{cfg::YouRust, parse_cfg};

  #[test]
  fn parsed_configuration_has_correct_params() {
    let cfg = br#"
            add_clippy_flags A
            rm_clippy_flags B, C
            add_rust_flags D
            rm_rust_flags E
            add_rustfmt_flags F
            template you-rust
            toolchain nightly-2020-11-11
        "#;
    let (params, tp) = parse_cfg(&cfg[..]).unwrap();
    assert_eq!(params, YouRust::default().0);
    assert_eq!(tp.add_clippy_flags, vec!["A"]);
    assert_eq!(tp.rm_clippy_flags, vec!["B", "C"]);
    assert_eq!(tp.add_rust_flags, vec!["D"]);
    assert_eq!(tp.rm_rust_flags, vec!["E"]);
    assert_eq!(tp.add_rustfmt_flags, vec!["F"]);
    assert_eq!(tp.rm_rustfmt_flags, Vec::<String>::new());
    assert_eq!(tp.toolchain, "nightly-2020-11-11");
  }
}
