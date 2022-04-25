//! Idp Resource.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::token::jwt::{decode_jwt, make_jwt};

// todo making secret
const SECRET: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationReqBody {
    name: String,
    passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationReqBody {
    token: String,
}

pub async fn make_jwt_handler(
    body: web::Json<AuthenticationReqBody>,
) -> actix_web::Result<HttpResponse> {
    // todo authentication
    let jwt = make_jwt(SECRET, &body.name);
    println!("jwt: {:?}", &jwt);
    Ok(HttpResponse::Ok().json(jwt))
}

pub async fn validate_jwt_handler(
    body: web::Json<AuthorizationReqBody>,
) -> actix_web::Result<HttpResponse> {
    let claims = decode_jwt(SECRET, &body.token);
    Ok(HttpResponse::Ok().json(claims))
}
