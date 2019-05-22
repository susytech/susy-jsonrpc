extern crate susy_jsonrpc_core;
#[macro_use]
extern crate susy_jsonrpc_macros;
extern crate susy_jsonrpc_tcp_server;

use std::collections::BTreeMap;

use susy_jsonrpc_core::{futures, MetaIoHandler, Metadata, Error, Value, Result};
use susy_jsonrpc_core::futures::future::FutureResult;

#[derive(Clone, Default)]
struct Meta(String);
impl Metadata for Meta {}

build_rpc_trait! {
	pub trait Rpc {
		type Metadata;

		/// Adds two numbers and returns a result
		#[rpc(name = "add")]
		fn add(&self, u64, u64) -> Result<u64>;

		/// Multiplies two numbers. Second number is optional.
		#[rpc(name = "mul")]
		fn mul(&self, u64, susy_jsonrpc_macros::Trailing<u64>) -> Result<u64>;

		/// Performs asynchronous operation
		#[rpc(name = "callAsync")]
		fn call(&self, u64) -> FutureResult<String, Error>;

		/// Performs asynchronous operation with meta
		#[rpc(meta, name = "callAsyncMeta", alias = [ "callAsyncMetaAlias", ])]
		fn call_meta(&self, Self::Metadata, BTreeMap<String, Value>) -> FutureResult<String, Error>;
	}
}

struct RpcImpl;
impl Rpc for RpcImpl {
	type Metadata = Meta;

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}

	fn mul(&self, a: u64, b: susy_jsonrpc_macros::Trailing<u64>) -> Result<u64> {
		Ok(a * b.unwrap_or(1))
	}

	fn call(&self, x: u64) -> FutureResult<String, Error> {
		futures::finished(format!("OK: {}", x))
	}

	fn call_meta(&self, meta: Self::Metadata, map: BTreeMap<String, Value>) -> FutureResult<String, Error> {
		futures::finished(format!("From: {}, got: {:?}", meta.0, map))
	}
}


fn main() {
	let mut io = MetaIoHandler::default();
	let rpc = RpcImpl;

	io.extend_with(rpc.to_delegate());

	let server = susy_jsonrpc_tcp_server::ServerBuilder::new(io)
		.session_meta_extractor(|context: &susy_jsonrpc_tcp_server::RequestContext| {
			Meta(format!("{}", context.peer_addr))
		})
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait()
}