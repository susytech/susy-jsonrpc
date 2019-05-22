# susy-jsonrpc-http-server
Rust http server using JSON-RPC 2.0.

[Documentation](http://susytech.github.io/jsonrpc/susy_jsonrpc_http_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
susy-jsonrpc-core = { git = "https://github.com/susytech/susy-jsonrpc" }
susy-jsonrpc-http-server = { git = "https://github.com/susytech/susy-jsonrpc" }
```

`main.rs`

```rust
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_http_server;

use susy_jsonrpc_core::*;
use susy_jsonrpc_http_server::*;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", |_| {
		Ok(Value::String("hello".into()))
	});

    let server = ServerBuilder::new(io)
			.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
			.start_http(&"127.0.0.1:3030".parse().unwrap())
			.expect("Unable to start RPC server");

	server.wait().unwrap();
}
```
