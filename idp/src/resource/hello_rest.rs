use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::token::jwt::make_jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}

pub async fn hello_rest_handler(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

pub async fn make_jwt_handler() -> HttpResponse {
    println!("make_jwt_handler");
    let jwt = make_jwt("secret", "aud");
    println!("jwt: {:?}", &jwt);
    HttpResponse::Ok().json(jwt) // <- send response
}
