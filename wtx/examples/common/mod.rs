use std::borrow::BorrowMut;
use wtx::{
  web_socket::{
    handshake::{WebSocketAccept, WebSocketAcceptRaw, WebSocketHandshake, WebSocketHandshakeRaw},
    FrameBufferVec, OpCode, WebSocketClient, WebSocketServer,
  },
  ReadBuffer, Stream,
};

pub(crate) async fn _accept_conn_and_echo_frames(
  fb: &mut FrameBufferVec,
  rb: &mut ReadBuffer,
  stream: impl Send + Stream + Sync,
) -> wtx::Result<()> {
  let (_, mut ws) = WebSocketAcceptRaw {
    fb,
    headers_buffer: &mut <_>::default(),
    key_buffer: &mut <_>::default(),
    rb,
    stream,
  }
  .accept()
  .await?;
  _handle_frames(fb, &mut ws).await?;
  Ok(())
}

pub(crate) async fn _connect<RB, S>(
  fb: &mut FrameBufferVec,
  uri: &str,
  rb: RB,
  stream: S,
) -> wtx::Result<WebSocketClient<RB, S>>
where
  RB: BorrowMut<ReadBuffer> + Send + Sync,
  S: Send + Stream + Sync,
{
  Ok(
    WebSocketHandshakeRaw { fb, headers_buffer: &mut <_>::default(), rb, uri, stream }
      .handshake()
      .await?
      .1,
  )
}

pub(crate) async fn _handle_frames<RB>(
  fb: &mut FrameBufferVec,
  ws: &mut WebSocketServer<RB, impl Stream>,
) -> wtx::Result<()>
where
  RB: BorrowMut<ReadBuffer>,
{
  loop {
    let mut frame = ws.read_msg(fb).await?;
    match frame.op_code() {
      OpCode::Binary | OpCode::Text => {
        ws.write_frame(&mut frame).await?;
      }
      OpCode::Close => break,
      _ => {}
    }
  }
  Ok(())
}

pub(crate) fn _host_from_args() -> String {
  std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_owned())
}

pub(crate) fn _uri_from_args() -> String {
  std::env::args().nth(1).unwrap_or_else(|| "http://127.0.0.1:8080".to_owned())
}
