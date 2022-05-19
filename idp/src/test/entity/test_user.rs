#[cfg(test)]
mod tests {
    use crate::{domain::mail_address::MailAddress, entity::user::User};

    #[test]
    fn it_works() {
        let mail_string = "test.test@gmail.com".to_string();
        let mail = MailAddress::try_from(mail_string.clone());
        let user = User::new(mail.unwrap());
        assert_eq!(String::from(user.email), mail_string);
    }
}
