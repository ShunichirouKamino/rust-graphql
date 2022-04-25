use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::token::jwt::make_jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestObj {
    name: String,
    number: i32,
}

pub async fn make_jwt_handler() -> actix_web::Result<HttpResponse> {
    println!("make_jwt_handler");
    let jwt = make_jwt("secret", "aud");
    println!("jwt: {:?}", &jwt);
    Ok(HttpResponse::Ok().json(jwt))
}
