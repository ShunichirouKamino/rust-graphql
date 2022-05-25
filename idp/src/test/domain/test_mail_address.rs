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
    fn test_mail_from_str_ok() {
        let mail_string = "test.test@gmail.com";
        let result = MailAddress::of(mail_string);
        assert_eq!(String::from(result.unwrap()), mail_string.to_owned());
    }

    #[test]
    fn test_mail_from_string_ok() {
        let mail_string = "test.test@gmail.com".to_string();
        let result = MailAddress::of(mail_string.clone());
        assert_eq!(String::from(result.unwrap()), mail_string);
    }

    #[test]
    fn test_mail_ng() {
        let mail_string = "test.test@@@gmail.com".to_string();
        let result = MailAddress::try_from(mail_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_mail_ng_from_str() {
        let mail_string = "test.test@@@gmail.com";
        let result = MailAddress::of(mail_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_equals() {
        let mail_string_1 = "fuga.test@gmail.com";
        let mail_string_2 = "fuga.test@gmail.com";
        let mail_string_3 = "hoge.test@gmail.com";

        let mail_1 = MailAddress::of(mail_string_1).unwrap();
        let mail_2 = MailAddress::of(mail_string_2).unwrap();
        let mail_3 = MailAddress::of(mail_string_3).unwrap();

        assert_eq!(mail_1, mail_2);
        assert_ne!(mail_2, mail_3);
        assert_ne!(mail_1, mail_3);
    }
}
