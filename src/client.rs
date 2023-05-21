use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

type Tx = UnboundedSender<Message>;

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
}
