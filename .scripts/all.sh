#!/usr/bin/env bash

. ".scripts/common.sh"

$rt rustfmt
$rt clippy

.scripts/rust-tools.sh

cargo test --all-features