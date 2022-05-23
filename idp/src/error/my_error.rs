use std::fmt;

pub struct MyError {
    cause: Box<dyn std::error::Error>,
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.cause, f)
    }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.cause)
    }
}

/// `MyError` for any error that implements `Error`
impl<T: std::error::Error + 'static> From<T> for MyError {
    fn from(err: T) -> MyError {
        MyError {
            cause: Box::new(err),
        }
    }
}
