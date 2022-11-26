# Auxiliary elements for collections

[![CI](https://github.com/c410-f3r/cl-traits/workflows/CI/badge.svg)](https://github.com/c410-f3r/cl-traits/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/cl-traits.svg)](https://crates.io/crates/cl-traits)
[![Documentation](https://docs.rs/cl-traits/badge.svg)](https://docs.rs/cl-traits)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
![Rustc](https://img.shields.io/badge/rustc-1.52-lightgray)

Provides well-defined traits with single methods that enable the achievement of maximum flexibility and freedom in several different operations instead of imposing abstract subsets.

```rust
use cl_aux::Length;

struct SomeCustomArray([i32; 2], [i32; 4]);

impl Length for SomeCustomArray {
  #[inline]
  fn length(&self) -> usize {
    self.0.length() + self.1.length()
  }
}

fn main() {
  let v = SomeCustomArray([1, 2], [3, 4, 5, 6]);
  assert_eq!(v.length(), 6);
}
```

Also provides structures for common use-cases.

```rust
use cl_aux::ArrayWrapper;

fn main() {
  let _array: [usize; 1] = ArrayWrapper::from_fn(|idx| idx).0;
}
```
