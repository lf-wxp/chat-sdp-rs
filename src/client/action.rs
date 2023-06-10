use serde::{Deserialize, Serialize};

use crate::{
  response::{ResponseMessage, State, Data},
  ClientMap,
};

use super::client::{Client, ClientInfo};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateName {
  name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ListClient;

impl Execute for UpdateName {
  fn execute(&self, client_map: ClientMap, client_id: String) -> ResponseMessage {
    match client_map.lock() {
      Ok(mut map) => match map.get_mut(&client_id) {
        Some(client) => {
          client.update_name(self.name.clone());
          return ResponseMessage::new(State::success, "success".to_owned(), None);
        }
        None => ResponseMessage::new(State::error, "update client name".to_owned(), None),
      },
      Err(_) => ResponseMessage::new(State::error, "update client name".to_owned(), None),
    }
  }
}

impl Execute for ListClient {
  fn execute(&self, client_map: ClientMap, _client_id: String) -> ResponseMessage {
    match client_map.lock() {
      Ok(map) => {
        let list = map.values().map(|x| ClientInfo::from(x.clone())).collect::<Vec<ClientInfo>>();
        ResponseMessage::new(
          State::success,
          "success".to_owned(),
          Some(Data::ClientList(list)),
        )
      }
      Err(_) => ResponseMessage::new(State::error, "error list room".to_owned(), None),
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  UpdateName(UpdateName),
  ListClient(ListClient),
}

impl Execute for Action {
  fn execute(&self, client_map: ClientMap, client_id: String) -> ResponseMessage {
    match self {
      Action::UpdateName(update_name) => update_name.execute(client_map, client_id),
      Action::ListClient(list_client) => list_client.execute(client_map, client_id),
    }
  }
}

pub trait Execute {
  fn execute(&self, client_map: ClientMap, client_id: String) -> ResponseMessage;
}
