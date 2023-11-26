use actix::{Actor, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws;
use minesweeper_core::messages::{GenericClientMessage, GenericServerMessage};

use super::{handle::LobbyHandle, lobby::LobbyMessageContent};

impl Actor for LobbyHandle {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let address = ctx.address().clone();
        let mut lobby_event_receiver = self.lobby_event_receiver.take().unwrap();
        tokio::spawn(async move {
            loop {
                let task = tokio::task::spawn_blocking(move || {
                    (lobby_event_receiver.recv(), lobby_event_receiver)
                })
                .await
                .unwrap();
                lobby_event_receiver = task.1;
                address
                    .send(GenericServerMessageWrapper(task.0.unwrap()))
                    .await
                    .unwrap();
            }
        });
    }
}

impl Handler<GenericServerMessageWrapper> for LobbyHandle {
    type Result = ();
    fn handle(&mut self, item: GenericServerMessageWrapper, ctx: &mut Self::Context) {
        #[cfg(debug_assertions)]
        ctx.text(serde_json::to_string(&item.0).unwrap());

        #[cfg(not(debug_assertions))]
        ctx.binary(bitcode::serialize(&item.0).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LobbyHandle {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        // let Ok(msg) = msg else { return };
        if let Some(client_message) = match msg.unwrap() {
            #[cfg(debug_assertions)]
            ws::Message::Text(text) => {
                Some(serde_json::from_str(text.to_string().as_str()).unwrap())
            }
            #[cfg(not(debug_assertions))]
            ws::Message::Binary(binary) => Some(bitcode::deserialize(binary.as_ref()).unwrap()),
            _ => Option::<GenericClientMessage>::None,
        } {
            self.lobby_message_sender
                .send(self.lobby_message(LobbyMessageContent::GenericClientMessage(client_message)))
                .unwrap();
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct GenericServerMessageWrapper(pub GenericServerMessage);
