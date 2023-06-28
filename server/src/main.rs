use actix_web::{web, App, HttpServer};
use connection_handler::index;
use lobby::Lobbies;
mod connection_handler;
mod lobby;
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let lobbies: web::Data<Lobbies> = web::Data::new(Default::default());
    HttpServer::new(move || {
        App::new()
            .app_data(lobbies.clone())
            .service(actix_files::Files::new("/", "../client").index_file("index.html"))
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
