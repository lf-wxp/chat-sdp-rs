use std::{net::SocketAddr, error::Error};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

type Tx = UnboundedSender<Message>;

#[derive(Clone)]
pub struct Client {
  uuid: String,
  name: String,
  pub tx: Tx,
}

impl Client {
  pub fn new(addr: SocketAddr, name: String, tx: Tx) -> Client {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientInfo {
  name: String,
  uuid: String,
}

impl From<Client> for ClientInfo {
  fn from(client: Client) -> Self {
    let Client {uuid, name, tx } = client;
    ClientInfo { name, uuid }
  }
}




