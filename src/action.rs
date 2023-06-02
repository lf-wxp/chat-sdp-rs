use serde::{Deserialize, Serialize};

use crate::response::ResponseMessage;
use crate::{room, ClientMap};
use crate::{client, RoomMap};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  Room(room::action::Action),
  Client(client::action::Action),
}

impl Execute for Action {
  fn execute(&self, room_map: RoomMap, client_map: ClientMap, client_id: String) -> ResponseMessage {
    match self {
      Action::Room(room_action) => match room_action {
        room::action::Action::CreateRoom(create_room) => room::action::Execute::execute(create_room, room_map.clone()),
        room::action::Action::RemoveRoom(remove_room) => room::action::Execute::execute(remove_room, room_map.clone()),
        room::action::Action::ListRoom(list_room) => room::action::Execute::execute(list_room, room_map.clone()),
      },
      Action::Client(client_action) => match client_action {
        client::action::Action::UpdateName(update_name) => client::action::Execute::execute(update_name, client_map.clone(), client_id),
        client::action::Action::ListClient(list_client) => client::action::Execute::execute(list_client, client_map.clone(), client_id)
      },
    }
  }
}
pub trait Execute {
  fn execute(&self, room_map: RoomMap, client_map: ClientMap, client_id: String) -> ResponseMessage;
}
