#!/usr/bin/env bash

. ".scripts/common.sh"

$rt rustfmt
$rt clippy "" "" --all-features --package=rust-tools

$rt test-with-features rust-tools