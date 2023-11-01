use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::{
  data::get_client_map,
  response::{ResponseMessage, State},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CallType {
  Video,
  Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdpMessage {
  pub call_type: CallType,
  pub sdp: String,
}

impl SdpMessage {
  pub fn is_empty(&self) -> bool {
    self.sdp.is_empty()
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast {
  from: String,
  message: SdpMessage,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unicast {
  from: String,
  pub to: String,
  message: SdpMessage,
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
            let message = serde_json::to_string(&TransmitMessage {
              from: self.from.clone(),
              message: self.message.clone(),
            })
            .unwrap();
            recp.tx.send(Message::Text(message)).unwrap();
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
        let message = serde_json::to_string(&TransmitMessage {
          from: self.from.clone(),
          message: self.message.clone(),
        })
        .unwrap();
        target_peer.tx.send(Message::Text(message)).unwrap();

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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransmitMessage {
  from: String,
  message: SdpMessage,
}

pub trait TransmitExecute {
  fn execute(&self) -> ResponseMessage;
}
