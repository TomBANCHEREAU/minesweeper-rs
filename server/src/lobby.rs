use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws;
// use actix_web_actors::ws;
// use futures_util::{stream::SplitSink, SinkExt};
use minesweeper_core::{
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
pub struct WsActor {
    pub lobby: Arc<Mutex<Lobby>>,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.lobby.lock().unwrap().join(ctx.address());
    }
}

impl Handler<GenericServerMessageWrapper> for WsActor {
    type Result = ();
    fn handle(&mut self, item: GenericServerMessageWrapper, ctx: &mut Self::Context) {
        #[cfg(debug_assertions)]
        ctx.text(serde_json::to_string(&item.0).unwrap());

        #[cfg(not(debug_assertions))]
        ctx.binary(bitcode::serialize(&item.0).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        // let Ok(msg) = msg else { return };
        match msg.unwrap() {
            ws::Message::Text(text) => {
                #[cfg(debug_assertions)]
                self.lobby
                    .lock()
                    .unwrap()
                    .handle_message(serde_json::from_str(text.to_string().as_str()).unwrap());
            }
            ws::Message::Binary(binary) => {
                #[cfg(not(debug_assertions))]
                self.lobby
                    .lock()
                    .unwrap()
                    .handle_message(bitcode::deserialize(binary.as_ref()).unwrap());
            }
            ws::Message::Continuation(_) => todo!(),
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Close(_) => (),
            ws::Message::Nop => todo!(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GenericServerMessageWrapper(pub GenericServerMessage);

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
    pub fn new(game: Game) -> Self {
        Self { game }
    }
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
