//! # idpサーバです
//!
mod hello_html;
mod hello_rest;

use actix_web::{App, HttpServer};
use hello_html::hello_html_handler;
use hello_rest::hello_rest_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_html_handler)
            .service(hello_rest_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
