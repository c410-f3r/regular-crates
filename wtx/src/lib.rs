#![cfg_attr(
  any(feature = "nightly", not(feature = "async-trait")),
  feature(array_chunks, async_fn_in_trait, impl_trait_projections, inline_const)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod error;
mod misc;
mod stream;
pub mod web_socket;

pub use error::Error;

/// Shortcut of [core::result::Result<T, Error>].
pub type Result<T> = core::result::Result<T, Error>;
