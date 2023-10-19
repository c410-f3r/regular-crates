#!/usr/bin/env bash

set -euxo pipefail

cargo fuzz run --features libfuzzer-sys/link_libfuzzer --fuzz-dir ndstruct-fuzz coo -- -runs=100000
cargo fuzz run --features libfuzzer-sys/link_libfuzzer --fuzz-dir ndstruct-fuzz csl -- -runs=100000
