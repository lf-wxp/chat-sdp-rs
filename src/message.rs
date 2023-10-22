use serde::{Deserialize, Serialize};

use crate::{
  action::{self, Action},
  response::ResponseMessage,
  transmit::{Transmit, TransmitExecute},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Message {
  Action(Action),
  Transmit(Transmit),
}

impl Execute for Message {
  fn execute(&self, client_id: String) -> ResponseMessage {
    match self {
      Message::Action(action) => action::Execute::execute(action, client_id),
      Message::Transmit(transmit) => transmit.execute(),
    }
  }
}

pub trait Execute {
  fn execute(&self, client_id: String) -> ResponseMessage;
}
