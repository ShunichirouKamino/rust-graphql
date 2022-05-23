use serde::Serialize;

use crate::domain::mail_address::MailAddress;

/// Entities consist of classic structures.
/// Represents a mutable object.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct User {
    pub email: MailAddress,
}

// Factory that instantiates from field values
impl User {
    pub fn of(email: MailAddress) -> Self {
        Self { email }
    }
}
