#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

mod structures;
mod traits;
mod tuple_impls;

pub use structures::*;
pub use traits::*;

/// Alias of [`core::result::Result<T, Error>`].
pub type Result<T> = core::result::Result<T, Error>;
