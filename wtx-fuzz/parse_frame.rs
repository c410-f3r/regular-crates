//! Parse

#![allow(
  // False positive
  clippy::redundant_closure
)]
#![cfg_attr(not(feature = "async-std"), feature(async_fn_in_trait))]
#![no_main]

use std::{
  pin::Pin,
  sync::OnceLock,
  task::{Context, Poll},
};
use tokio::{
  io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
  runtime::Handle,
  sync::{oneshot, oneshot::Sender, Mutex},
};
use wtx::{
  web_socket::{FrameBufferVec, WebSocketServer},
  Stream,
};

static FBV: OnceLock<Mutex<FrameBufferVec>> = OnceLock::new();

#[derive(Debug)]
struct ArbitraryByteStream {
  data: Vec<u8>,
  tx: Option<Sender<()>>,
}

impl ArbitraryByteStream {
  fn new(data: Vec<u8>, tx: Sender<()>) -> Self {
    Self { data, tx: Some(tx) }
  }
}

impl AsyncRead for ArbitraryByteStream {
  fn poll_read(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<std::io::Result<()>> {
    let this = self.get_mut();
    let len = buf.remaining().min(this.data.len());
    let data = this.data.drain(..len).collect::<Vec<_>>();
    buf.put_slice(&data);
    if this.data.is_empty() {
      if let Some(tx) = this.tx.take() {
        tx.send(()).map_err(|_err| std::io::Error::from(std::io::ErrorKind::ConnectionRefused))?;
      }
      return Poll::Pending;
    }
    Poll::Ready(Ok(()))
  }
}

impl AsyncWrite for ArbitraryByteStream {
  fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }

  fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
    Poll::Ready(Ok(()))
  }

  fn poll_write(
    self: Pin<&mut Self>,
    _: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<std::io::Result<usize>> {
    Poll::Ready(Ok(buf.len()))
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl Stream for ArbitraryByteStream {
  async fn read(&mut self, bytes: &mut [u8]) -> wtx::Result<usize> {
    Ok(<Self as AsyncReadExt>::read(self, bytes).await?)
  }

  async fn write_all(&mut self, bytes: &[u8]) -> wtx::Result<()> {
    <Self as AsyncWriteExt>::write_all(self, bytes).await?;
    Ok(())
  }
}

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
  {
    let (tx, rx) = oneshot::channel();
    let stream = ArbitraryByteStream::new(data.to_vec(), tx);
    let mut ws = WebSocketServer::new(stream);
    ws.set_max_payload_size(u16::MAX.into());
    Handle::current().block_on(async move {
      let mut lock = FBV.get_or_init(|| <_>::default()).lock().await;
      tokio::select! {
        _ = rx => {}
        _ = ws.read_frame(&mut lock) => {}
      }
    });
  };

  {
    let (tx, rx) = oneshot::channel();
    let stream = ArbitraryByteStream::new(data.to_vec(), tx);
    let mut ws = WebSocketServer::new(stream);
    ws.set_max_payload_size(u16::MAX.into());
    Handle::current().block_on(async move {
      let mut lock = FBV.get_or_init(|| <_>::default()).lock().await;
      tokio::select! {
        _ = rx => {}
        _ = ws.read_msg(&mut lock) => {}
      }
    });
  };
});
