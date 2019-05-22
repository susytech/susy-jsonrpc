# susy-jsonrpc-ipc-server
IPC server (Windows & Linux) for JSON-RPC 2.0.

[Documentation](http://susytech.github.io/jsonrpc/susy_jsonrpc_ipc_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
susy-jsonrpc-ipc-server = { git = "https://github.com/susytech/susy-jsonrpc" }
```

`main.rs`

```rust
extern crate susy_jsonrpc_ipc_server;

use susy_jsonrpc_ipc_server::Server;
use susy_jsonrpc_ipc_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".into()))
	});

	let server = Server::new("/tmp/json-ipc-test.ipc", io).unwrap();
	::std::thread::spawn(move || server.run());
}
```

