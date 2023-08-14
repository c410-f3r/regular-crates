#[cfg(feature = "http-client")]
pub(super) mod http_client;
#[cfg(feature = "hyper")]
pub(super) mod hyper;

use crate::web_socket::{Stream, WebSocketClient, WebSocketServer};
use core::future::Future;

/// Initial negotiation between two actors to start exchange frames.
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait WebSocketHandshake {
  /// Specific handshake input
  type HandshakeInput;
  /// Specific implementation response
  type Response;
  /// Specific implementation stream
  type Stream: Stream;

  /// Performs the client handshake.
  async fn handshake(
    &self,
    input: Self::HandshakeInput,
  ) -> crate::Result<(Self::Response, WebSocketClient<Self::Stream>)>;
}

/// Manages the upgrade of requests into WebSocket connections.
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait WebSocketUpgrade {
  /// Specific implementation request
  type Request;
  /// Specific implementation response
  type Response;
  /// Specific implementation stream
  type Stream: Stream;
  /// Specific implementation future that resolves to [WebSocket]
  type Upgrade: Future<Output = crate::Result<WebSocketServer<Self::Stream>>>;

  /// Checks if `request` is a WebSocket upgrade request.
  fn is_upgrade_request(&self, request: &Self::Request) -> bool;

  /// Try to upgrade a received request to a WebSocket connection.
  async fn upgrade(
    &self,
    input: &mut Self::Request,
  ) -> crate::Result<(Self::Response, Self::Upgrade)>;
}

#[cfg(any(feature = "http-client", feature = "hyper"))]
pub(crate) mod utils {
  use crate::misc::{from_utf8_opt, Rng};
  use base64::{engine::general_purpose::STANDARD, Engine};
  use sha1::{Digest, Sha1};

  pub(crate) fn gen_key(buffer: &mut [u8; 26]) -> &str {
    base64_from_array(&Rng::default()._random_u8_16(), buffer)
  }

  pub(crate) fn sec_websocket_protocol<'buffer>(
    buffer: &'buffer mut [u8; 30],
    key: &[u8],
  ) -> &'buffer str {
    let mut sha1 = Sha1::new();
    sha1.update(key);
    sha1.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"); // magic string
    base64_from_array(&sha1.finalize().into(), buffer)
  }

  pub(crate) fn trim(bytes: &[u8]) -> &[u8] {
    trim_end(trim_begin(bytes))
  }

  #[
    allow(
      // False positive
      clippy::arithmetic_side_effects,
      // Buffer has enough capacity and `base64` already returns a valid string
      clippy::unwrap_used
    )
  ]
  fn base64_from_array<'output, const I: usize, const O: usize>(
    input: &[u8; I],
    output: &'output mut [u8; O],
  ) -> &'output str {
    assert!(O >= I * 150 / 100);
    let len = STANDARD.encode_slice(input, output).unwrap();
    from_utf8_opt(output.get(..len).unwrap_or_default()).unwrap()
  }

  fn trim_begin(mut bytes: &[u8]) -> &[u8] {
    while let [first, rest @ ..] = bytes {
      if first.is_ascii_whitespace() {
        bytes = rest;
      } else {
        break;
      }
    }
    bytes
  }

  fn trim_end(mut bytes: &[u8]) -> &[u8] {
    while let [rest @ .., last] = bytes {
      if last.is_ascii_whitespace() {
        bytes = rest;
      } else {
        break;
      }
    }
    bytes
  }
}
