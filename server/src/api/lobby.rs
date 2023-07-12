use core::{game::Game, grid::vec_grid::VecGrid, tile::Tile};
use std::sync::{Arc, Mutex};

use actix_web::{
    get, post,
    web::{self, Redirect},
    Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;
use nanoid::nanoid;
use serde::Deserialize;

use crate::lobby::{Lobbies, Lobby, WsActor};

#[derive(Deserialize)]
pub struct CreateLobbyBody {
    grid_width: u8,
    grid_height: u8,
}

#[get("")]
pub async fn get_index(lobbies: web::Data<Lobbies>) -> actix_web::Result<impl Responder> {
    let lobbies = lobbies.lock().unwrap();
    Ok(web::Json(lobbies.len()))
}

#[post("")]
pub async fn create(
    lobbies: web::Data<Lobbies>,
    body: web::Form<CreateLobbyBody>,
) -> impl Responder {
    let id = nanoid!();
    let new_lobby = Arc::new(Mutex::new(Lobby::new(Game::new(VecGrid::<Tile>::new(
        body.grid_width,
        body.grid_height,
    )))));
    lobbies.lock().unwrap().insert(id.clone(), new_lobby);
    Redirect::to(format!("/lobby.html?id={}", id)).see_other()
}

// #[get("/{lobby_id}")]
// pub async fn get_index(lobbies: web::Data<Lobbies>) -> actix_web::Result<impl Responder> {
//     let lobbies = lobbies.lock().unwrap();
//     Ok(web::Json(lobbies.len()))
// }

#[get("/{lobby_id}/ws")]
pub async fn lobby_ws(
    lobby_id: web::Path<String>,
    lobbies: web::Data<Lobbies>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let lobbies = lobbies.lock().unwrap();
    let Some(lobby) = lobbies.get(lobby_id.as_str()) else {return Ok(HttpResponse::BadRequest().finish())};
    return ws::start(
        WsActor {
            lobby: Arc::clone(lobby),
        },
        &req,
        stream,
    );
}
