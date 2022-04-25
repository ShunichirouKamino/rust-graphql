//! Idp Resource.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::token::jwt::make_jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationReqBody {
    name: String,
    passwd: String,
}

pub async fn make_jwt_handler(
    body: web::Json<AuthenticationReqBody>,
) -> actix_web::Result<HttpResponse> {
    // 認証処理
    let jwt = make_jwt("secret", "aud");
    println!("jwt: {:?}", &jwt);
    Ok(HttpResponse::Ok().json(jwt))
}
