# susy-jsonrpc-tcp-server
TCP server for JSON-RPC 2.0.

[Documentation](http://susytech.github.io/jsonrpc/susy_jsonrpc_tcp_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
susy-jsonrpc-tcp-server = { git = "https://github.com/susytech/susy-jsonrpc" }
```

`main.rs`

```rust
extern crate susy_jsonrpc_tcp_server;

use susy_jsonrpc_tcp_server::*;
use susy_jsonrpc_tcp_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
```


