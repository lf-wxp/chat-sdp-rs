use serde::{Deserialize, Serialize};

use crate::{
  data::get_room_map,
  response::{Data, ResponseMessage, State},
};

use super::room_struct::Room;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoom {
  name: String,
  desc: Option<String>,
  passwd: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveRoom {
  uuid: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRoom;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRoomResponse {
  state: State,
  message: String,
  data: Vec<Room>,
}

impl RoomExecute for CreateRoom {
  fn execute(&self) -> ResponseMessage {
    let room = Room::new(
      self.name.to_owned(),
      self.desc.to_owned(),
      self.passwd.to_owned(),
    );
    match get_room_map() {
      Some(map) => {
        map.insert(room.uuid(), room);
        ResponseMessage::new(State::success, "success".to_owned(), None)
      }
      None => ResponseMessage::new(State::error, "create room error".to_owned(), None),
    }
  }
}

impl RoomExecute for RemoveRoom {
  fn execute(&self) -> ResponseMessage {
    let error = ResponseMessage::new(State::error, "remove room error".to_owned(), None);
    match get_room_map() {
      Some(map) => map.remove(&self.uuid).map_or(error.clone(), |_| {
        ResponseMessage::new(State::success, "success".to_owned(), None)
      }),
      None => error,
    }
  }
}

impl RoomExecute for ListRoom {
  fn execute(&self) -> ResponseMessage {
    match get_room_map() {
      Some(map) => {
        let list = map.values().cloned().collect::<Vec<Room>>();
        ResponseMessage::new(
          State::success,
          "success".to_owned(),
          Some(Data::RoomList(list)),
        )
      }
      None => ResponseMessage::new(State::error, "error list room".to_owned(), None),
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

impl RoomExecute for Action {
  fn execute(&self) -> ResponseMessage {
    match self {
      Action::Create(create_room) => create_room.execute(),
      Action::Remove(remove_room) => remove_room.execute(),
      Action::List(list_room) => list_room.execute(),
    }
  }
}

pub trait RoomExecute {
  fn execute(&self) -> ResponseMessage;
}
