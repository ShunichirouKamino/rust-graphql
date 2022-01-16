pub trait ValidationStrategy {
    fn validate(target: &str) -> Result<String, ValidationError>;
}

pub enum ValidationError {
    NotAllowedEmpty,
    InvalidFormat(String),
}
