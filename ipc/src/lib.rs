//! Cross-platform JSON-RPC IPC transport.

#![warn(missing_docs)]

extern crate susy_jsonrpc_core as jsonrpc;
extern crate susy_jsonrpc_server_utils as server_utils;
extern crate susy_tokio_ipc;
extern crate tokio_service;

#[macro_use] extern crate log;

#[cfg(test)] #[macro_use] extern crate lazy_static;
#[cfg(test)] extern crate env_logger;
#[cfg(test)] mod logger;

mod server;
mod meta;
pub use meta::{MetaExtractor, RequestContext};
pub use server::{Server, ServerBuilder};

pub use self::server_utils::tokio_core;
pub use self::server_utils::session::{SessionStats, SessionId};