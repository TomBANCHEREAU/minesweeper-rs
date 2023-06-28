use actix::Addr;
// use actix_web_actors::ws;
// use futures_util::{stream::SplitSink, SinkExt};
use core::{
    game::{Game, GameEvent, GameInput},
    messages::{GenericClientMessage, GenericServerMessage},
    pubsub::{self, Subject},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
// use tokio::{net::TcpStream, sync::Mutex};
// use tokio_tungstenite::WebSocketStream;
// use tungstenite::Message;

use crate::connection_handler::{GenericServerMessageWrapper, WsActor};

pub type Lobbies = Mutex<HashMap<String, Arc<Mutex<Lobby>>>>;
// type Sender = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>;

#[derive(Default)]
pub struct Lobby {
    game: Game,
}
struct Observer {
    sender: Addr<WsActor>,
}
impl pubsub::Observer<GameEvent> for Observer {
    fn notify(&mut self, event: GameEvent) {
        tokio::spawn(self.sender.send(GenericServerMessageWrapper(
            GenericServerMessage::GameEvent(event),
        )));
    }
}
impl Lobby {
    pub fn join(&mut self, sender: Addr<WsActor>) {
        self.game.subscribe(Observer { sender })
    }
    pub fn handle_message(&mut self, message: GenericClientMessage) {
        dbg!(&message);
        match message {
            GenericClientMessage::GameAction(game_action) => self.game.play(GameInput {
                action: game_action,
            }),
        }
    }
}
