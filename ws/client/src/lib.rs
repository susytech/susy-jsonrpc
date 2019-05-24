//! JSON-RPC websocket client implementation.
#![deny(missing_docs)]
use failure::Error;
use futures::prelude::*;
use futures::sync::mpsc;
use susy_jsonrpc_client::RpcClient;
pub use susy_jsonrpc_client::{RpcChannel, RpcError};
use log::info;
use std::collections::VecDeque;
use websocket::{ClientBuilder, OwnedMessage};

/// Connect to a JSON-RPC websocket server.
///
/// Uses an unbuffered channel to queue outgoing rpc messages.
pub fn connect<T>(url: &str) -> Result<impl Future<Item = T, Error = RpcError>, Error>
where
	T: From<RpcChannel>,
{
	let client = ClientBuilder::new(url)?
		.async_connect(None)
		.map(|(client, _)| {
			let (sink, stream) = client.split();
			let (sink, stream) = WebsocketClient::new(sink, stream).split();
			let (sender, receiver) = mpsc::channel(0);
			let rpc_client = RpcClient::new(sink, stream, receiver).map_err(|error| eprintln!("{:?}", error));
			tokio::spawn(rpc_client);
			sender.into()
		})
		.map_err(|error| RpcError::Other(error.into()));
	Ok(client)
}

struct WebsocketClient<TSink, TStream> {
	sink: TSink,
	stream: TStream,
	queue: VecDeque<OwnedMessage>,
}

impl<TSink, TStream, TError> WebsocketClient<TSink, TStream>
where
	TSink: Sink<SinkItem = OwnedMessage, SinkError = TError>,
	TStream: Stream<Item = OwnedMessage, Error = TError>,
	TError: Into<Error>,
{
	pub fn new(sink: TSink, stream: TStream) -> Self {
		Self {
			sink,
			stream,
			queue: VecDeque::new(),
		}
	}
}

impl<TSink, TStream, TError> Sink for WebsocketClient<TSink, TStream>
where
	TSink: Sink<SinkItem = OwnedMessage, SinkError = TError>,
	TStream: Stream<Item = OwnedMessage, Error = TError>,
	TError: Into<Error>,
{
	type SinkItem = String;
	type SinkError = RpcError;

	fn start_send(&mut self, request: Self::SinkItem) -> Result<AsyncSink<Self::SinkItem>, Self::SinkError> {
		self.queue.push_back(OwnedMessage::Text(request));
		Ok(AsyncSink::Ready)
	}

	fn poll_complete(&mut self) -> Result<Async<()>, Self::SinkError> {
		loop {
			match self.queue.pop_front() {
				Some(request) => match self.sink.start_send(request) {
					Ok(AsyncSink::Ready) => continue,
					Ok(AsyncSink::NotReady(request)) => {
						self.queue.push_front(request);
						break;
					}
					Err(error) => return Err(RpcError::Other(error.into())),
				},
				None => break,
			}
		}
		self.sink.poll_complete().map_err(|error| RpcError::Other(error.into()))
	}
}

impl<TSink, TStream, TError> Stream for WebsocketClient<TSink, TStream>
where
	TSink: Sink<SinkItem = OwnedMessage, SinkError = TError>,
	TStream: Stream<Item = OwnedMessage, Error = TError>,
	TError: Into<Error>,
{
	type Item = String;
	type Error = RpcError;

	fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
		loop {
			match self.stream.poll() {
				Ok(Async::Ready(Some(message))) => match message {
					OwnedMessage::Text(data) => return Ok(Async::Ready(Some(data))),
					OwnedMessage::Binary(data) => info!("server sent binary data {:?}", data),
					OwnedMessage::Ping(p) => self.queue.push_front(OwnedMessage::Pong(p)),
					OwnedMessage::Pong(_) => {}
					OwnedMessage::Close(c) => self.queue.push_front(OwnedMessage::Close(c)),
				},
				Ok(Async::Ready(None)) => {
					// TODO try to reconnect (#411).
					return Ok(Async::Ready(None));
				}
				Ok(Async::NotReady) => return Ok(Async::NotReady),
				Err(error) => return Err(RpcError::Other(error.into())),
			}
		}
	}
}
