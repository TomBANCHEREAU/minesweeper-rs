use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws;
// use actix_web_actors::ws;
// use futures_util::{stream::SplitSink, SinkExt};
use minesweeper_core::{
    game::{Game, GameEvent, GameInput},
    grid::vec_grid::VecGrid,
    messages::{GenericClientMessage, GenericServerMessage},
    tile::Tile,
};
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::middleware::auth::User;
// use tokio::{net::TcpStream, sync::Mutex};
// use tokio_tungstenite::WebSocketStream;
// use tungstenite::Message;
pub struct WsActor {
    pub user: User,
    pub lobby: Arc<Mutex<Lobby>>,
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.lobby.lock().unwrap().join(ctx.address(), &self.user);
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
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // let Ok(msg) = msg else { return };
        match msg.unwrap() {
            ws::Message::Text(text) => {
                #[cfg(debug_assertions)]
                self.lobby.lock().unwrap().handle_message(
                    serde_json::from_str(text.to_string().as_str()).unwrap(),
                    &self.user,
                );
            }
            ws::Message::Binary(binary) => {
                #[cfg(not(debug_assertions))]
                self.lobby
                    .lock()
                    .unwrap()
                    .handle_message(bitcode::deserialize(binary.as_ref()).unwrap(), &self.user);
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

pub struct Lobby {
    sender: Sender<GameInput>,
    listenners: Arc<Mutex<Vec<Addr<WsActor>>>>,
}
async fn forward_event_to_listeners(
    mut event_receiver: Receiver<GameEvent>,
    listenners: Arc<Mutex<Vec<Addr<WsActor>>>>,
) {
    loop {
        let event =
            tokio::task::spawn_blocking(|| (event_receiver.recv().unwrap(), event_receiver))
                .await
                .unwrap();
        event_receiver = event.1;
        let event = event.0;
        let len = listenners.lock().unwrap().len();
        for listenner in 0..len {
            let cloned = listenners.lock().unwrap().get(listenner).unwrap().clone();
            cloned
                .send(GenericServerMessageWrapper(
                    GenericServerMessage::GameEvent(event.clone()),
                ))
                .await
                .unwrap();
        }
    }
}
impl Lobby {
    pub fn new(grid: VecGrid<Tile>) -> Self {
        let (event_sender, event_receiver) = channel();
        let (input_sender, input_receiver) = channel();
        Game::new(grid, event_sender, input_receiver).start();
        let this = Self {
            sender: input_sender,
            listenners: Default::default(),
        };
        let listenners = this.listenners.clone();
        tokio::spawn(forward_event_to_listeners(event_receiver, listenners));
        return this;
    }
    pub fn join(&mut self, sender: Addr<WsActor>, user: &User) {
        self.sender
            .send(GameInput {
                username: user.username.clone(),
                action: minesweeper_core::game::GameAction::RedrawRequest,
            })
            .unwrap();
        self.listenners.lock().unwrap().push(sender);
    }
    pub fn handle_message(&mut self, message: GenericClientMessage, user: &User) {
        match message {
            GenericClientMessage::GameAction(game_action) => self
                .sender
                .send(GameInput {
                    username: user.username.clone(),
                    action: game_action,
                })
                .unwrap(),
        }
    }
}
