use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use nanoid::nanoid;

type Tx = UnboundedSender<Message>;

#[derive(Clone)]
pub struct Client {
  uuid: String,
  name: String,
  pub tx: Tx,
}

impl Client {
  pub fn new(addr: SocketAddr, name: Option<String>, tx: Tx) -> Client {
    let name = name.unwrap_or(nanoid!());
    Client {
      uuid: format!("{}-{}", addr, name),
      name,
      tx,
    }
  }
  pub fn uuid(&self) -> String {
    self.uuid.clone()
  }

  pub fn update_name(&mut self, name: String) {
    self.name = name;
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
  name: String,
  uuid: String,
}

impl From<&Client> for ClientInfo {
  fn from(client: &Client) -> Self {
    let Client {uuid, name, tx: _ } = client;
    ClientInfo { name: name.to_string(), uuid: uuid.to_string() }
  }
}




