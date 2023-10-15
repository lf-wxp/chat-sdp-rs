use serde::{Serialize, Deserialize};

use crate::{response::{State, ResponseMessage, Data}, RoomMap};

use super::room_struct::Room;

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

impl Execute for CreateRoom {
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

impl Execute for RemoveRoom {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match room_map.lock().unwrap().remove(&self.uuid) {
      Some(_) => ResponseMessage::new(State::success, "success".to_owned(), None),
      None => ResponseMessage::new(State::error, "error remove room".to_owned(), None),
    }
  }
}

impl Execute for ListRoom {
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
#[serde(rename_all = "camelCase")]
pub enum Action {
  Create(CreateRoom),
  Remove(RemoveRoom),
  List(ListRoom),
}

impl Execute for Action {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage {
    match self {
      Action::Create(create_room) => create_room.execute(room_map.clone()),
      Action::Remove(remove_room) => remove_room.execute(room_map.clone()),
      Action::List(list_room) => list_room.execute(room_map.clone()),
    }
  }
}

pub trait Execute {
  fn execute(&self, room_map: RoomMap) -> ResponseMessage;
}
