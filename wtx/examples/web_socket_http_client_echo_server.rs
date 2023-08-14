//! WebSocket echo server using hyper

use async_std::{
  net::{TcpListener, TcpStream},
  stream::StreamExt,
  sync::Mutex,
};
use std::sync::OnceLock;
use wtx::web_socket::{
  FrameBufferVec, OpCode, UpgradeFutHttpClient, WebSocketUpgrade, WebSocketUpgradeHttpClient,
};

static FBV: OnceLock<Mutex<FrameBufferVec>> = OnceLock::new();

#[tokio::main(flavor = "current_thread")]
async fn main() -> wtx::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  loop {
    let mut incoming = listener.incoming();
    while let Some(stream_rslt) = incoming.next().await {
      let stream = stream_rslt?;
      let _jh = async_std::task::spawn(async {
        if let Err(err) = server_upgrade(stream).await {
          println!("An error occurred: {err}");
        }
      });
    }
  }
}

async fn handle_frames(fut: UpgradeFutHttpClient) -> wtx::Result<()> {
  let mut ws = fut.await?;
  loop {
    let mut lock = FBV.get_or_init(|| <_>::default()).lock().await;
    let frame = ws.read_msg(&mut lock).await?;
    match frame.op_code() {
      OpCode::Binary | OpCode::Text => {
        ws.write_frame(frame).await?;
      }
      OpCode::Close => break,
      _ => {}
    }
  }
  Ok(())
}

async fn server_upgrade(stream: TcpStream) -> http_types::Result<()> {
  async_h1::accept(stream.clone(), |mut req| async move {
    let (res, fut) = WebSocketUpgradeHttpClient::default().upgrade(&mut req).await?;
    if let Err(err) = handle_frames(fut).await {
      eprintln!("Error in WebSocket connection: {err}");
    }
    Ok(res)
  })
  .await?;
  Ok(())
}
