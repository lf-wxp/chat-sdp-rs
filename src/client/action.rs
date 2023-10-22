use serde::{Deserialize, Serialize};

use crate::{
  data::get_client_map,
  response::{Data, ResponseMessage, State},
};

use super::client_struct::ClientInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateName {
  name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListClient;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInfo;

impl ClientExecute for UpdateName {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match get_client_map() {
      Some(map) => match map.get_mut(&client_id) {
        Some(client) => {
          client.update_name(self.name.clone());
          ResponseMessage::new(State::success, "success".to_owned(), None)
        }
        None => ResponseMessage::new(State::error, "update client name".to_owned(), None),
      },
      None => ResponseMessage::new(State::error, "update client name".to_owned(), None),
    }
  }
}

impl ClientExecute for ListClient {
  fn execute(&self, _client_id: String) -> ResponseMessage {
    match get_client_map() {
      Some(map) => {
        let list = map
          .values()
          .map(ClientInfo::from)
          .collect::<Vec<ClientInfo>>();
        ResponseMessage::new(
          State::success,
          "success".to_owned(),
          Some(Data::ClientList(list)),
        )
      }
      None => ResponseMessage::new(State::error, "error list room".to_owned(), None),
    }
  }
}

impl ClientExecute for GetInfo {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match get_client_map() {
      Some(map) => {
        if let Some(client) = map.get(&client_id) {
          return ResponseMessage::new(
            State::success,
            "success".to_owned(),
            Some(Data::ClientInfo(ClientInfo::from(client))),
          );
        }
        ResponseMessage::new(State::error, "error get client info".to_owned(), None)
      }
      None => ResponseMessage::new(State::error, "error get client info".to_owned(), None),
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  UpdateName(UpdateName),
  ListClient(ListClient),
  GetInfo(GetInfo),
}

impl ClientExecute for Action {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match self {
      Action::UpdateName(update_name) => update_name.execute(client_id),
      Action::ListClient(list_client) => list_client.execute(client_id),
      Action::GetInfo(get_info) => get_info.execute(client_id),
    }
  }
}

pub trait ClientExecute {
  fn execute(&self, client_id: String) -> ResponseMessage;
}
