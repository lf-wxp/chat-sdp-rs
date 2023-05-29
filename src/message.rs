use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionExecute};
use crate::response::ResponseMessage;
use crate::transmit::{Transmit, TransmitExecute};
use crate::{PeerMap, RoomMap};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Message {
  Action(Action),
  Transmit(Transmit),
}

impl MessageExecute for Message {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) -> ResponseMessage {
    match self {
      Message::Action(action) => action.execute(room_map.clone()),
      Message::Transmit(transmit) => transmit.execute(peer_map.clone()),
    }
  }
}

pub trait MessageExecute {
  fn execute(&self, peer_map: PeerMap, room_map: RoomMap) -> ResponseMessage;
}
