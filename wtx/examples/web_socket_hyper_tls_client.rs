//! WebSocket TLS client using hyper

use hyper::{rt::Executor, upgrade::Upgraded, Body, Request};
use std::{future::Future, io::ErrorKind, sync::Arc};
use tokio::net::TcpStream;
use tokio_rustls::{
  rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName},
  TlsConnector,
};
use webpki_roots::TLS_SERVER_ROOTS;
use wtx::web_socket::{
  Frame, FrameBufferVec, OpCode, WebSocketClient, WebSocketHandshake, WebSocketHandshakeHyper,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> wtx::Result<()> {
  let mut fb = FrameBufferVec::default();
  let mut ws = connect().await?;
  loop {
    let frame = match ws.read_msg(&mut fb).await {
      Err(err) => {
        println!("Error: {err}");
        ws.write_frame(Frame::new_fin(fb, OpCode::Close, &[])?).await?;
        break;
      }
      Ok(elem) => elem,
    };
    match (frame.op_code(), frame.text_payload()) {
      (_, Some(elem)) => println!("{elem}"),
      (OpCode::Close, _) => break,
      _ => {}
    }
  }
  Ok(())
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

async fn connect() -> wtx::Result<WebSocketClient<Upgraded>> {
  let domain = "stream.binance.com";
  let addr = "stream.binance.com:9443";
  let uri = "wss://stream.binance.com:9443/ws/btcusdt@bookTicker";

  let tcp_stream = TcpStream::connect(addr).await?;
  let tls_connector = tls_connector();
  let map_err = |_err| std::io::Error::new(ErrorKind::InvalidInput, "invalid dnsname");
  let domain: ServerName = ServerName::try_from(domain).map_err(map_err)?;
  let tls_stream = tls_connector.connect(domain, tcp_stream).await?;
  let req = Request::get(uri).body(Body::empty())?;
  Ok(WebSocketHandshakeHyper::default().handshake((&SpawnExecutor, req, tls_stream)).await?.1)
}

fn tls_connector() -> TlsConnector {
  let mut root_store = RootCertStore::empty();
  root_store.add_trust_anchors(TLS_SERVER_ROOTS.0.iter().map(|ta| {
    OwnedTrustAnchor::from_subject_spki_name_constraints(ta.subject, ta.spki, ta.name_constraints)
  }));
  let config = ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();
  TlsConnector::from(Arc::new(config))
}
