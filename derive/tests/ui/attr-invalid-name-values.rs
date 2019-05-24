extern crate serde;
extern crate susy_jsonrpc_core;
#[macro_use]
extern crate susy_jsonrpc_derive;

use susy_jsonrpc_core::Result;

#[rpc]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(Xname = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;
}

fn main() {}
