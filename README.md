# Susy JSON-RPC

Rust implementation of JSON-RPC 2.0 Specification.
Transport-agnostic `core` and transport servers for `http`, `ipc`, `websockets` and `tcp`.

**New!** Support for [clients](#Client-support).

[![Build Status][travis-image]][travis-url]
[![Build Status][appveyor-image]][appveyor-url]

[travis-image]: https://travis-ci.org/susytech/susy-jsonrpc.svg?branch=master
[travis-url]: https://travis-ci.org/susytech/susy-jsonrpc
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/susytech/susy-jsonrpc?svg=true
[appveyor-url]: https://ci.appveyor.com/project/susytech/susy-jsonrpc/branch/master

[Documentation](http://susytech.github.io/jsonrpc/)

## Sub-projects
- [susy-jsonrpc-core](./core) [![crates.io][core-image]][core-url]
- [susy-jsonrpc-http-server](./http) [![crates.io][http-server-image]][http-server-url]
- [susy-jsonrpc-ipc-server](./ipc) [![crates.io][ipc-server-image]][ipc-server-url]
- [susy-jsonrpc-tcp-server](./tcp) [![crates.io][tcp-server-image]][tcp-server-url]
- [susy-jsonrpc-ws-server](./ws) [![crates.io][ws-server-image]][ws-server-url]
- [susy-jsonrpc-stdio-server](./stdio) [![crates.io][stdio-server-image]][stdio-server-url]
- [susy-jsonrpc-derive](./derive) [![crates.io][derive-image]][derive-url]
- [susy-jsonrpc-server-utils](./server-utils) [![crates.io][server-utils-image]][server-utils-url]
- [susy-jsonrpc-pubsub](./pubsub) [![crates.io][pubsub-image]][pubsub-url]

[core-image]: https://img.shields.io/crates/v/susy-jsonrpc-core.svg
[core-url]: https://crates.io/crates/susy-jsonrpc-core
[http-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-http-server.svg
[http-server-url]: https://crates.io/crates/susy-jsonrpc-http-server
[ipc-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-ipc-server.svg
[ipc-server-url]: https://crates.io/crates/susy-jsonrpc-ipc-server
[tcp-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-tcp-server.svg
[tcp-server-url]: https://crates.io/crates/susy-jsonrpc-tcp-server
[ws-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-ws-server.svg
[ws-server-url]: https://crates.io/crates/susy-jsonrpc-ws-server
[stdio-server-image]: https://img.shields.io/crates/v/susy-jsonrpc-stdio-server.svg
[stdio-server-url]: https://crates.io/crates/susy-jsonrpc-stdio-server
[derive-image]: https://img.shields.io/crates/v/susy-jsonrpc-derive.svg
[derive-url]: https://crates.io/crates/susy-jsonrpc-derive
[server-utils-image]: https://img.shields.io/crates/v/susy-jsonrpc-server-utils.svg
[server-utils-url]: https://crates.io/crates/susy-jsonrpc-server-utils
[pubsub-image]: https://img.shields.io/crates/v/susy-jsonrpc-pubsub.svg
[pubsub-url]: https://crates.io/crates/susy-jsonrpc-pubsub

## Examples

- [core](./core/examples)
- [derive](./derive/examples)
- [pubsub](./pubsub/examples)

### Basic Usage (with HTTP transport)

```rust
use susy_jsonrpc_core::{IoHandler, Value, Params};
use susy_jsonrpc_http_server::{ServerBuilder};

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

### Basic usage with derive

```rust
use susy_jsonrpc_core::Result;
use susy_jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
	/// Adds two numbers and returns a result
	#[rpc(name = "add")]
	fn add(&self, u64, u64) -> Result<u64>;
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
```

### Client support

```rust
use susy_jsonrpc_client::local;
use susy_jsonrpc_core::futures::future::{self, Future, FutureResult};
use susy_jsonrpc_core::{Error, IoHandler, Result};
use susy_jsonrpc_derive::rpc;

/// Rpc trait
#[rpc]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> FutureResult<String, Error>;
}

struct RpcImpl;

impl Rpc for RpcImpl {
	fn protocol_version(&self) -> Result<String> {
		Ok("version1".into())
	}

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}

	fn call(&self, _: u64) -> FutureResult<String, Error> {
		future::ok("OK".to_owned())
	}
}

fn main() {
	let mut io = IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

	let fut = {
		let (client, server) = local::connect::<gen_client::Client, _, _>(io);
		client.add(5, 6).map(|res| println!("5 + 6 = {}", res)).join(server)
	};
	fut.wait().unwrap();
}

```
