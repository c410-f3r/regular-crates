#!/usr/bin/env bash

. ".scripts/common.sh"

$rt test-generic wtx
$rt test-with-features wtx async-std
$rt test-with-features wtx async-trait
$rt test-with-features wtx base64
$rt test-with-features wtx futures-lite
$rt test-with-features wtx glommio
$rt test-with-features wtx http
$rt test-with-features wtx httparse
$rt test-with-features wtx hyper
$rt test-with-features wtx sha1
$rt test-with-features wtx simdutf8
$rt test-with-features wtx std
$rt test-with-features wtx tokio
$rt test-with-features wtx web-socket-handshake
$rt test-with-features wtx web-socket-hyper