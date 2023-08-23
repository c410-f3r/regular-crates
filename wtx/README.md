# WTX 

[![CI](https://github.com/c410-f3r/regular-crates/workflows/CI/badge.svg)](https://github.com/c410-f3r/wtx/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/wtx.svg)](https://crates.io/crates/wtx)
[![Documentation](https://docs.rs/wtx/badge.svg)](https://docs.rs/wtx)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.71-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.71.html)

Intended to group different web transport implementations.

## WebSocket

Provides low and high level abstractions to dispatch frames, as such, it is up to you to implement [Stream](https://docs.rs/wtx/latest/wtx/trait.Stream.html) with any desired logic or use any of the built-in strategies through the selection of features.

[fastwebsockets](https://github.com/denoland/fastwebsockets) served as an initial inspiration for the skeleton of this implementation so thanks to the authors.

```rust
use wtx::{Stream, web_socket::{FrameBufferVec, FrameVecMut, OpCode, WebSocketClientOwned}};

pub async fn handle_client_frames(ws: &mut WebSocketClientOwned<impl Stream>) -> wtx::Result<()> {
  let fb = &mut FrameBufferVec::default();
  loop {
    let frame = match ws.read_msg(fb).await {
      Err(err) => {
        println!("Error: {err}");
        ws.write_frame(FrameVecMut::new_fin(fb.into(), OpCode::Close, &[])?).await?;
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