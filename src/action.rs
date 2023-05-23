use serde::{Deserialize, Serialize};

use crate::message::{ResponseMessage, State};
use crate::room::Room;
use crate::RoomMap;

#[derive(Serialize, Deserialize)]
pub struct CreateRoom {
  name: String,
  desc: Option<String>,
  passwd: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveRoom {
  uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListRoom;

impl RoomExecute for CreateRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    let room = Room::new(
      self.name.to_owned(),
      self.desc.to_owned(),
      self.passwd.to_owned(),
    );
    match room_map.lock().unwrap().insert(room.uuid(), room) {
      Some(_) => ResponseMessage::new(State::success, "success".to_owned()),
      None => ResponseMessage::new(State::error, "error".to_owned()),
    }
  }
}

impl RoomExecute for RemoveRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match room_map.lock().unwrap().remove(&self.uuid) {
      Some(_) => ResponseMessage::new(State::success, "success".to_owned()),
      None => ResponseMessage::new(State::error, "error".to_owned()),
    }
  }
}

impl RoomExecute for ListRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match room_map.lock() {
      Ok(map) => {
        let list = map.values().cloned().collect::<Vec<Room>>();
        ResponseMessage::new(State::success, "success".to_owned())
      },
      Err(_) => {
        ResponseMessage::new(State::error, "error".to_owned())
      },
    }
  }
}

#[derive(Serialize, Deserialize)]
pub enum Action {
  CreateRoom(CreateRoom),
  RemoveRoom(RemoveRoom),
  ListRoom(ListRoom),
}

impl ActionExecute for Action {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match self {
      Action::CreateRoom(create_room) => create_room.execute(room_map.clone()),
      Action::RemoveRoom(remove_room) => remove_room.execute(room_map.clone()),
      Action::ListRoom(list_room) => list_room.execute(room_map.clone()),
    }
  }
}

pub trait RoomExecute {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage;
}

pub trait ActionExecute {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage;
}
