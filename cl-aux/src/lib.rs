//! Auxiliary elements for collections
//!
//! This crate provides a single method for each `trait` to achieve maximum flexibility and
//! freedom instead of imposing an abstraction subset for all situations and users.

#![cfg_attr(feature = "gat", feature(generic_associated_types))]
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

/// Alias of core::result::Result<T, Error>
pub type Result<T> = core::result::Result<T, Error>;
