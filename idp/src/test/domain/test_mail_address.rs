#[cfg(test)]
mod tests {
    use crate::domain::mail_address::MailAddress;

    #[test]
    fn it_works() {
        let mail_string = "test.test@gmail.com".to_string();
        let mail = MailAddress::try_from(mail_string);
        assert_eq!(String::from(mail.unwrap()), mail_string);
    }
}
