#[macro_use]
extern crate susy_jsonrpc_derive;
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_pubsub;

#[rpc]
pub trait Rpc {
	type Metadata;

	/// Hello subscription
	#[pubsub(subscription = "hello", subscribe, name = "hello_subscribe", alias("hello_sub"))]
	fn subscribe(&self, _: Self::Metadata, _: typed::Subscriber<String>, _: u64);

	// note that the unsubscribe method is missing
}

fn main() {}
