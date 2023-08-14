#[cfg(feature = "http-client")]
use alloc::boxed::Box;
use core::{
  fmt::{Debug, Display, Formatter},
  num::TryFromIntError,
};

/// Grouped individual errors
#[derive(Debug)]
pub enum Error {
  /// It it not possible to read a frame of a connection that was previously closed.
  ConnectionClosed,
  /// Control frames must not be fragmented.
  FragmentedControlFrame,
  /// Received close frame has invalid parameters.
  InvalidCloseFrame,
  /// Received header of a handshake does not contain `connection`.
  InvalidConnectionHeader,
  /// Received frame wasn't supposed be to a continuation.
  InvalidContinuationFrame,
  /// Header indices are out-of-bounds or the number of bytes are too small.
  InvalidHeaderBounds,
  /// Following frames that compose a message must be a continuation.
  InvalidMsgFrame,
  /// No op code can be represented with the provided byte.
  InvalidOpCodeByte {
    /// Provided byte
    byte: u8,
  },
  /// Payload indices are out-of-bounds or the number of bytes are too small.
  InvalidPayloadBounds,
  /// Sec-Websocket-Version must be 13.
  InvalidSecWebsocketVersion,
  /// Invalid UTF-8.
  InvalidUTF8,
  /// Received header of a upgrade does not contain `Sec-WebSocket-Key`.
  MissingSecWebSocketKey,
  /// Received status code of a handshake differs from `SwitchingProtocols`.
  MissingSwitchingProtocols,
  /// Received header of a handshake does not contain `upgrade`.
  MissingUpgradeHeader,
  /// Url does not contain an authority.
  NoAuthority,
  /// Server received a frame without a mask.
  NoFrameMask,
  /// It wasn't possible to establish a upgrade.
  NoUpgradeConnection,
  /// Reserved bits are not zero.
  ReservedBitsAreNotZero,
  /// Unexpected end of file when reading.
  UnexpectedEOF,
  /// Control frames have a maximum allowed size.
  VeryLargeControlFrame,
  /// Frame payload exceeds the defined threshold.
  VeryLargePayload,

  // External
  //
  #[cfg(feature = "hyper")]
  /// See [hyper::Error]
  HyperError(hyper::Error),
  #[cfg(feature = "hyper")]
  /// See [hyper::Error]
  HyperHttpError(hyper::http::Error),
  #[cfg(feature = "http-client")]
  /// See [http_types::Error]
  HttpTypesError(Box<http_types::Error>),
  #[cfg(feature = "std")]
  /// See [std::io::Error]
  IoError(std::io::Error),
  /// See [TryFromIntError]
  TryFromIntError(TryFromIntError),
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    <Self as Debug>::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for Error {
  #[inline]
  fn from(from: hyper::Error) -> Self {
    Self::HyperError(from)
  }
}

#[cfg(feature = "hyper")]
impl From<hyper::http::Error> for Error {
  #[inline]
  fn from(from: hyper::http::Error) -> Self {
    Self::HyperHttpError(from)
  }
}

#[cfg(feature = "http-client")]
impl From<http_types::Error> for Error {
  #[inline]
  fn from(from: http_types::Error) -> Self {
    Self::HttpTypesError(from.into())
  }
}

impl From<core::str::Utf8Error> for Error {
  #[inline]
  fn from(_: core::str::Utf8Error) -> Self {
    Self::InvalidUTF8
  }
}

impl From<TryFromIntError> for Error {
  #[inline]
  fn from(from: TryFromIntError) -> Self {
    Self::TryFromIntError(from)
  }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
  #[inline]
  fn from(from: std::io::Error) -> Self {
    Self::IoError(from)
  }
}
