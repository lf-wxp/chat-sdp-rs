use std::{
  collections::HashMap,
  env,
  io::Error as IoError,
  net::SocketAddr,
  sync::{Arc, Mutex},
};

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
mod message;
mod room;
mod transmit;
mod response;

use client::client::Client;
use room::room::Room;
use message::Execute;
use response::{ResponseMessage, State};

type ClientMap = Arc<Mutex<HashMap<String, Client>>>;
type RoomMap = Arc<Mutex<HashMap<String, Room>>>;

async fn handle_connection(
  client_map: ClientMap,
  room_map: RoomMap,
  raw_stream: TcpStream,
  addr: SocketAddr,
) {
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

  client_map.lock().unwrap().insert(uuid_key.clone(), client);

  let (sink, stream) = ws_stream.split();

  let (transform_tx, transform_rx) = unbounded_channel::<Message>();

  let message_tx = transform_tx.clone();
  let execute_message = stream.try_for_each(|msg| {
    let message = match serde_json::from_str::<message::Message>(msg.to_text().unwrap()) {
      Ok(message) => {
        println!(
          "Received a message from {}: {}",
          addr,
          msg.to_text().unwrap()
        );
        message.execute(client_map.clone(), room_map.clone(), uuid_key.clone())
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
  client_map.lock().unwrap().remove(&uuid_key);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
  let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".to_string());

  let client_map = ClientMap::new(Mutex::new(HashMap::new()));
  let room_map = RoomMap::new(Mutex::new(HashMap::new()));

  let try_socket = TcpListener::bind(&addr).await;
  let listener = try_socket.expect("Failed to bind");
  println!("Listening on: {}", addr);

  while let Ok((stream, addr)) = listener.accept().await {
    tokio::spawn(handle_connection(
      client_map.clone(),
      room_map.clone(),
      stream,
      addr,
    ));
  }

  Ok(())
}
