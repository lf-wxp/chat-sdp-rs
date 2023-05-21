use std::{
  collections::HashMap,
  env,
  io::Error as IoError,
  net::SocketAddr,
  sync::{Arc, Mutex},
};

use futures::{
  future::{self, Either},
  pin_mut, SinkExt, StreamExt, TryStreamExt,
};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
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

use client::Client;
use room::Room;

use crate::{action::RoomExecute, transmit::TransmitExecute};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Client>>>;
type RoomMap = Arc<Mutex<HashMap<String, Room>>>;

async fn handle_connection(
  peer_map: PeerMap,
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

  let (tx, mut rx) = unbounded_channel();
  let client = Client::new(addr, "test".to_owned(), tx);
  let uuid_key = client.uuid();

  peer_map.lock().unwrap().insert(uuid_key.clone(), client);

  let (mut sink, stream) = ws_stream.split();

  let (inner_tx, mut inner_rx) = unbounded_channel::<Message>();

  let incoming_tx = inner_tx.clone();
  let broadcast_incoming = stream.try_for_each(|msg| {
    match serde_json::from_str(msg.to_text().unwrap()) {
      Ok(message) => {
        println!(
          "Received a message from {}: {}",
          addr,
          msg.to_text().unwrap()
        );
        match message {
          message::Message::Action(action) => match action {
            action::Action::CreateRoom(create_room) => {
              create_room.execute(room_map.clone());
            }
          },
          message::Message::Transmit(transmit) => match transmit {
            transmit::Transmit::Broadcast(broadcast) => {
              broadcast.execute(peer_map.clone());
            }
            transmit::Transmit::Unicast(unicast) => {
              unicast.execute(peer_map.clone());
            }
          },
        };
      }
      Err(_) => {
        println!("error message");
        incoming_tx
          .send(Message::Text(
            serde_json::to_string(&message::ResponseMessage {
              state: message::State::error,
              message: "contracture".to_owned(),
            })
            .unwrap(),
          ))
          .unwrap();
      }
    };

    future::ok(())
  });

  tokio::spawn(async move {
    while let Some(message) = inner_rx.recv().await {
      sink.send(message).await.unwrap();
    }
  });

  // let receive_from_others = UnboundedReceiverStream::new(rx).map(Ok).forward(sink);
  let cast_tx = inner_tx.clone();
  let receive_from_others = async move {
    while let Some(message) = rx.recv().await {
      cast_tx.send(message).unwrap();
    }
  };

  pin_mut!(broadcast_incoming, receive_from_others);
  match future::select(broadcast_incoming, receive_from_others).await {
    Either::Left((value1, _)) => println!("broadcast {:?}", value1),
    Either::Right((value2, _)) => println!("receive {:?}", value2),
  };

  println!("{} disconnected", &addr);
  peer_map.lock().unwrap().remove(&uuid_key);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
  let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".to_string());

  let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
  let room_map = RoomMap::new(Mutex::new(HashMap::new()));

  let try_socket = TcpListener::bind(&addr).await;
  let listener = try_socket.expect("Failed to bind");
  println!("Listening on: {}", addr);

  while let Ok((stream, addr)) = listener.accept().await {
    tokio::spawn(handle_connection(
      peer_map.clone(),
      room_map.clone(),
      stream,
      addr,
    ));
  }

  Ok(())
}
