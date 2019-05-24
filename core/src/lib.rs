//! ### Transport agnostic jsonrpc library.
//!
//! Right now it supports only server side handling requests.
//!
//! ```rust
//! use susy_jsonrpc_core::*;
//! use susy_jsonrpc_core::futures::Future;
//!
//! fn main() {
//! 	let mut io = IoHandler::new();
//! 	io.add_method("say_hello", |_| {
//!			Ok(Value::String("Hello World!".into()))
//! 	});
//!
//! 	let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
//! 	let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;
//!
//! 	assert_eq!(io.handle_request(request).wait().unwrap(), Some(response.to_string()));
//! }
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub use futures;

#[doc(hidden)]
pub extern crate serde_json;

mod calls;
mod io;

pub mod delegates;
pub mod middleware;
pub mod types;

/// A `Future` trait object.
pub type BoxFuture<T> = Box<futures::Future<Item = T, Error = Error> + Send>;

/// A Result type.
pub type Result<T> = ::std::result::Result<T, Error>;

pub use crate::calls::{Metadata, RemoteProcedure, RpcMethod, RpcMethodSimple, RpcNotification, RpcNotificationSimple};
pub use crate::delegates::IoDelegate;
pub use crate::io::{
	Compatibility, FutureOutput, FutureResponse, FutureResult, FutureRpcResult, IoHandler, MetaIoHandler,
};
pub use crate::middleware::{Middleware, Noop as NoopMiddleware};
pub use crate::types::*;
