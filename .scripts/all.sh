#!/usr/bin/env bash

. ".scripts/common.sh"

$rt rustfmt
$rt clippy

.scripts/cl-aux.sh
.scripts/ndstruct.sh
.scripts/rust-tools.sh
.scripts/wtx.sh

cargo test --all-features --doc