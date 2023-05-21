use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::transmit::Transmit;

#[derive(Serialize, Deserialize)]  
pub enum Message {
  Action(Action),
  Transmit(Transmit),
}

#[derive(Serialize, Deserialize)]  
pub enum State {
  success,
  error,
}

#[derive(Serialize, Deserialize)]  
pub struct  ResponseMessage {
  pub state: State,
  pub message: String,
}

