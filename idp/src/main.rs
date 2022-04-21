//! # idpサーバです
//!
mod hello_html;
mod hello_rest;

use actix_web::{middleware, web, App, HttpServer};
use hello_html::hello_html_handler;
use hello_rest::hello_rest_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello_html_handler)
            .service(web::resource("/rest").route(web::post().to(hello_rest_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
