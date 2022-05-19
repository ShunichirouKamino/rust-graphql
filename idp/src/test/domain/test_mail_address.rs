#[cfg(test)]
mod tests {
    use crate::domain::mail_address::MailAddress;

    #[test]
    fn test_mail_ok() {
        let mail_string = "test.test@gmail.com".to_string();
        let result = MailAddress::try_from(mail_string.clone());
        assert_eq!(String::from(result.unwrap()), mail_string);
    }

    #[test]
    fn test_mail_ng() {
        let mail_string = "test.test@@@gmail.com".to_string();
        let result = MailAddress::try_from(mail_string.clone());
        assert!(result.is_err());
    }
}
