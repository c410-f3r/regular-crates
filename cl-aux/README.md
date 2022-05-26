# Auxiliary elements for collections

[![CI](https://github.com/c410-f3r/cl-traits/workflows/CI/badge.svg)](https://github.com/c410-f3r/cl-traits/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/cl-traits.svg)](https://crates.io/crates/cl-traits)
[![Documentation](https://docs.rs/cl-traits/badge.svg)](https://docs.rs/cl-traits)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
![Rustc](https://img.shields.io/badge/rustc-1.52-lightgray)

Yet another library that generalizes collections.

This crate provides a single method for each `trait` to achieve maximum flexibility and freedom instead of imposing an abstraction subset for all situations and users.

## Examples

```rust
use cl_aux::*;

struct SomeCustomVector(Vec<i32>, Vec<i32>);

impl Length for SomeCustomVector {
  #[inline]
  fn length(&self) -> usize {
    self.0.length() + self.1.length()
  }
}

fn main() {
  let v = SomeCustomVector(vec![1, 2], vec![3, 4, 5, 6]);
  assert_eq!(v.length(), 6);
}
```
