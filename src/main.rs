use std::{env, io::Error as IoError, net::SocketAddr};

use futures::{
  future::{self, Either},
  pin_mut, StreamExt, TryStreamExt,
};

use sender_sink::wrappers::UnboundedSenderSink;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
  accept_hdr_async,
  tungstenite::{
    handshake::server::{Request, Response},
    protocol::Message,
  },
};

mod action;
mod client;
mod connect;
mod data;
mod message;
mod response;
mod room;
mod transmit;

use {
  client::client_struct::Client,
  connect::{Connect, ConnectExecute},
  data::get_client_map,
  message::Execute,
  response::{ResponseMessage, State},
};

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
  println!("Incoming TCP connection from: {}", addr);

  let get_headers = |req: &Request, response: Response| {
    println!("Received a new ws handshake");
    println!("The request's path is: {}", req.uri().path());
    println!("The request's headers are:");
    for (ref header, _value) in req.headers() {
      println!("* {}: {:?}", header, _value);
    }
    Ok(response)
  };

  let ws_stream = accept_hdr_async(raw_stream, get_headers)
    .await
    .expect("Error during the websocket handshake occurred");
  println!("WebSocket connection established: {}", addr);

  let (tx, rx) = unbounded_channel();
  let client = Client::new(addr, None, tx);
  let uuid_key = client.uuid();

  if let Some(client_map) = get_client_map() {
    client_map.insert(uuid_key.clone(), client);
  }

  Connect {}.execute();

  let (sink, stream) = ws_stream.split();

  let (transform_tx, transform_rx) = unbounded_channel::<Message>();

  let message_tx = transform_tx.clone();

  let execute_message = stream.try_for_each(|msg| {
    println!(
      "Received a pure message from {}: {}",
      addr,
      msg.to_text().unwrap()
    );
    let message = match serde_json::from_str::<message::Message>(msg.to_text().unwrap()) {
      Ok(message) => {
        println!(
          "Received a message from {}: {}",
          addr,
          msg.to_text().unwrap()
        );
        message.execute(uuid_key.clone())
      }
      Err(_) => ResponseMessage::new(State::error, "construct".to_owned(), None),
    };

    message_tx.send(message.try_into().unwrap()).unwrap();

    future::ok(())
  });

  let transform_task = UnboundedReceiverStream::new(transform_rx)
    .map(Ok)
    .forward(sink);

  let receive_tx = transform_tx.clone();
  let receive_from_others = UnboundedReceiverStream::new(rx)
    .map(Ok)
    .forward(UnboundedSenderSink::from(receive_tx));

  pin_mut!(execute_message, receive_from_others, transform_task);
  match future::select(
    future::select(execute_message, receive_from_others),
    transform_task,
  )
  .await
  {
    Either::Left((value, _)) => match value {
      Either::Left((value1, _)) => println!("broadcast {:?}", value1),
      Either::Right((value2, _)) => println!("receive {:?}", value2),
    },
    Either::Right((value2, _)) => println!("receive {:?}", value2),
  }

  println!("{} disconnected", &addr);
  if let Some(client_map) = get_client_map() {
    client_map.remove(&uuid_key);
  }
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
  let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8888".to_string());

  let try_socket = TcpListener::bind(&addr).await;
  let listener = try_socket.expect("Failed to bind");
  println!("Listening on: {}", addr);

  while let Ok((stream, addr)) = listener.accept().await {
    tokio::spawn(handle_connection(stream, addr));
  }

  Ok(())
}
