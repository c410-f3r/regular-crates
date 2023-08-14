//! WebSocket echo server using hyper

use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::sync::OnceLock;
use tokio::{net::TcpListener, sync::Mutex, task};
use wtx::web_socket::{
  FrameBufferVec, OpCode, UpgradeFutHyper, WebSocketUpgrade, WebSocketUpgradeHyper,
};

static FBV: OnceLock<Mutex<FrameBufferVec>> = OnceLock::new();

#[tokio::main(flavor = "current_thread")]
async fn main() -> wtx::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  loop {
    let (stream, _) = listener.accept().await?;
    let _jh = tokio::spawn(async move {
      let uc = Http::new().serve_connection(stream, service_fn(server_upgrade)).with_upgrades();
      if let Err(err) = uc.await {
        println!("An error occurred: {err}");
      }
    });
  }
}

async fn handle_frames(fut: UpgradeFutHyper) -> wtx::Result<()> {
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

async fn server_upgrade(mut req: Request<Body>) -> wtx::Result<Response<Body>> {
  let (res, fut) = WebSocketUpgradeHyper::default().upgrade(&mut req).await?;
  let _jh = task::spawn(async move {
    if let Err(err) = task::unconstrained(handle_frames(fut)).await {
      eprintln!("Error in WebSocket connection: {err}");
    }
  });
  Ok(res)
}
