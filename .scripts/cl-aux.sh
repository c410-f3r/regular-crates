#!/usr/bin/env bash

. ".scripts/common.sh"

$rt test-with-features cl-aux
$rt test-with-features cl-aux alloc
$rt test-with-features cl-aux arrayvec
$rt test-with-features cl-aux serde
$rt test-with-features cl-aux smallvec
$rt test-with-features cl-aux std
$rt test-with-features cl-aux tinyvec
