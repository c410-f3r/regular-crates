//! WebSocket TLS server using hyper

use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::sync::{Arc, OnceLock};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_rustls::{
  rustls::{Certificate, PrivateKey, ServerConfig},
  TlsAcceptor,
};
use wtx::web_socket::{
  FrameBufferVec, OpCode, UpgradeFutHyper, WebSocketUpgrade, WebSocketUpgradeHyper,
};

static CERT: &[u8] = include_bytes!("./localhost.crt");
static FBV: OnceLock<Mutex<FrameBufferVec>> = OnceLock::new();
static KEY: &[u8] = include_bytes!("./localhost.key");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
  let acceptor = tls_acceptor()?;
  let listener = TcpListener::bind("127.0.0.1:8080").await.map_err(wtx::Error::from)?;
  loop {
    let (stream, _) = listener.accept().await.map_err(wtx::Error::from)?;
    let acceptor = acceptor.clone();
    let _jh = tokio::spawn(async move {
      let fun = || async {
        let stream = acceptor.accept(stream).await?;
        let uc = Http::new().serve_connection(stream, service_fn(server_upgrade)).with_upgrades();
        wtx::Result::Ok(uc)
      };
      if let Err(err) = fun().await {
        println!("An error occurred: {err}");
      }
    });
  }
}

/// Error
#[derive(Debug)]
pub enum Error {
  /// TokioRustls
  TokioRustls(tokio_rustls::rustls::Error),
  /// Wtx
  Wtx(wtx::Error),
}

impl From<tokio_rustls::rustls::Error> for Error {
  fn from(from: tokio_rustls::rustls::Error) -> Self {
    Self::TokioRustls(from)
  }
}

impl From<wtx::Error> for Error {
  fn from(from: wtx::Error) -> Self {
    Self::Wtx(from)
  }
}

async fn handle_frames(fut: UpgradeFutHyper) -> wtx::Result<()> {
  let mut ws = fut.await?;
  loop {
    let mut lock = FBV.get_or_init(|| <_>::default()).lock().await;
    let frame = ws.read_frame(&mut lock).await?;
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
  let _jh = tokio::spawn(handle_frames(fut));
  Ok(res)
}

fn tls_acceptor() -> Result<TlsAcceptor, Error> {
  let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut &*KEY)
    .map(|certs| certs.into_iter().map(PrivateKey).collect())
    .map_err(wtx::Error::from)?;
  let certs = certs(&mut &*CERT)
    .map(|certs| certs.into_iter().map(Certificate).collect())
    .map_err(wtx::Error::from)?;
  let config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(certs, keys.remove(0))?;
  Ok(TlsAcceptor::from(Arc::new(config)))
}
