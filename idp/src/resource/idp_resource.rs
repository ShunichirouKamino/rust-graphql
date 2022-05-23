//! Idp Resource.

use crate::domain::mail_address::MailAddress;
use crate::entity::user::User;
use crate::resource::model::response_model::SingInResponse;
use crate::token::jwt::{decode_jwt, make_jwt};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

// todo making secret
const SECRET: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationReqBody {
    email: String,
    passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationReqBody {
    email: String,
    token: String,
}

pub async fn make_jwt_handler(
    body: web::Json<AuthenticationReqBody>,
) -> actix_web::Result<HttpResponse> {
    // todo authentication
    let mail = MailAddress::try_from(body.email.clone()).unwrap();
    let jwt = make_jwt(SECRET, &mail);
    println!("jwt: {:?}", &jwt);
    let res = SingInResponse {
        user: User::of(mail),
        token: jwt.unwrap(),
    };
    Ok(HttpResponse::Ok().json(res))
}

pub async fn validate_jwt_handler(
    body: web::Json<AuthorizationReqBody>,
) -> actix_web::Result<HttpResponse> {
    let claims = decode_jwt(SECRET, &body.token, &body.email);
    Ok(HttpResponse::Ok().json(claims))
}
