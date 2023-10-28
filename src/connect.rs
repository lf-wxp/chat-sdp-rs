use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use crate::{
  client::client_struct::ClientInfo,
  data::{get_client_map, get_room_map},
  response::{Data, ResponseMessage, State},
  room::room_struct::Room,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectInfo {
  pub room_list: Vec<Room>,
  pub client_list: Vec<ClientInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connect;

impl ConnectExecute for Connect {
  fn execute(&self) -> ResponseMessage {
    match get_client_map() {
      Some(peers) => {
        let broadcast_recipients = peers.iter().map(|(_, ws_sink)| ws_sink);

        println!(
          "broadcast count, {:?}",
          broadcast_recipients.clone().count()
        );

        let room_list =
          get_room_map().map_or(vec![], |x| x.values().cloned().collect::<Vec<Room>>());
        let client_list = get_client_map().map_or(vec![], |x| {
          x.values()
            .map(ClientInfo::from)
            .collect::<Vec<ClientInfo>>()
        });
        let connect_info = ConnectInfo {
          room_list,
          client_list,
        };

        let message = ResponseMessage::new(
          State::success,
          "ok".to_owned(),
          Some(Data::ConnectInfo(connect_info)),
        );

        for recp in broadcast_recipients {
          recp
            .tx
            .send(Message::Text(serde_json::to_string(&message).unwrap()))
            .unwrap();
        }

        message
      }
      None => ResponseMessage::new(
        State::error,
        "get room and client info error".to_owned(),
        None,
      ),
    }
  }
}
pub trait ConnectExecute {
  fn execute(&self) -> ResponseMessage;
}
