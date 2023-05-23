use serde::{Deserialize, Serialize};

use crate::{RoomMap, response::{State, ResponseMessage, Data}, room::Room};

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

#[derive(Serialize, Deserialize)]
pub struct ListRoomResponse {
  state: State,
  message: String,
  data: Vec<Room>,
}

impl RoomExecute for CreateRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    let room = Room::new(
      self.name.to_owned(),
      self.desc.to_owned(),
      self.passwd.to_owned(),
    );
    room_map.lock().unwrap().insert(room.uuid(), room);
    ResponseMessage::new(State::success, "success".to_owned(), None)
  }
}

impl RoomExecute for RemoveRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match room_map.lock().unwrap().remove(&self.uuid) {
      Some(_) => ResponseMessage::new(State::success, "success".to_owned(), None),
      None => ResponseMessage::new(State::error, "error remove room".to_owned(), None),
    }
  }
}

impl RoomExecute for ListRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match room_map.lock() {
      Ok(map) => {
        let list = map.values().cloned().collect::<Vec<Room>>();
        ResponseMessage::new(State::success, "success".to_owned(), Some(Data::RoomList(list)))
      },
      Err(_) => {
        ResponseMessage::new(State::error, "error list room".to_owned(), None)
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
