//! WebSocket Authbahn client using hyper

use hyper::{rt::Executor, upgrade::Upgraded, Body, Request};
use std::{future::Future, str};
use tokio::net::TcpStream;
use wtx::web_socket::{
  Frame, FrameBufferVec, OpCode, WebSocketClient, WebSocketHandshake, WebSocketHandshakeHyper,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
  let mut fb = &mut FrameBufferVec::default();
  let count = get_case_count(&mut fb).await?;

  for case in 1..=count {
    let mut ws = connect(&format!("runCase?case={case}&agent=wtx")).await?;
    loop {
      let frame = match ws.read_msg(&mut fb).await {
        Err(err) => {
          println!("Error: {err}");
          ws.write_frame(Frame::new_fin(fb.into(), OpCode::Close, &[])?).await?;
          break;
        }
        Ok(elem) => elem,
      };
      match frame.op_code() {
        OpCode::Binary | OpCode::Text => ws.write_frame(frame).await?,
        OpCode::Close => break,
        _ => {}
      }
    }
  }

  let mut ws = connect("updateReports?agent=wtx").await?;
  ws.write_frame(Frame::close_from_params(1000, fb.into(), &[])?).await?;
  Ok(())
}

/// Error
#[derive(Debug)]
pub enum Error {
  /// ParseIntError
  ParseIntError(std::num::ParseIntError),
  /// Wtx
  Wtx(wtx::Error),
}

impl From<std::num::ParseIntError> for Error {
  fn from(from: std::num::ParseIntError) -> Self {
    Self::ParseIntError(from)
  }
}

impl From<wtx::Error> for Error {
  fn from(from: wtx::Error) -> Self {
    Self::Wtx(from)
  }
}

struct SpawnExecutor;

impl<Fut> Executor<Fut> for SpawnExecutor
where
  Fut: Future + Send + 'static,
  Fut::Output: Send + 'static,
{
  fn execute(&self, fut: Fut) {
    let _jh = tokio::task::spawn(fut);
  }
}

async fn connect(path: &str) -> Result<WebSocketClient<Upgraded>, Error> {
  let addr = "localhost:9080";
  let stream = TcpStream::connect(addr).await.map_err(wtx::Error::from)?;
  let uri = format!("http://{addr}/{path}");
  let req = Request::get(uri).body(Body::empty()).map_err(wtx::Error::from)?;
  Ok(WebSocketHandshakeHyper::default().handshake((&SpawnExecutor, req, stream)).await?.1)
}

async fn get_case_count(fb: &mut FrameBufferVec) -> Result<u32, Error> {
  let mut ws = connect("getCaseCount").await?;
  let frame = ws.read_msg(fb).await?;
  let rslt = str::from_utf8(frame.fb().payload()).map_err(wtx::Error::from)?.parse()?;
  ws.write_frame(Frame::close_from_params(1000, fb.into(), &[])?).await?;
  Ok(rslt)
}
