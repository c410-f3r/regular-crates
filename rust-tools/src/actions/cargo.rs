use crate::{handle_cmd_output, Actions};
use std::process::Command;

macro_rules! create_fns {
  (
    $cargo_action:literal, $info:literal, $generic_fn:ident, $with_features_fn:ident
  ) => {
    pub(crate) fn $generic_fn(&self, package: String) -> crate::Result<()> {
      handle_cmd_output(Command::new("echo").args(&[
        "-e",
        &format!(
          concat!(r#"\e[0;33m***** "#, $info, r#" "{}" without features *****\e[0m\n"#),
          package
        ),
      ]))?;
      handle_cmd_output(Command::new("cargo").args(&[
        $cargo_action,
        "--manifest-path",
        &format!("{}/Cargo.toml", package),
        "--no-default-features",
      ]))?;

      handle_cmd_output(Command::new("echo").args(&[
        "-e",
        &format!(
          concat!(r#"\e[0;33m***** "#, $info, r#" "{}" with all features *****\e[0m\n"#),
          package
        ),
      ]))?;
      handle_cmd_output(Command::new("cargo").args(&[
        $cargo_action,
        "--all-features",
        "--manifest-path",
        &format!("{}/Cargo.toml", package),
      ]))?;
      Ok(())
    }

    pub(crate) fn $with_features_fn(&self, package: String, features: String) -> crate::Result<()> {
      handle_cmd_output(Command::new("echo").args(&[
        "-e",
        &format!(
          concat!(r#"\e[0;33m***** "#, $info, r#" "{}" with features "{}" *****\e[0m\n"#),
          package, features
        ),
      ]))?;
      handle_cmd_output(Command::new("cargo").args(&[
        $cargo_action,
        "--features",
        &features,
        "--manifest-path",
        &format!("{}/Cargo.toml", package),
        "--no-default-features",
      ]))?;
      Ok(())
    }
  };
}

impl Actions {
  create_fns!("build", "Building", build_generic, build_with_features);
  create_fns!("check", "Checking", check_generic, check_with_features);
  create_fns!("test", "Testing", test_generic, test_with_features);
}
