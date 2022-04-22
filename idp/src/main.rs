//! Idp Web Server
//!
mod resource;
mod token;

use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use resource::hello_html::hello_html_handler;
use resource::hello_rest::hello_rest_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(
                web::JsonConfig::default()
                    .limit(4096) // limit request payload size
                    .content_type(|mime| mime == mime::TEXT_PLAIN) // only accept text/plain content type
                    .error_handler(|err, req| {
                        log::info!("error request {}", req.path());
                        error::InternalError::from_response(err, HttpResponse::Conflict().into())
                            .into()
                    }), // use custom error handler
            )
            .service(hello_html_handler)
            .service(web::resource("/rest").route(web::post().to(hello_rest_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
