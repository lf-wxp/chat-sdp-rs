use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

use crate::action::{Action, ActionExecute};
use crate::transmit::{Transmit, TransmitExecute};
use crate::{PeerMap, RoomMap};

#[derive(Serialize, Deserialize)]
pub enum Message {
  Action(Action),
  Transmit(Transmit),
}

#[derive(Serialize, Deserialize)]
pub enum State {
  success,
  error,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
  pub state: State,
  pub message: String,
}

impl ResponseMessage {
  pub fn new(state: State, message: String) -> ResponseMessage {
    ResponseMessage { state, message }
  }
  pub fn message(state: State, message: String) -> Result<tungstenite::Message, serde_json::Error> {
    ResponseMessage::new(state, message).try_into()
  }
}

impl TryInto<tungstenite::Message> for ResponseMessage {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
    Ok(tungstenite::Message::Text(serde_json::to_string(&self)?))
  }
}

impl MessageExecute for Message {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) -> ResponseMessage {
    match self {
      Message::Action(action) =>  action.execute(room_map.clone()),
      Message::Transmit(transmit) => transmit.execute(peer_map.clone()),
    }
  }
}

pub trait MessageExecute {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) -> ResponseMessage;
}
