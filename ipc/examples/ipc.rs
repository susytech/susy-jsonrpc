use susy_jsonrpc_ipc_server;

use susy_jsonrpc_ipc_server::susy_jsonrpc_core::*;

fn main() {
	let mut io = MetaIoHandler::<()>::default();
	io.add_method("say_hello", |_params| Ok(Value::String("hello".to_string())));
	let _server = susy_jsonrpc_ipc_server::ServerBuilder::new(io)
		.start("/tmp/susy-example.ipc")
		.expect("Server should start ok");
}
