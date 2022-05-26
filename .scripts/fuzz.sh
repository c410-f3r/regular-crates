#!/usr/bin/env bash

set -euxo pipefail

cargo +nightly-2021-12-19 fuzz run --fuzz-dir ndstruct-fuzz coo -- -runs=100000
cargo +nightly-2021-12-19 fuzz run --fuzz-dir ndstruct-fuzz csl -- -runs=100000