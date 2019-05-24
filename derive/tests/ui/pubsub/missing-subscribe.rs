#[macro_use]
extern crate susy_jsonrpc_derive;
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_pubsub;

#[rpc]
pub trait Rpc {
	type Metadata;

	// note that a subscribe method is missing

	/// Unsubscribe from hello subscription.
	#[pubsub(subscription = "hello", unsubscribe, name = "hello_unsubscribe")]
	fn unsubscribe(&self, Option<Self::Metadata>, SubscriptionId) -> Result<bool>;
}

fn main() {}
