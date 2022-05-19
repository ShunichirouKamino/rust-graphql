use crate::domain::mail_address::MailAddress;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct User {
    pub email: MailAddress,
}

impl User {
    // User のコンストラクタ
    pub fn new(email: MailAddress) -> Self {
        Self { email }
    }
}
