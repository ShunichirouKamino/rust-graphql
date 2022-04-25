//! Idp Resource.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::token::jwt::make_jwt;

// todo making secret
const SECRET: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationReqBody {
    name: String,
    passwd: String,
}

pub async fn make_jwt_handler(
    body: web::Json<AuthenticationReqBody>,
) -> actix_web::Result<HttpResponse> {
    // 認証処理
    let jwt = make_jwt(SECRET, &body.name);
    println!("jwt: {:?}", &jwt);
    Ok(HttpResponse::Ok().json(jwt))
}
