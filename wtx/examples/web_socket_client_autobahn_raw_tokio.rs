//! WebSocket autobahn client.

mod common;

use tokio::net::TcpStream;
use wtx::{
  web_socket::{FrameBufferVec, FrameVecMut, OpCode},
  ReadBuffer,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let host = &common::_host_from_args();
  let mut fb = &mut <_>::default();
  let mut rb = <_>::default();
  for case in 1..=get_case_count(&mut fb, &host, &mut rb).await? {
    let mut ws = common::_connect(
      fb,
      &format!("http://{host}/runCase?case={case}&agent=wtx"),
      &mut rb,
      TcpStream::connect(host).await.map_err(wtx::Error::from)?,
    )
    .await?;
    loop {
      let frame = match ws.read_msg(&mut fb).await {
        Err(err) => {
          println!("Error: {err}");
          ws.write_frame(FrameVecMut::new_fin(fb.into(), OpCode::Close, &[])?).await?;
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
  common::_connect(
    fb,
    &format!("http://{host}/updateReports?agent=wtx"),
    &mut rb,
    TcpStream::connect(host).await.map_err(wtx::Error::from)?,
  )
  .await?
  .write_frame(FrameVecMut::close_from_params(1000, fb.into(), &[])?)
  .await?;
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

async fn get_case_count(
  fb: &mut FrameBufferVec,
  host: &str,
  rb: &mut ReadBuffer,
) -> Result<u32, Error> {
  let mut ws = common::_connect(
    fb,
    &format!("http://{host}/getCaseCount"),
    rb,
    TcpStream::connect(host).await.map_err(wtx::Error::from)?,
  )
  .await?;
  let rslt = ws.read_msg(fb).await?.text_payload().unwrap_or_default().parse()?;
  ws.write_frame(FrameVecMut::close_from_params(1000, fb.into(), &[])?).await?;
  Ok(rslt)
}
