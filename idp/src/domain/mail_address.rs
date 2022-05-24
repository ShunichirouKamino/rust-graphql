use regex::Regex;
use serde::Serialize;
use std::convert::TryFrom;

use crate::error::my_error::{self, MyError};

/// Value objects are tuple structures because they are one primitive-based.
/// Uniquely identifies a user.
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Serialize)]
pub struct MailAddress {
    mail_string: String,
}

// Constructs a value object following the regular expression of an email address.
impl TryFrom<String> for MailAddress {
    type Error = MyError;

    fn try_from(mail_string: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(mail_string.as_str()) {
            Ok(Self { mail_string })
        } else {
            Err(my_error::MyError::InvalidValue)
        }
    }
}

// impl MailAddress {
//     fn new<T: Into<String>>(mail_string: T) -> Result<Self, MyError> {
//         MailAddress::try_from(mail_string.into())?;
//     }
// }

/// MailAddress to String conversion process
impl From<MailAddress> for String {
    fn from(email: MailAddress) -> Self {
        email.mail_string
    }
}
