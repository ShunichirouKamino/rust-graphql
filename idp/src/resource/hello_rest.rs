use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}

pub async fn hello_rest_handler(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}
