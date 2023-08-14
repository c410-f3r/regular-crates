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
use http_client::HttpClient;
use http_types::{
  headers::{HeaderName, CONNECTION, HOST, UPGRADE},
  upgrade::{Connection, Receiver},
  Headers, Request, Response, StatusCode,
};

/// A future that resolves to a WebSocket stream when the associated HTTP upgrade completes.
#[derive(Debug)]
pub struct UpgradeFutHttpClient {
  inner: Receiver,
}

impl Future for UpgradeFutHttpClient {
  type Output = crate::Result<WebSocketServer<Connection>>;

  #[inline]
  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let opt = ready!(pin!(&mut self.inner).poll(cx));
    let Some(stream) = opt else { return Poll::Ready(Err(crate::Error::NoUpgradeConnection)); };
    Poll::Ready(Ok(WebSocketServer::new(stream)))
  }
}

/// Marker used to implement [WebSocketHandshake].
#[derive(Debug)]
pub struct WebSocketHandshakeHttpClient<'client, C>(PhantomData<(&'client (), C)>);

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<'client, C> WebSocketHandshake for WebSocketHandshakeHttpClient<'client, C>
where
  C: HttpClient,
{
  type HandshakeInput = (&'client C, Request);
  type Response = Response;
  type Stream = Connection;

  #[inline]
  async fn handshake(
    &self,
    (client, mut req): Self::HandshakeInput,
  ) -> crate::Result<(Self::Response, WebSocketClient<Self::Stream>)> {
    let host = req.url().authority().to_owned();
    drop(req.insert_header(CONNECTION, "upgrade"));
    drop(req.insert_header(HOST, host));
    drop(req.insert_header("Sec-WebSocket-Key", gen_key(&mut <_>::default())));
    drop(req.insert_header("Sec-WebSocket-Version", "13"));
    drop(req.insert_header(UPGRADE, "websocket"));
    let mut res = client.send(req).await?;
    verify(&res)?;
    match res.recv_upgrade().await.await {
      None => Err(crate::Error::NoUpgradeConnection),
      Some(elem) => Ok((res, WebSocketClient::new(elem))),
    }
  }
}

impl<C> Default for WebSocketHandshakeHttpClient<'_, C> {
  #[inline]
  fn default() -> Self {
    Self(PhantomData)
  }
}

/// Marker used to implement [WebSocketUpgrade].
#[derive(Debug, Default)]
pub struct WebSocketUpgradeHttpClient;

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl WebSocketUpgrade for WebSocketUpgradeHttpClient {
  type Request = Request;
  type Response = Response;
  type Stream = Connection;
  type Upgrade = UpgradeFutHttpClient;

  #[inline]
  fn is_upgrade_request(&self, req: &Self::Request) -> bool {
    header_contains_value(&CONNECTION, req.as_ref(), "Upgrade")
      && header_contains_value(&UPGRADE, req.as_ref(), "websocket")
  }

  #[inline]
  async fn upgrade(
    &self,
    req: &mut Self::Request,
  ) -> crate::Result<(Self::Response, Self::Upgrade)> {
    let key = req.header("Sec-WebSocket-Key").ok_or(crate::Error::MissingSecWebSocketKey)?;
    if req.header("Sec-WebSocket-Version").map(|el| el.as_str().as_bytes()) != Some(b"13") {
      return Err(crate::Error::InvalidSecWebsocketVersion);
    }
    let mut swa_buffer = <_>::default();
    let mut res = Response::new(StatusCode::SwitchingProtocols);
    res.append_header(CONNECTION, "upgrade");
    res.append_header(UPGRADE, "websocket");
    res.append_header(
      "Sec-WebSocket-Accept",
      sec_websocket_protocol(&mut swa_buffer, key.as_str().as_bytes()),
    );
    res.set_body("switching to websocket protocol");
    let stream = UpgradeFutHttpClient { inner: res.recv_upgrade().await };
    Ok((res, stream))
  }
}

/// Check if there is a header of the given name containing the wanted value.
fn header_contains_value(header: &HeaderName, headers: &Headers, value: impl AsRef<[u8]>) -> bool {
  let bytes = value.as_ref();
  for (_, hv) in headers.iter().filter(|(key, _)| *key == header) {
    if hv.as_str().as_bytes().split(|&c| c == b',').any(|x| trim(x).eq_ignore_ascii_case(bytes)) {
      return true;
    }
  }
  false
}

fn verify(res: &Response) -> crate::Result<()> {
  if res.status() != StatusCode::SwitchingProtocols {
    return Err(crate::Error::MissingSwitchingProtocols);
  }
  if !res.header("Upgrade").map_or(false, |h| h.as_str().eq_ignore_ascii_case("websocket")) {
    return Err(crate::Error::MissingUpgradeHeader);
  }
  if !res.header("Connection").map_or(false, |h| h.as_str().eq_ignore_ascii_case("Upgrade")) {
    return Err(crate::Error::InvalidConnectionHeader);
  }
  Ok(())
}
