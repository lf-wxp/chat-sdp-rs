use serde::{Deserialize, Serialize};

use crate::action::{Action, self};
use crate::client::client::Client;
use crate::response::ResponseMessage;
use crate::transmit::{Transmit, TransmitExecute};
use crate::{ClientMap, RoomMap};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Message {
  Action(Action),
  Transmit(Transmit),
}

impl Execute for Message {
  fn execute(&self, client_map: ClientMap, room_map: RoomMap, client_id: String) -> ResponseMessage {
    match self {
      Message::Action(action) => action::Execute::execute(action, room_map.clone(), client_map.clone(), client_id),
      Message::Transmit(transmit) => transmit.execute(client_map.clone()),
    }
  }
}

pub trait Execute {
  fn execute(&self, client_map: ClientMap, room_map: RoomMap, client_id: String) -> ResponseMessage;
}
