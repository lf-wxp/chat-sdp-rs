use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::{ClientMap, response::{ResponseMessage, State}};

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
  from: String,
  message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Unicast {
  from: String,
  pub to: String,
  message: String,
}

impl TransmitExecute for Broadcast {
  fn execute(&self, client_map: ClientMap) -> ResponseMessage {
    let peers = client_map.lock().unwrap();
    let broadcast_recipients = peers
      .iter()
      .filter(|(uuid, _)| uuid != &&self.from)
      .map(|(_, ws_sink)| ws_sink);

    println!(
      "broadcast count, {:?}",
      broadcast_recipients.clone().count()
    );

    for recp in broadcast_recipients {
      if !self.message.is_empty() {
        recp.tx.send(Message::Text(self.message.clone())).unwrap();
      };
    }

    ResponseMessage::new(State::success, "ok broadcast".to_owned(), None)
  }
}

impl TransmitExecute for Unicast {
  fn execute(&self, client_map: ClientMap) -> ResponseMessage {
    let peers = client_map.lock().unwrap();
    let target_peer = peers.get(&self.to).unwrap();
    target_peer
      .tx
      .send(Message::Text(self.message.clone()))
      .unwrap();

    ResponseMessage::new(State::success, "ok unicast".to_owned(), None)
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transmit {
  Broadcast(Broadcast),
  Unicast(Unicast),
}

impl TransmitExecute for Transmit {
  fn execute(&self, client_map: ClientMap) -> ResponseMessage {
    match self {
      Transmit::Broadcast(broadcast) => {
        broadcast.execute(client_map)
      }
      Transmit::Unicast(unicast) => {
        unicast.execute(client_map)
      }
    }
  }
}

pub trait TransmitExecute {
  fn execute(&self, client_map: ClientMap) -> ResponseMessage;
}
