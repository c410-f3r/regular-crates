# WebSocket benchmark

Call the `ws-bench` binary passing the URLs of all different available echo servers.

```
cargo run --bin ws-bench --release -- http://127.0.0.1:8080/some_server_name http://127.0.0.1:8081/another_server_name ..
```