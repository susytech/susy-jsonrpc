//! jsonrpc server over tcp/ip
//!
//! ```no_run
//! extern crate susy_jsonrpc_core;
//! extern crate susy_jsonrpc_tcp_server;
//!
//! use susy_jsonrpc_core::*;
//! use susy_jsonrpc_tcp_server::ServerBuilder;
//!
//! fn main() {
//! 	let mut io = IoHandler::default();
//! 	io.add_method("say_hello", |_params| {
//! 		Ok(Value::String("hello".to_string()))
//! 	});
//! 	let server = ServerBuilder::new(io)
//!			.start(&"0.0.0.0:0".parse().unwrap())
//!			.expect("Server must start with no issues.");
//!
//!		server.wait();
//! }
//! ```

#![warn(missing_docs)]

extern crate susy_jsonrpc_core as jsonrpc;
extern crate susy_jsonrpc_server_utils as server_utils;
extern crate parking_lot;
extern crate tokio_service;

#[macro_use] extern crate log;

#[cfg(test)] #[macro_use] extern crate lazy_static;
#[cfg(test)] extern crate env_logger;

mod dispatch;
mod meta;
mod server;
mod service;

#[cfg(test)] mod logger;
#[cfg(test)] mod tests;

pub use dispatch::{Dispatcher, PushMessageError};
pub use meta::{MetaExtractor, RequestContext};
pub use server::{ServerBuilder, Server};
pub use self::server_utils::tokio_core;