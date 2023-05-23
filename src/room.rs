use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]  
pub struct  Room {
  desc: Option<String>,
	users: Vec<String>,
	uuid: String,
	name: String,
	passwd: Option<String>,
}
impl Room {
  pub fn new(name: String, desc: Option<String>, passwd: Option<String>) -> Room {
    Room { desc, users: vec![], uuid: nanoid!(), name, passwd }
  }
  pub fn uuid(&self) -> String {
    self.uuid.clone()
  }
}
