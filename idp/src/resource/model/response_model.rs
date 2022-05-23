use serde::Serialize;

use crate::{entity::user::User, token::jwt::Claims};

#[derive(Serialize)]
pub struct SingInResponse {
    pub user: User,
    pub token: String,
}

#[derive(Serialize)]
pub struct TokenValidatedResponse {
    pub claims: Claims,
    pub user: User,
}
