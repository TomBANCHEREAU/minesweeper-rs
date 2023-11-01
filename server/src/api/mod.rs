use actix_web::web;

use crate::lobby::Lobbies;

mod lobby;

#[derive(Clone)]
pub struct ApiConfig {
    lobbies: web::Data<Lobbies>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            lobbies: web::Data::new(Lobbies::default()),
        }
    }
}

impl ApiConfig {
    pub fn configure(&self) -> impl FnOnce(&mut web::ServiceConfig) {
        let lobbies = self.lobbies.clone();
        move |cfg: &mut web::ServiceConfig| {
            cfg.service(
                web::scope("/lobby")
                    .app_data(lobbies)
                    .service(lobby::create)
                    .service(lobby::get_index)
                    .service(lobby::get_lobby)
                    .service(lobby::lobby_ws),
            );
            // .route(
            //     "/",
            //     web::get().to(|| async { HttpResponse::Ok().body("body") }),
            // );
            // println!("Call ended {:?}", cfg  );
        }
    }
}
