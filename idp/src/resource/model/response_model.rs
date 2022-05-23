use serde::Serialize;

use crate::entity::user::User;

#[derive(Serialize)]
pub struct SingInResponse {
    pub user: User,
    pub token: String,
}
