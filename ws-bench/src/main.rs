//! WebSocket benchmark

#![allow(
  // Does not matter
  clippy::arithmetic_side_effects,
  // Does not matter
  clippy::unwrap_used
)]

use plotters::{
  prelude::{
    ChartBuilder, IntoDrawingArea, IntoSegmentedCoord, LabelAreaPosition, PathElement, SVGBackend,
    SeriesLabelPosition,
  },
  series::Histogram,
  style::{AsRelative, Color, Palette99, PaletteColor, BLACK, WHITE},
};
use std::time::Instant;
use tokio::{net::TcpStream, task::JoinSet};
use wtx::{
  web_socket::{
    handshake::{WebSocketHandshake, WebSocketHandshakeRaw},
    FrameBufferVec, FrameVecMut, OpCode, WebSocketClientOwned,
  },
  UriParts,
};

const CONNECTIONS: usize = 1;
const NUM_MSGS: usize = 1;
const FRAME_LEN: usize = 1024;
const NUM_FRAMES: usize = {
  let n = NUM_MSGS / 4;
  if n == 0 {
    1
  } else {
    n
  }
};

static FRAME_DATA: &[u8; FRAME_LEN] = &[53; FRAME_LEN];

#[tokio::main]
async fn main() {
  let uris: Vec<_> = std::env::args().skip(1).collect();
  let mut agents = Vec::new();
  for uri in uris {
    let uri_parts = UriParts::from(uri.as_str());
    let mut agent = Agent { result: 0, name: uri_parts.href.to_owned() };
    bench(uri_parts.authority, &mut agent, &uri).await;
    agents.push(agent);
  }
  flush(&agents);
}

async fn bench(addr: &str, agent: &mut Agent, uri: &str) {
  let instant = Instant::now();
  let mut set = JoinSet::new();
  for _ in 0..CONNECTIONS {
    let _handle = set.spawn({
      let local_addr: String = addr.to_owned();
      let local_uri = uri.to_owned();
      async move {
        let fb = &mut FrameBufferVec::default();
        let mut ws = ws(&local_addr, fb, &local_uri).await;
        for _ in 0..NUM_MSGS {
          match NUM_FRAMES {
            0 => {}
            1 => {
              ws.write_frame(FrameVecMut::new_fin(fb.into(), OpCode::Text, FRAME_DATA).unwrap())
                .await
                .unwrap();
            }
            2 => {
              ws.write_frame(FrameVecMut::new_unfin(fb.into(), OpCode::Text, FRAME_DATA).unwrap())
                .await
                .unwrap();
              ws.write_frame(
                FrameVecMut::new_fin(fb.into(), OpCode::Continuation, FRAME_DATA).unwrap(),
              )
              .await
              .unwrap();
            }
            _ => {
              ws.write_frame(FrameVecMut::new_unfin(fb.into(), OpCode::Text, FRAME_DATA).unwrap())
                .await
                .unwrap();
              for _ in (0..NUM_FRAMES).skip(2) {
                ws.write_frame(
                  FrameVecMut::new_unfin(fb.into(), OpCode::Continuation, FRAME_DATA).unwrap(),
                )
                .await
                .unwrap();
              }
              ws.write_frame(
                FrameVecMut::new_fin(fb.into(), OpCode::Continuation, FRAME_DATA).unwrap(),
              )
              .await
              .unwrap();
            }
          }
          assert_eq!(ws.read_frame(fb).await.unwrap().fb().payload().len(), FRAME_LEN * NUM_FRAMES);
        }
        ws.write_frame(FrameVecMut::new_fin(fb.into(), OpCode::Close, &[]).unwrap()).await.unwrap();
      }
    });
  }
  while let Some(rslt) = set.join_next().await {
    rslt.unwrap();
  }
  agent.result = instant.elapsed().as_millis();
}

fn flush(agents: &[Agent]) {
  if agents.is_empty() {
    return;
  }
  let x_spec = agents.iter().map(|el| &el.name).cloned().collect::<Vec<_>>();
  let root = SVGBackend::new("/tmp/ws-bench.png", (1000, 500)).into_drawing_area();
  root.fill(&WHITE).unwrap();
  let mut ctx = ChartBuilder::on(&root)
    .caption(
      format!("{CONNECTIONS} connection(s) sending {NUM_MSGS} message(s) composed by {NUM_FRAMES} frame(s) of {FRAME_LEN} byte(s)"),
      ("sans-serif", (4).percent_height()),
    )
    .margin((1).percent())
    .set_label_area_size(LabelAreaPosition::Left, (15).percent())
    .set_label_area_size(LabelAreaPosition::Bottom, (5).percent())
    .build_cartesian_2d(x_spec.into_segmented(), 0u128..5000)
    .unwrap();
  ctx
    .configure_mesh()
    .axis_desc_style(("sans-serif", 15))
    .bold_line_style(WHITE.mix(0.3))
    .y_desc("Time (ms)")
    .draw()
    .unwrap();
  for (idx, agent) in agents.iter().enumerate() {
    let _ = ctx
      .draw_series(
        Histogram::vertical(&ctx)
          .style(PaletteColor::<Palette99>::pick(idx).mix(0.5).filled())
          .data([(&agent.name, agent.result)]),
      )
      .unwrap()
      .label(format!("{} ({}ms)", &agent.name, agent.result))
      .legend(move |(x, y)| {
        PathElement::new([(x, y), (x + 20, y)], PaletteColor::<Palette99>::pick(idx))
      });
  }
  ctx
    .configure_series_labels()
    .border_style(BLACK)
    .background_style(WHITE.mix(0.8))
    .position(SeriesLabelPosition::UpperRight)
    .draw()
    .unwrap();
  root.present().unwrap();
}

async fn ws(
  authority: &str,
  fb: &mut FrameBufferVec,
  uri: &str,
) -> WebSocketClientOwned<TcpStream> {
  WebSocketHandshakeRaw {
    fb,
    headers_buffer: &mut <_>::default(),
    rb: <_>::default(),
    stream: TcpStream::connect(authority).await.unwrap(),
    uri,
  }
  .handshake()
  .await
  .unwrap()
  .1
}

#[derive(Debug)]
struct Agent {
  result: u128,
  name: String,
}
