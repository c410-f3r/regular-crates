#!/usr/bin/env bash

set -euxo pipefail

cargo fuzz run --fuzz-dir ndstruct-fuzz coo -- -runs=100000
cargo fuzz run --fuzz-dir ndstruct-fuzz csl -- -runs=100000
cargo fuzz run --fuzz-dir wtx-fuzz unmask -- -runs=100000
cargo fuzz run --fuzz-dir wtx-fuzz parse-frame -- -runs=100000
