use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::{
  data::get_client_map,
  response::{ResponseMessage, State},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast {
  from: String,
  message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unicast {
  from: String,
  pub to: String,
  message: String,
}

impl TransmitExecute for Broadcast {
  fn execute(&self) -> ResponseMessage {
    match get_client_map() {
      Some(peers) => {
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
      None => ResponseMessage::new(State::error, "get client map error".to_owned(), None),
    }
  }
}

impl TransmitExecute for Unicast {
  fn execute(&self) -> ResponseMessage {
    match get_client_map() {
      Some(peers) => {
        let target_peer = peers.get(&self.to).unwrap();
        target_peer
          .tx
          .send(Message::Text(self.message.clone()))
          .unwrap();

        ResponseMessage::new(State::success, "ok unicast".to_owned(), None)
      }
      None => ResponseMessage::new(State::error, "get client map error".to_owned(), None),
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transmit {
  Broadcast(Broadcast),
  Unicast(Unicast),
}

impl TransmitExecute for Transmit {
  fn execute(&self) -> ResponseMessage {
    match self {
      Transmit::Broadcast(broadcast) => broadcast.execute(),
      Transmit::Unicast(unicast) => unicast.execute(),
    }
  }
}

pub trait TransmitExecute {
  fn execute(&self) -> ResponseMessage;
}
