use actix_web::{get, web, Responder};
use serde::{Deserialize, Serialize};

#[get("/todos/{id}")]
async fn hello_rest_handler(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}
