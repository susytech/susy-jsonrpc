//! JSON-RPC servers utilities.

#![warn(missing_docs)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use susy_jsonrpc_core as core;

pub use tokio;
pub use tokio_codec;

pub mod cors;
pub mod hosts;
pub mod session;
pub mod reactor;
mod matcher;
mod stream_codec;
mod suspendable_stream;

pub use crate::suspendable_stream::SuspendableStream;
pub use crate::matcher::Pattern;

/// Codecs utilities
pub mod codecs {
    pub use crate::stream_codec::{StreamCodec, Separator};
}

