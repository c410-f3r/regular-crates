[![CI](https://github.com/c410-f3r/rust-tools/workflows/CI/badge.svg)](https://github.com/c410-f3r/rust-tools/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/rust-tools.svg)](https://crates.io/crates/rust-tools)
[![Documentation](https://docs.rs/rust-tools/badge.svg)](https://docs.rs/rust-tools)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-stable-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-stable.html)

A CLI intended to aid development and CI workflows. This project can be seen as a high-level "orchestrator" or "wrapper" for tools that involve the Rust programing language.

## Configurations

The application will primarily interact with a configuration (collection of rules) that defines how tools should behave and there are three different sources:

### Built-in configuration

Pre-fixed set of rules found within this repository. Feel free to open a PR to create a new configuration.

```bash
rust-tools --template you-rust SOME_COMMAND
```

```txt
YouRust(
    Params {
        clippy_flags: [
            "-Dclippy::restriction",
            "-Dwarnings",
            "-Aclippy::implicit_return",
            "-Aclippy::missing_docs_in_private_items",
        ],
        rust_flags: [
            "-Dbad_style",
            "-Dfuture_incompatible",
            "-Dmissing_debug_implementations",
            "-Dmissing_docs",
            "-Dnonstandard_style",
            "-Drust_2018_compatibility",
            "-Drust_2018_idioms",
            "-Dtrivial_casts",
            "-Dunused_lifetimes",
            "-Dunused_qualifications",
            "-Dwarnings",
        ],
        rustfmt_flags: [
            "edition=\"2018",
            "tab_spaces=2",
            "use_field_init_shorthand=true",
            "use_small_heuristics=\"Max",
        ],
        toolchain: "",
    },
)
```

### Configuration file

Can use a built-in configuration as a template for custom needs. Enables the addition or removal of flags.

```bash
rust-tools --file SOME_CONFIGURATION_FILE.cfg SOME_COMMAND
```

```ini
add_clipy_flags -Aclippy::type_complexity
rm_clippy_flags -Aclippy::implicit_return,-Aclippy::missing_docs_in_private_items
template you-rust
toolchain nightly-2020-11-11
```

## CLI parameters

Depending on the selected target, it is possible to define or extend certain rules;

```bash
rust-tools rust-flags -Dbad_style,-Dunused_qualifications
```

The final configuration will obey the following order: `built-in` -> `file` -> `CLI`, i.e., built-in parameters are overwritten by file parameters and file parameters are overwritten by cli parameters.

## Script example

```bash
#!/usr/bin/env bash

set -euxo pipefail

cargo install rust-tools

export RUSTFLAGS="$(rust-tools rust-flags)"

rust-tools clippy

rust-tools check-generic SOME_CRATE
rust-tools test-with-features SOME_CRATE FIRST_FEATURE,SECOND_FEATURE
```

## Supported targets

- clippy flags
- rust flags
- rust-toolchain file
- rustfmt file and flags

