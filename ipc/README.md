# susy-jsonrpc-ipc-server
IPC server (Windows & Linux) for JSON-RPC 2.0.

[Documentation](http://susytech.github.io/jsonrpc/susy_jsonrpc_ipc_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
susy-jsonrpc-ipc-server = "11.0"
```

`main.rs`

```rust
extern crate susy_jsonrpc_ipc_server;

use susy_jsonrpc_ipc_server::ServerBuilder;
use susy_jsonrpc_ipc_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".into()))
	});

	let builder = ServerBuilder::new(io);
	let server = builder.start("/tmp/json-ipc-test.ipc").expect("Couldn't open socket");
	server.wait();
}
```

