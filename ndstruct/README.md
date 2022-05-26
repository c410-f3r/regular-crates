# ndstruct 

[![CI](https://github.com/c410-f3r/ndstruct/workflows/CI/badge.svg)](https://github.com/c410-f3r/ndstruct/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/ndstruct.svg)](https://crates.io/crates/ndstruct)
[![Documentation](https://docs.rs/ndstruct/badge.svg)](https://docs.rs/ndstruct)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.52-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.52.html)

Structures to store and retrieve N-dimensional data. Well, not any `N ∈ ℕ` but any natural number that fits into the pointer size of the machine that you are using. E.g., an 8-bit microcontroller can manipulate any structure with up to 255 dimensions.

For those that might be wondering about why this crate should be used, it generally comes down to space-efficiency, ergometrics and retrieving speed. The following snippet shows some use-cases for potential replacement with `_cube_of_vecs` being the most inefficient of all.

```rust
let _vec_of_options: Vec<Option<i32>> = Default::default();
let _matrix_of_options: [Option<[Option<i32>; 8]>; 16] = Default::default();
let _cube_of_vecs: Vec<Vec<Vec<i32>>> = Default::default();
// The list worsens exponentially for higher dimensions
```

## Example

```rust
use ndstruct::{coo::CooArray, csl::CslVec};

fn main() -> ndstruct::Result<()> {
  // A CSL and COO cube.
  //
  //      ___ ___
  //    /   /   /\
  //   /___/___/ /\
  //  / 1 /   /\/2/
  // /_1_/___/ /\/
  // \_1_\___\/ /
  //  \___\___\/
  let coo = CooArray::new([2, 2, 2], [([0, 0, 0], 1.0), ([1, 1, 1], 2.0)])?;
  let mut csl = CslVec::default();
  csl
    .constructor()?
    .next_outermost_dim(2)?
    .push_line([(0, 1.0)].iter().copied())?
    .next_outermost_dim(2)?
    .push_empty_line()?
    .next_outermost_dim(2)?
    .push_empty_line()?
    .push_line([(1, 2.0)].iter().copied())?;
  assert_eq!(coo.value([0, 0, 0]), csl.value([0, 0, 0]));
  assert_eq!(coo.value([1, 1, 1]), csl.value([1, 1, 1]));
  Ok(())
}
```

## Supported structures

- Compressed Sparse Line (CSL)
- Coordinate format (COO)
- Dense

## Features

- `no_std`
- Different storages (Array, Vec, Slice and more!)
- Fully documented
- Fuzz testing
- No unsafe

## Optional features

- `alloc` and `std`
- Bindings (Py03, wasm-bindgen)
- Deserialization/Serialization (serde)
- Parallel iterators (rayon)
- Random instances (rand)

## Future

Although CSR and COO are general sparse structures, they aren't good enough for certain situations, therefore, the existence of DIA, JDS, ELL, LIL, DOK and many others.

If there are enough interest, the mentioned sparse storages might be added at some point in the future.

## Algebra library

This project isn't and will never be an algebra library because of its own self-contained responsibility and complexity. Futhermore, a good implementation of such library would require a titanic amount of work and research for different algorithms, operations, decompositions, solvers and hardwares.

## Alternatives

One of these libraries might suit you better:

* [`sprs`][sprs]: Sparse linear algebra.
* [`ndarray`][ndarray]: Dense N-dimensional operations.
* [`nalgebra`][nalgebra]: Dense linear algebra.

[nalgebra]: https://github.com/rustsim/nalgebra
[ndarray]: https://github.com/rust-ndarray/ndarray
[sprs]: https://github.com/vbarrielle/sprs