use actix_web::{middleware, web::scope, App, HttpServer};
mod api;
mod lobby;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api_config = api::ApiConfig::default();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .service(scope("/api").configure(api_config.configure()))
            .service(actix_files::Files::new("/", "../client").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
