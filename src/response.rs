use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

use crate::{room::room::Room};
use crate::client::client::ClientInfo;

#[derive(Serialize, Deserialize)]
pub enum State {
  success,
  error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Data {
  RoomList(Vec<Room>),
  ClientList(Vec<ClientInfo>),
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
  pub state: State,
  pub message: String,
  pub data: Option<Data>,
}

impl ResponseMessage {
  pub fn new(state: State, message: String, data: Option<Data>) -> ResponseMessage {
    ResponseMessage {
      state,
      message,
      data,
    }
  }
}

impl TryInto<tungstenite::Message> for ResponseMessage {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
    Ok(tungstenite::Message::Text(serde_json::to_string(&self)?))
  }
}
