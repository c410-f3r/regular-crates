# WTX 

[![CI](https://github.com/c410-f3r/regularcrates/workflows/CI/badge.svg)](https://github.com/c410-f3r/wtx/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/wtx.svg)](https://crates.io/crates/wtx)
[![Documentation](https://docs.rs/wtx/badge.svg)](https://docs.rs/wtx)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.71-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.71.html)

Intended to group different web transport implementations.

## WebSocket

[fastwebsockets](https://github.com/denoland/fastwebsockets) served as an initial inspiration for the skeleton of this implementation so thanks to the authors.

```rust
use core::str;
use wtx::web_socket::{Frame, FrameBufferVec, OpCode, Stream, WebSocketClient};

pub async fn handle_client_frames(ws: &mut WebSocketClient<impl Stream>) -> wtx::Result<()> {
  let mut fb = FrameBufferVec::default();
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
```

See the `examples` directory for more suggestions.