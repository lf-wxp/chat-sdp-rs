use serde::{Deserialize, Serialize};

use crate::{
  action::{self, Action},
  response::ResponseMessage,
  transmit::{Transmit, TransmitExecute},
  connect::{Connect, ConnectExecute},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Message {
  Action(Action),
  Transmit(Transmit),
  Connect(Connect),
}

impl Execute for Message {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match self {
      Message::Action(action) => action::Execute::execute(action, client_id),
      Message::Transmit(transmit) => transmit.execute(),
      Message::Connect(connect) => connect.execute(),
    }
  }
}

pub trait Execute {
  fn execute(&self, client_id: String) -> ResponseMessage;
}
