# susy-jsonrpc-stdio-server
STDIN/STDOUT server for JSON-RPC 2.0.
Takes one request per line and outputs each response on a new line.

[Documentation](http://susytech.github.io/jsonrpc/susy_jsonrpc_stdio_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
susy-jsonrpc-stdio-server = { git = "https://github.com/susytech/susy-jsonrpc" }
```

`main.rs`

```rust
extern crate susy_jsonrpc_stdio_server;

use susy_jsonrpc_stdio_server::server;
use susy_jsonrpc_stdio_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".to_owned()))
	});

	ServerBuilder::new(io).build();
}
```
