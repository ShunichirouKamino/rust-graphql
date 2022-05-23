use regex::Regex;
use serde::Serialize;
use std::convert::TryFrom;

/// Value objects are tuple structures because they are one primitive-based.
/// Uniquely identifies a user.
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Serialize)]
pub struct MailAddress(String);

// Constructs a value object following the regular expression of an email address.
impl TryFrom<String> for MailAddress {
    type Error = ();

    fn try_from(email: String) -> Result<Self, Self::Error> {
        let regex = Regex::new(r#"^[a-zA-Z0-9_+-]+(.[a-zA-Z0-9_+-]+)*@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"#).unwrap();
        if regex.is_match(email.as_str()) {
            Ok(Self(email))
        } else {
            Err(())
        }
    }
}

/// MailAddress to String conversion process
impl From<MailAddress> for String {
    fn from(email: MailAddress) -> Self {
        email.0
    }
}
