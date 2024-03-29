use actix_files::NamedFile;
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    middleware::{Logger, NormalizePath, TrailingSlash},
    web::scope,
    App, HttpServer,
};
use middleware::auth::Authenticate;
mod api;
mod lobby;
mod middleware;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let api_config = api::ApiConfig::default();
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::new("%r %s %T"))
            .wrap(Authenticate)
            .service(scope("/api").configure(api_config.configure()))
            .service(actix_files::Files::new("/images", "../client/images"))
            .service(
                actix_files::Files::new("/", "../client/dist")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("../client/dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
