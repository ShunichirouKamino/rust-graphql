use std::{error::Error, fmt};

#[derive(Debug)]
pub enum MyError {
    Decode,
    Encode,
    InvalidValue,
}

impl MyError {}

impl Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::InvalidValue => "Invalid Value Error",
            MyError::Decode => "Decode Error",
            MyError::Encode => "Encode Error",
        }
    }
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::InvalidValue => f.write_str("Invalid Value Error"),
            MyError::Decode => f.write_str("Decode Error"),
            MyError::Encode => f.write_str("Encode Error"),
        }
    }
}
