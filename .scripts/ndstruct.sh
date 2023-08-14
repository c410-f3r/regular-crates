#!/usr/bin/env bash

. ".scripts/common.sh"

$rt test-generic ndstruct
$rt test-with-features ndstruct alloc
$rt test-with-features ndstruct std
$rt test-with-features ndstruct rand
$rt test-with-features ndstruct rayon
$rt test-with-features ndstruct serde
