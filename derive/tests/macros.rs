use serde_json;
use susy_jsonrpc_derive::rpc;
use susy_jsonrpc_core::{IoHandler, Response};

pub enum MyError {}
impl From<MyError> for susy_jsonrpc_core::Error {
	fn from(_e: MyError) -> Self {
		unreachable!()
	}
}

type Result<T> = ::std::result::Result<T, MyError>;

#[rpc]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Negates number and returns a result
	#[rpc(name = "neg")]
	fn neg(&self, _: i64) -> Result<i64>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("add_alias1", "add_alias2"))]
	fn add(&self, _: u64, _: u64) -> Result<u64>;
}

#[derive(Default)]
struct RpcImpl;

impl Rpc for RpcImpl {
	fn protocol_version(&self) -> Result<String> {
		Ok("version1".into())
	}

	fn neg(&self, a: i64) -> Result<i64> {
		Ok(-a)
	}

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

#[test]
fn should_accept_empty_array_as_no_params() {
	let mut io = IoHandler::new();
	let rpc = RpcImpl::default();
	io.extend_with(rpc.to_delegate());

	// when
	let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"protocolVersion","params":[]}"#;
	let req2 = r#"{"jsonrpc":"2.0","id":1,"method":"protocolVersion","params":null}"#;
	let req3 = r#"{"jsonrpc":"2.0","id":1,"method":"protocolVersion"}"#;

	let res1 = io.handle_request_sync(req1);
	let res2 = io.handle_request_sync(req2);
	let res3 = io.handle_request_sync(req3);
	let expected = r#"{
		"jsonrpc": "2.0",
		"result": "version1",
		"id": 1
	}"#;
	let expected: Response = serde_json::from_str(expected).unwrap();

	// then
	let result1: Response = serde_json::from_str(&res1.unwrap()).unwrap();
	assert_eq!(expected, result1);

	let result2: Response = serde_json::from_str(&res2.unwrap()).unwrap();
	assert_eq!(expected, result2);

	let result3: Response = serde_json::from_str(&res3.unwrap()).unwrap();
	assert_eq!(expected, result3);
}

#[test]
fn should_accept_single_param() {
	let mut io = IoHandler::new();
	let rpc = RpcImpl::default();
	io.extend_with(rpc.to_delegate());

	// when
	let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"neg","params":[1]}"#;

	let res1 = io.handle_request_sync(req1);

	// then
	let result1: Response = serde_json::from_str(&res1.unwrap()).unwrap();
	assert_eq!(result1, serde_json::from_str(r#"{
		"jsonrpc": "2.0",
		"result": -1,
		"id": 1
	}"#).unwrap());
}

#[test]
fn should_accept_multiple_params() {
	let mut io = IoHandler::new();
	let rpc = RpcImpl::default();
	io.extend_with(rpc.to_delegate());

	// when
	let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"add","params":[1, 2]}"#;

	let res1 = io.handle_request_sync(req1);

	// then
	let result1: Response = serde_json::from_str(&res1.unwrap()).unwrap();
	assert_eq!(result1, serde_json::from_str(r#"{
		"jsonrpc": "2.0",
		"result": 3,
		"id": 1
	}"#).unwrap());
}

#[test]
fn should_use_method_name_aliases() {
	let mut io = IoHandler::new();
	let rpc = RpcImpl::default();
	io.extend_with(rpc.to_delegate());

	// when
	let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"add_alias1","params":[1, 2]}"#;
	let req2 = r#"{"jsonrpc":"2.0","id":1,"method":"add_alias2","params":[1, 2]}"#;

	let res1 = io.handle_request_sync(req1);
	let res2 = io.handle_request_sync(req2);

	// then
	let result1: Response = serde_json::from_str(&res1.unwrap()).unwrap();
	assert_eq!(result1, serde_json::from_str(r#"{
		"jsonrpc": "2.0",
		"result": 3,
		"id": 1
	}"#).unwrap());

	let result2: Response = serde_json::from_str(&res2.unwrap()).unwrap();
	assert_eq!(result2, serde_json::from_str(r#"{
		"jsonrpc": "2.0",
		"result": 3,
		"id": 1
	}"#).unwrap());
}

