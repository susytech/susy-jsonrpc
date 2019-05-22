extern crate susy_jsonrpc_ws_server;

use susy_jsonrpc_ws_server::ServerBuilder;
use susy_jsonrpc_ws_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		println!("Processing");
		Ok(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
