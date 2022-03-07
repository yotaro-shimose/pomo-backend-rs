use std::fmt;
#[derive(Debug)]
pub enum LambdaServerError {
    InternalServerError(String),
    BadRequest(String),
}

impl fmt::Display for LambdaServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::InternalServerError(msg) => write!(f, "InternalServerError: {}", msg),
            Self::BadRequest(msg) => write!(f, "BadRequest: {}", msg),
        }
    }
}

impl std::error::Error for LambdaServerError {}
