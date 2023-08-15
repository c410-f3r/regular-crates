use crate::web_socket::{
  http::utils::{gen_key, sec_websocket_protocol, trim},
  WebSocketClient, WebSocketHandshake, WebSocketServer, WebSocketUpgrade,
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use core::{
  future::Future,
  marker::PhantomData,
  pin::{pin, Pin},
  task::{ready, Context, Poll},
};
use hyper::{
  client::conn::{self, Connection},
  header::{AsHeaderName, CONNECTION, HOST, UPGRADE},
  http::HeaderValue,
  rt::Executor,
  upgrade::{self, OnUpgrade, Upgraded},
  Body, HeaderMap, Request, Response, StatusCode,
};
use tokio::io::{AsyncRead, AsyncWrite};

/// A future that resolves to a WebSocket stream when the associated HTTP upgrade completes.
#[derive(Debug)]
pub struct UpgradeFutHyper {
  inner: OnUpgrade,
}

impl Future for UpgradeFutHyper {
  type Output = crate::Result<WebSocketServer<Upgraded>>;

  #[inline]
  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let stream = ready!(pin!(&mut self.inner).poll(cx))?;
    Poll::Ready(Ok(WebSocketServer::new(stream)))
  }
}

/// Marker used to implement [WebSocketHandshake].
#[derive(Debug)]
pub struct WebSocketHandshakeHyper<'executor, E, S>(PhantomData<(&'executor (), E, S)>);

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<'executor, E, S> WebSocketHandshake for WebSocketHandshakeHyper<'executor, E, S>
where
  E: Executor<Connection<S, Body>> + Send + Sync + 'executor,
  S: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
{
  type HandshakeInput = (&'executor E, Request<Body>, S);
  type Response = Response<Body>;
  type Stream = Upgraded;

  #[allow(
    // All strings are valid header values
    clippy::unwrap_used
  )]
  #[inline]
  async fn handshake(
    &self,
    (executor, mut req, socket): Self::HandshakeInput,
  ) -> crate::Result<(Self::Response, WebSocketClient<Self::Stream>)> {
    let host = req.uri().authority().ok_or(crate::Error::NoAuthority)?.as_str().parse().unwrap();
    let swk = gen_key(&mut <_>::default()).parse().unwrap();
    drop(req.headers_mut().insert(CONNECTION, HeaderValue::from_static("upgrade")));
    drop(req.headers_mut().insert(HOST, host));
    drop(req.headers_mut().insert("Sec-WebSocket-Key", swk));
    drop(req.headers_mut().insert("Sec-WebSocket-Version", HeaderValue::from_static("13")));
    drop(req.headers_mut().insert(UPGRADE, HeaderValue::from_static("websocket")));
    let (mut sender, conn) = conn::handshake(socket).await?;
    executor.execute(conn);
    let mut res = sender.send_request(req).await?;
    verify(&res)?;
    match upgrade::on(&mut res).await {
      Err(err) => Err(err.into()),
      Ok(elem) => Ok((res, WebSocketClient::new(elem))),
    }
  }
}

impl<E, S> Default for WebSocketHandshakeHyper<'_, E, S> {
  #[inline]
  fn default() -> Self {
    Self(PhantomData)
  }
}

/// Marker used to implement [WebSocketUpgrade].
#[derive(Debug)]
pub struct WebSocketUpgradeHyper<RB>(PhantomData<RB>);

#[allow(
  // False positive
  clippy::unused_async
)]
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<RB> WebSocketUpgrade for WebSocketUpgradeHyper<RB>
where
  RB: Send + Sync,
{
  type Request = Request<RB>;
  type Response = Response<Body>;
  type Stream = Upgraded;
  type Upgrade = UpgradeFutHyper;

  #[inline]
  fn is_upgrade_request(&self, req: &Self::Request) -> bool {
    header_contains_value(&CONNECTION, req.headers(), "Upgrade")
      && header_contains_value(&UPGRADE, req.headers(), "websocket")
  }

  #[inline]
  async fn upgrade(
    &self,
    req: &mut Self::Request,
  ) -> crate::Result<(Self::Response, Self::Upgrade)> {
    let key = req.headers().get("Sec-WebSocket-Key").ok_or(crate::Error::MissingSecWebSocketKey)?;
    if req.headers().get("Sec-WebSocket-Version").map(HeaderValue::as_bytes) != Some(b"13") {
      return Err(crate::Error::InvalidSecWebsocketVersion);
    }
    let mut swa_buffer = <_>::default();
    let res = Response::builder()
      .status(StatusCode::SWITCHING_PROTOCOLS)
      .header(CONNECTION, "upgrade")
      .header(UPGRADE, "websocket")
      .header("Sec-WebSocket-Accept", sec_websocket_protocol(&mut swa_buffer, key.as_bytes()))
      .body(Body::from("switching to websocket protocol"))?;
    let stream = UpgradeFutHyper { inner: upgrade::on(req) };
    Ok((res, stream))
  }
}

impl<RB> Default for WebSocketUpgradeHyper<RB> {
  #[inline]
  fn default() -> Self {
    Self(PhantomData)
  }
}

/// Check if there is a header of the given name containing the wanted value.
fn header_contains_value(
  header: impl AsHeaderName,
  headers: &HeaderMap,
  value: impl AsRef<[u8]>,
) -> bool {
  let bytes = value.as_ref();
  for elem in headers.get_all(header) {
    if elem.as_bytes().split(|&c| c == b',').any(|x| trim(x).eq_ignore_ascii_case(bytes)) {
      return true;
    }
  }
  false
}

fn verify(res: &Response<Body>) -> crate::Result<()> {
  if res.status() != StatusCode::SWITCHING_PROTOCOLS {
    return Err(crate::Error::MissingSwitchingProtocols);
  }
  if !res
    .headers()
    .get("Upgrade")
    .and_then(|h| h.to_str().ok())
    .map_or(false, |h| h.eq_ignore_ascii_case("websocket"))
  {
    return Err(crate::Error::MissingUpgradeHeader);
  }
  if !res
    .headers()
    .get("Connection")
    .and_then(|h| h.to_str().ok())
    .map_or(false, |h| h.eq_ignore_ascii_case("Upgrade"))
  {
    return Err(crate::Error::InvalidConnectionHeader);
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  macro_rules! call_tests {
    (($ty:ident, $fb:expr, $ws:expr), $($struct:ident),+ $(,)?) => {
      $(
        println!("***** {} - {}", stringify!($ty), stringify!($struct));
        $struct::$ty($fb, $ws).await;
        sleep(Duration::from_millis(200)).await;
      )+
    };
  }

  use crate::web_socket::{
    frame::FrameMut, Frame, FrameBufferVec, OpCode, WebSocketClient, WebSocketHandshake,
    WebSocketHandshakeHyper, WebSocketServer, WebSocketUpgrade, WebSocketUpgradeHyper,
  };
  use core::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
  };
  use hyper::{
    rt::Executor,
    server::Server,
    service::{make_service_fn, service_fn},
    upgrade::Upgraded,
    Body, Request, Response,
  };
  use std::net::{Ipv6Addr, TcpListener};
  use tokio::{net::TcpStream, time::sleep};

  static HAS_SERVER_FINISHED: AtomicBool = AtomicBool::new(false);

  #[tokio::test]
  async fn client_and_server_frames() {
    async fn server(mut request: Request<Body>) -> crate::Result<Response<Body>> {
      assert_eq!(true, WebSocketUpgradeHyper::default().is_upgrade_request(&request));
      let (res, stream) = WebSocketUpgradeHyper::default().upgrade(&mut request).await.unwrap();
      let _jh = tokio::spawn(async move {
        let mut fb = FrameBufferVec::with_capacity(16 * 1024 * 1024);
        let mut ws = stream.await.unwrap();
        call_tests!(
          (server, &mut fb, &mut ws),
          LargeFragmentedMessage,
          SeveralBytes,
          FragmentedMessage,
          TwoPings,
          PingAndText,
          HelloAndGoodbye
        );
        HAS_SERVER_FINISHED.store(true, Ordering::Relaxed);
      });
      Ok(res)
    }

    let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, 0u16)).unwrap();
    let bind_addr = listener.local_addr().unwrap();
    let _jh = tokio::spawn(async move {
      let svc = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(server)) });
      Server::from_tcp(listener).unwrap().http1_only(true).serve(svc).await.unwrap();
    });
    let (_res, mut ws) = WebSocketHandshakeHyper::default()
      .handshake((
        &TestExecutor,
        Request::get("ws://localhost/foo").body(Body::empty()).unwrap(),
        TcpStream::connect(bind_addr).await.unwrap(),
      ))
      .await
      .unwrap();
    let mut fb = FrameBufferVec::with_capacity(16 * 1024 * 1024);
    call_tests!(
      (client, &mut fb, &mut ws),
      LargeFragmentedMessage,
      SeveralBytes,
      FragmentedMessage,
      TwoPings,
      PingAndText,
      HelloAndGoodbye
    );
    let mut has_server_finished = false;
    for _ in 0..15 {
      let local_has_server_finished = HAS_SERVER_FINISHED.load(Ordering::Relaxed);
      if local_has_server_finished {
        has_server_finished = local_has_server_finished;
        break;
      }
      sleep(Duration::from_millis(200)).await;
    }
    if !has_server_finished {
      panic!("Server didn't finish");
    }
  }

  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  trait Test {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>);

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>);
  }

  struct LargeFragmentedMessage;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for LargeFragmentedMessage {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      async fn write(frame: FrameMut<'_, true>, ws: &mut WebSocketClient<Upgraded>) {
        ws.write_frame(frame).await.unwrap();
      }
      let bytes = vec![51; 1024 * 1024];
      write(Frame::new_unfin(fb.into(), OpCode::Text, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
      write(Frame::new_fin(fb.into(), OpCode::Continuation, &bytes).unwrap(), ws).await;
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      let text = ws.read_msg(fb).await.unwrap();
      assert_eq!(OpCode::Text, text.op_code());
      assert_eq!(&vec![51; 10 * 1024 * 1024], text.fb().payload());
    }
  }

  struct SeveralBytes;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for SeveralBytes {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      async fn write(frame: FrameMut<'_, true>, ws: &mut WebSocketClient<Upgraded>) {
        ws.write_frame(frame).await.unwrap();
      }
      write(Frame::new_unfin(fb.into(), OpCode::Text, &[206]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[186]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[225]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[189]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[185]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[207]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[131]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[206]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[188]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[206]).unwrap(), ws).await;
      write(Frame::new_unfin(fb.into(), OpCode::Continuation, &[181]).unwrap(), ws).await;
      write(Frame::new_fin(fb.into(), OpCode::Continuation, &[]).unwrap(), ws).await;
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      let text = ws.read_msg(fb).await.unwrap();
      assert_eq!(OpCode::Text, text.op_code());
      assert_eq!("κόσμε".as_bytes(), text.fb().payload());
    }
  }

  struct FragmentedMessage;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for FragmentedMessage {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      ws.write_frame(Frame::new_unfin(fb.into(), OpCode::Text, b"1").unwrap()).await.unwrap();
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Continuation, b"23").unwrap())
        .await
        .unwrap();
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      let text = ws.read_msg(fb).await.unwrap();
      assert_eq!(OpCode::Text, text.op_code());
      assert_eq!(b"123", text.fb().payload());
    }
  }

  struct TwoPings;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for TwoPings {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Ping, b"0").unwrap()).await.unwrap();
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Ping, b"1").unwrap()).await.unwrap();
      let _0 = ws.read_frame(fb).await.unwrap();
      assert_eq!(OpCode::Pong, _0.op_code());
      assert_eq!(b"0", _0.fb().payload());
      let _1 = ws.read_frame(fb).await.unwrap();
      assert_eq!(OpCode::Pong, _1.op_code());
      assert_eq!(b"1", _1.fb().payload());
      let a = Frame::new_fin(fb.into(), OpCode::Text, b"").unwrap();
      ws.write_frame(a).await.unwrap();
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      let _0 = ws.read_frame(fb).await.unwrap();
      assert_eq!(OpCode::Text, _0.op_code());
      assert_eq!(b"", _0.fb().payload());
    }
  }

  struct PingAndText;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for PingAndText {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Ping, b"").unwrap()).await.unwrap();
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Text, b"ipat").unwrap()).await.unwrap();
      assert_eq!(OpCode::Pong, ws.read_frame(fb).await.unwrap().op_code());
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      assert_eq!(b"ipat", ws.read_frame(fb).await.unwrap().fb().payload());
    }
  }

  struct HelloAndGoodbye;
  #[cfg_attr(feature = "async-trait", async_trait::async_trait)]
  impl Test for HelloAndGoodbye {
    async fn client(fb: &mut FrameBufferVec, ws: &mut WebSocketClient<Upgraded>) {
      let hello = ws.read_frame(fb).await.unwrap();
      assert_eq!(OpCode::Text, hello.op_code());
      assert_eq!(b"Hello!", hello.fb().payload());
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Text, b"Goodbye!").unwrap()).await.unwrap();
      assert_eq!(OpCode::Close, ws.read_frame(fb).await.unwrap().op_code());
    }

    async fn server(fb: &mut FrameBufferVec, ws: &mut WebSocketServer<Upgraded>) {
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Text, b"Hello!").unwrap()).await.unwrap();
      assert_eq!(ws.read_frame(&mut *fb).await.unwrap().fb().payload(), b"Goodbye!");
      ws.write_frame(Frame::new_fin(fb.into(), OpCode::Close, &[]).unwrap()).await.unwrap();
    }
  }

  struct TestExecutor;

  impl<Fut> Executor<Fut> for TestExecutor
  where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
  {
    fn execute(&self, fut: Fut) {
      let _jh = tokio::spawn(fut);
    }
  }
}
