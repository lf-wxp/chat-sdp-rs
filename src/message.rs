use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

use crate::action::{Action, RoomExecute};
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
}

impl TryInto<tungstenite::Message> for ResponseMessage {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
    Ok(tungstenite::Message::Text(serde_json::to_string(&self)?))
  }
}

impl MessageExecute for Message {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) {
    match self {
      Message::Action(action) => match action {
        Action::CreateRoom(create_room) => {
          create_room.execute(room_map.clone());
        }
      },
      Message::Transmit(transmit) => match transmit {
        Transmit::Broadcast(broadcast) => {
          broadcast.execute(peer_map.clone());
        }
        Transmit::Unicast(unicast) => {
          unicast.execute(peer_map.clone());
        }
      },
    };
  }
}

pub trait MessageExecute {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) {}
}
