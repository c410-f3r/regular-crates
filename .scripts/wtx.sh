#!/usr/bin/env bash

. ".scripts/common.sh"

$rt test-generic wtx
$rt test-with-features wtx async-std
$rt test-with-features wtx async-trait
$rt test-with-features wtx base64
$rt test-with-features wtx http-client
$rt test-with-features wtx http-types
$rt test-with-features wtx hyper
$rt test-with-features wtx sha1
$rt test-with-features wtx simdutf8
$rt test-with-features wtx std
$rt test-with-features wtx tokio