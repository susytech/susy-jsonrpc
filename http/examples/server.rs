use susy_jsonrpc_http_server::susy_jsonrpc_core::*;
use susy_jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params: Params| Ok(Value::String("hello".to_string())));

	let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
