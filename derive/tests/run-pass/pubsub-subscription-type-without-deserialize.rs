extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_client;
extern crate susy_jsonrpc_pubsub;
#[macro_use]
extern crate susy_jsonrpc_derive;
extern crate log;

use std::sync::Arc;
use susy_jsonrpc_core::Result;
use susy_jsonrpc_pubsub::{typed::Subscriber, SubscriptionId, Session, PubSubHandler};

#[rpc]
pub trait Rpc<T> {
	type Metadata;

	/// Hello subscription
	#[pubsub(subscription = "hello", subscribe, name = "hello_subscribe", alias("hello_sub"))]
	fn subscribe(&self, _: Self::Metadata, _: Subscriber<T>);

	/// Unsubscribe from hello subscription.
	#[pubsub(subscription = "hello", unsubscribe, name = "hello_unsubscribe")]
	fn unsubscribe(&self, a: Option<Self::Metadata>, b: SubscriptionId) -> Result<bool>;
}

// One way serialization
#[derive(Serialize)]
struct SerializeOnly {
	foo: String,
}

struct RpcImpl;
impl Rpc<SerializeOnly> for RpcImpl {
	type Metadata = Arc<Session>;

	fn subscribe(&self, _: Self::Metadata, _: Subscriber<SerializeOnly>) {
		unimplemented!();
	}

	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool> {
		unimplemented!();
	}
}

fn main() {
	let mut io = PubSubHandler::default();
	let rpc = RpcImpl;
	io.extend_with(rpc.to_delegate());
}
