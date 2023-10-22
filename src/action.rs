use serde::{Deserialize, Serialize};

use crate::{
  client::{self, action::ClientExecute},
  response::ResponseMessage,
  room::{self, action::RoomExecute},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  Room(room::action::Action),
  Client(client::action::Action),
}

impl Execute for Action {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match self {
      Action::Room(room_action) => room_action.execute(),
      Action::Client(client_action) => client_action.execute(client_id),
    }
  }
}
pub trait Execute {
  fn execute(&self, client_id: String) -> ResponseMessage;
}
