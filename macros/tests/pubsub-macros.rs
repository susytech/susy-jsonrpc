extern crate serde_json;
extern crate susy_jsonrpc_core;
extern crate susy_jsonrpc_pubsub;
#[macro_use]
extern crate susy_jsonrpc_macros;

use std::sync::Arc;
use susy_jsonrpc_core::futures::sync::mpsc;
use susy_jsonrpc_core::Result;
use susy_jsonrpc_pubsub::{PubSubHandler, SubscriptionId, Session, PubSubMetadata};
use susy_jsonrpc_macros::{pubsub, Trailing};

build_rpc_trait! {
	pub trait Rpc {
		type Metadata;

		#[pubsub(name = "hello")] {
			/// Hello subscription
			#[rpc(name = "hello_subscribe")]
			fn subscribe(&self, Self::Metadata, pubsub::Subscriber<String>, u32, Trailing<u64>);

			/// Unsubscribe from hello subscription.
			#[rpc(name = "hello_unsubscribe")]
			fn unsubscribe(&self, SubscriptionId) -> Result<bool>;
		}
	}
}

#[derive(Default)]
struct RpcImpl;

impl Rpc for RpcImpl {
	type Metadata = Metadata;

	fn subscribe(&self, _meta: Self::Metadata, subscriber: pubsub::Subscriber<String>, _pre: u32, _trailing: Trailing<u64>) {
		let _sink = subscriber.assign_id(SubscriptionId::Number(5));
	}

	fn unsubscribe(&self, _id: SubscriptionId) -> Result<bool> {
		Ok(true)
	}
}

#[derive(Clone, Default)]
struct Metadata;
impl susy_jsonrpc_core::Metadata for Metadata {}
impl PubSubMetadata for Metadata {
	fn session(&self) -> Option<Arc<Session>> {
		let (tx, _rx) = mpsc::channel(1);
		Some(Arc::new(Session::new(tx)))
	}
}

#[test]
fn test_invalid_trailing_pubsub_params() {
	let mut io = PubSubHandler::default();
	let rpc = RpcImpl::default();
	io.extend_with(rpc.to_delegate());

	// when
	let meta = Metadata;
	let req = r#"{"jsonrpc":"2.0","id":1,"method":"hello_subscribe","params":[]}"#;
	let res = io.handle_request_sync(req, meta);
	let expected = r#"{
		"jsonrpc": "2.0",
		"error": {
			"code": -32602,
			"message": "Couldn't parse parameters: `params` should have at least 1 argument(s)",
			"data": "\"\""
		},
		"id": 1
	}"#;

	let expected: susy_jsonrpc_core::Response = serde_json::from_str(expected).unwrap();
	let result: susy_jsonrpc_core::Response = serde_json::from_str(&res.unwrap()).unwrap();
	assert_eq!(expected, result);
}