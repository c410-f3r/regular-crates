//! Rust tools

#[macro_use]
mod macros;

mod action_option;
mod actions;
mod cfg;
mod cfg_option;
mod error;
mod params;
mod parse_cfg;
mod transforming_params;

use action_option::ActionOption;
use actions::Actions;
use cfg_option::CfgOption;
use error::Error;
use params::Params;
use parse_cfg::parse_cfg;
use std::{
  env::{args, Args},
  fs::File,
  io::{stderr, stdout, BufRead, BufReader, Write},
  process::Command,
};
use transforming_params::TransformingParams;

type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
  let mut args = args();
  let _first_arg = arg(&mut args)?;
  let mut maybe_action = arg(&mut args)?;

  let mut param = |name: &str| {
    Ok::<_, Error>(if maybe_action == name {
      let rslt = arg(&mut args)?;
      maybe_action = arg(&mut args)?;
      rslt
    } else {
      <_>::default()
    })
  };

  let file = param("--file")?;
  let (mut params, mut tp) =
    if file.is_empty() { Default::default() } else { parse_cfg(File::open(file)?)? };

  let template = param("--template")?;
  if !template.is_empty() {
    params = template.parse::<CfgOption>()?.into_params();
  }

  let toolchain = param("--toolchain")?;
  if !toolchain.is_empty() {
    tp.toolchain = toolchain;
  }

  parse_action(&mut args, &maybe_action, params, tp)?;

  Ok(())
}

fn handle_cmd_output(cmd: &mut Command) -> Result<()> {
  let mut buffer = String::new();
  let mut child = cmd.spawn()?;
  macro_rules! write_stdio {
    ($inner:expr, $output:expr) => {
      let mut br = BufReader::new($inner);
      while br.read_line(&mut buffer)? != 0 {
        $output.write_all(buffer.as_bytes())?;
        buffer.clear();
      }
    };
  }
  if let Some(ref mut child_stderr) = child.stderr {
    write_stdio!(child_stderr, stderr());
  }
  if let Some(ref mut child_stdout) = child.stdout {
    write_stdio!(child_stdout, stdout());
  }
  if !child.wait()?.success() {
    return Err(Error::FailedCommand);
  }
  Ok(())
}

fn opt(args: &mut Args) -> String {
  args.next().unwrap_or_default()
}

fn parse_action(
  args: &mut Args,
  action_string: &str,
  params: Params,
  mut tp: TransformingParams,
) -> Result<()> {
  let mut actions = Actions::new(params);
  match action_string.parse()? {
    ActionOption::BuildGeneric => {
      actions.params.modify(&tp);
      actions.build_generic(arg(args)?)?;
    }
    ActionOption::BuildWithFeatures => {
      actions.params.modify(&tp);
      actions.build_with_features(arg(args)?, opt(args))?;
    }
    ActionOption::CheckGeneric => {
      actions.params.modify(&tp);
      actions.check_generic(arg(args)?)?;
    }
    ActionOption::CheckWithFeatures => {
      actions.params.modify(&tp);
      actions.check_with_features(arg(args)?, opt(args))?;
    }
    ActionOption::Clippy => {
      tp.add_clippy_flags.extend(opt(args).split(',').map(Into::into));
      tp.rm_clippy_flags.extend(opt(args).split(',').map(Into::into));
      actions.params.modify(&tp);
      actions.clippy(args)?;
    }
    ActionOption::RustFlags => {
      tp.add_rust_flags.extend(opt(args).split(',').map(Into::into));
      tp.rm_rust_flags.extend(opt(args).split(',').map(Into::into));
      actions.params.modify(&tp);
      actions.rust_flags()?;
    }
    ActionOption::Rustfmt => {
      actions.params.modify(&tp);
      actions.rustfmt()?;
    }
    ActionOption::SetUp => {
      actions.params.modify(&tp);
      actions.set_up()?;
    }
    ActionOption::TargetDir => {
      actions.params.modify(&tp);
      actions.target_dir()?;
    }
    ActionOption::TestGeneric => {
      actions.params.modify(&tp);
      actions.test_generic(arg(args)?)?;
    }
    ActionOption::TestWithFeatures => {
      actions.params.modify(&tp);
      actions.test_with_features(arg(args)?, opt(args))?;
    }
  };
  Ok(())
}

fn arg(args: &mut Args) -> Result<String> {
  args.next().ok_or(Error::WrongNumberOfArgs { expected: 1, received: 0 })
}
