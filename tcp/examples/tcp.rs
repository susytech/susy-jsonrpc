extern crate susy_jsonrpc_tcp_server;
extern crate env_logger;
use susy_jsonrpc_tcp_server::ServerBuilder;
use susy_jsonrpc_tcp_server::susy_jsonrpc_core::*;

fn main() {
	env_logger::init();
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		println!("Processing");
		Ok(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait()
}

