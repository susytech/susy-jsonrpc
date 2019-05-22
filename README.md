# Susy JSON-RPC

Rust implementation of JSON-RPC 2.0 Specification.
Transport-agnostic `core` and transport servers for `http`, `ipc`, `websockets` and `tcp`.

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/susytech/susy-jsonrpc.svg?branch=master
[travis-url]: https://travis-ci.org/susytech/susy-jsonrpc

[Documentation](http://susytech.github.io/jsonrpc/)

## Sub-projects
- [susy-jsonrpc-core](./core) [![crates.io][core-image]][core-url]
- [susy-jsonrpc-http-server](./http) [![crates.io][http-server-image]][http-server-url]
- [susy-jsonrpc-minihttp-server](./minihttp)
- [susy-jsonrpc-ipc-server](./ipc)
- [susy-jsonrpc-tcp-server](./tcp) [![crates.io][tcp-server-image]][tcp-server-url]
- [susy-jsonrpc-ws-server](./ws)
- [susy-jsonrpc-stdio-server](./stdio)
- [susy-jsonrpc-macros](./macros) [![crates.io][macros-image]][macros-url]
- [susy-jsonrpc-server-utils](./server-utils) [![crates.io][server-utils-image]][server-utils-url]
- [susy-jsonrpc-pubsub](./pubsub) [![crates.io][pubsub-image]][pubsub-url]

[core-image]: https://img.shields.io/crates/v/susy-jsonrpc-core.svg
[core-url]: https://crates.io/crates/susy-jsonrpc-core
[http-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-http-server.svg
[http-server-url]: https://crates.io/crates/susy-jsonrpc-http-server
[tcp-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-tcp-server.svg
[tcp-server-url]: https://crates.io/crates/susy-jsonrpc-tcp-server
[macros-image]: https://img.shields.io/crates/v/susy-jsonrpc-macros.svg
[macros-url]: https://crates.io/crates/susy-jsonrpc-macros
[server-utils-image]: https://img.shields.io/crates/v/susy-jsonrpc-server-utils.svg
[server-utils-url]: https://crates.io/crates/susy-jsonrpc-server-utils
[pubsub-image]: https://img.shields.io/crates/v/susy-jsonrpc-pubsub.svg
[pubsub-url]: https://crates.io/crates/susy-jsonrpc-pubsub

## Examples

- [core](./core/examples)
- [macros](./macros/examples)
- [pubsub](./pubsub/examples)

### Basic Usage (with HTTP transport)

```rust
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_minihttp_server;

use susy_jsonrpc_core::{IoHandler, Value, Params};
use susy_jsonrpc_minihttp_server::{ServerBuilder};

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params: Params| {
		Ok(Value::String("hello".to_string()))
	});

	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait().unwrap();
}
```

### Basic usage with macros

```rust
extern crate susy_jsonrpc_core;
#[macro_use]
extern crate susy_jsonrpc_macros;

use susy_jsonrpc_core::Result;

build_rpc_trait! {
	pub trait Rpc {
		/// Adds two numbers and returns a result
		#[rpc(name = "add")]
		fn add(&self, u64, u64) -> Result<u64>;
	}
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}


fn main() {
	let mut io = susy_jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate())
}
