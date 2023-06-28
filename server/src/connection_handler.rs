use std::sync::{Arc, Mutex};

use actix::{Actor, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use core::messages::{GenericClientMessage, GenericServerMessage};

use crate::lobby::{Lobbies, Lobby};

pub struct WsActor {
    lobby: Arc<Mutex<Lobby>>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct GenericServerMessageWrapper(pub GenericServerMessage);

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.lobby.lock().unwrap().join(ctx.address());
    }
}

impl Handler<GenericServerMessageWrapper> for WsActor {
    type Result = ();
    fn handle(&mut self, item: GenericServerMessageWrapper, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&item.0).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        // let Ok(msg) = msg else { return };
        match msg.unwrap() {
            ws::Message::Text(text) => self
                .lobby
                .lock()
                .unwrap()
                .handle_message(serde_json::from_str(text.to_string().as_str()).unwrap()),
            ws::Message::Binary(_) => todo!(),
            ws::Message::Continuation(_) => todo!(),
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Close(_) => todo!(),
            ws::Message::Nop => todo!(),
        }
    }
}

#[get("/lobby/{lobby_id}")]
pub async fn index(
    path: web::Path<(String)>,
    lobbies: web::Data<Lobbies>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let lobbies = lobbies.lock().unwrap();
    let Some(lobby) = lobbies.get(path.as_str()) else {return Ok(HttpResponse::BadRequest().finish())};
    let resp = ws::start(
        WsActor {
            lobby: Arc::clone(lobby),
        },
        &req,
        stream,
    );
    resp
}

// use crate::{lobby::Lobby, server};

// pub type MutexedLobby = Arc<Mutex<Lobby>>;

// pub async fn accept_connection(peer: SocketAddr, stream: TcpStream, state: MutexedServerState) {
//     let mut lobby: Option<MutexedLobby> = None;
//     let ws_stream = accept_hdr_async(stream, |req: &Request, mut response: Response| {
//     })
//     let (sender, mut receiver) = ws_stream.split();
//     // sender.
//     // sender.send();
//     let sender = Arc::new(sync::Mutex::new(sender));
//     lobby.lock().unwrap().join(sender);

//     while let Some(msg) = receiver.next().await {
//         let msg = msg.unwrap();
//         if let Message::Text(str) = msg {
//             let mut lobby = lobby.lock().unwrap();
//             let client_message: GenericClientMessage = serde_json::from_str(str.as_str()).unwrap();
//             lobby.handle_message(client_message);
//         }
//     }
// }
