use serde::{Deserialize, Serialize};

use crate::RoomMap;
use crate::room::Room;

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

impl RoomExecute for CreateRoom {
  fn execute(&self, room_map: RoomMap) {
    let room = Room::new(self.name.to_owned(), self.desc.to_owned(), self.passwd.to_owned());
    room_map.lock().unwrap().insert(room.uuid(), room);
  }
}

impl RoomExecute for RemoveRoom {
  fn execute(&self, room_map: RoomMap) {
    room_map.lock().unwrap().remove(&self.uuid);
  }
}

#[derive(Serialize, Deserialize)]  
pub enum Action {
  CreateRoom(CreateRoom),  
}

pub trait RoomExecute {
  fn execute(&self, room_map: RoomMap) {} 
}
