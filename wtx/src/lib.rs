#![cfg_attr(
  not(feature = "async-trait"),
  feature(array_chunks, async_fn_in_trait, impl_trait_projections, inline_const)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod cache;
mod error;
mod header;
mod misc;
mod read_buffer;
#[cfg(feature = "web-socket-handshake")]
mod request;
#[cfg(feature = "web-socket-handshake")]
mod response;
mod stream;
pub mod web_socket;
mod wrapper;

pub use crate::stream::{BytesStream, DummyStream, Stream};
pub use cache::Cache;
pub use error::Error;
pub use header::ExpectedHeader;
pub use misc::UriParts;
pub use read_buffer::ReadBuffer;
#[cfg(feature = "web-socket-handshake")]
pub use request::Request;
#[cfg(feature = "web-socket-handshake")]
pub use response::Response;
pub use wrapper::Wrapper;

/// Shortcut of [core::result::Result<T, Error>].
pub type Result<T> = core::result::Result<T, Error>;
