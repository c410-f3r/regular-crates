#[cfg(feature = "async-trait")]
use alloc::boxed::Box;

/// A stream of values produced asynchronously.
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait Stream {
  /// Pulls some bytes from this source into the specified buffer, returning how many bytes
  /// were read.
  async fn read(&mut self, bytes: &mut [u8]) -> crate::Result<usize>;

  /// Attempts to write all elements of `bytes`.
  async fn write_all(&mut self, bytes: &[u8]) -> crate::Result<()>;
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<T> Stream for &mut T
where
  T: Send + Stream + Sync,
{
  #[inline]
  async fn read(&mut self, bytes: &mut [u8]) -> crate::Result<usize> {
    (*self).read(bytes).await
  }

  #[inline]
  async fn write_all(&mut self, bytes: &[u8]) -> crate::Result<()> {
    (*self).write_all(bytes).await
  }
}

/// Does nothing.
#[derive(Debug)]
pub struct DummyStream;

#[allow(
  // False positive
  clippy::unused_async
)]
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl Stream for DummyStream {
  #[inline]
  async fn read(&mut self, _: &mut [u8]) -> crate::Result<usize> {
    Ok(0)
  }

  #[inline]
  async fn write_all(&mut self, _: &[u8]) -> crate::Result<()> {
    Ok(())
  }
}

#[cfg(feature = "http-client")]
mod http_client {
  use crate::Stream;
  #[cfg(feature = "async-trait")]
  use alloc::boxed::Box;
  use async_std::io::{ReadExt, WriteExt};
  use http_types::upgrade::Connection;

  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Stream for Connection {
    #[inline]
    async fn read(&mut self, bytes: &mut [u8]) -> crate::Result<usize> {
      Ok(<Self as ReadExt>::read(self, bytes).await?)
    }

    #[inline]
    async fn write_all(&mut self, bytes: &[u8]) -> crate::Result<()> {
      <Self as WriteExt>::write_all(self, bytes).await?;
      Ok(())
    }
  }
}

#[cfg(feature = "hyper")]
mod hyper {
  use crate::Stream;
  #[cfg(feature = "async-trait")]
  use alloc::boxed::Box;
  use hyper::upgrade::Upgraded;
  use tokio::io::{AsyncReadExt, AsyncWriteExt};

  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Stream for Upgraded {
    #[inline]
    async fn read(&mut self, bytes: &mut [u8]) -> crate::Result<usize> {
      Ok(<Self as AsyncReadExt>::read(self, bytes).await?)
    }

    #[inline]
    async fn write_all(&mut self, bytes: &[u8]) -> crate::Result<()> {
      <Self as AsyncWriteExt>::write_all(self, bytes).await?;
      Ok(())
    }
  }
}
