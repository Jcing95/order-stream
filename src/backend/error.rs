
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    NotFound(String),
    NotAuthorized(String),
    InternalError(String),
    InvalidArgument(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::NotAuthorized(msg) => write!(f, "Not authorized: {}", msg),
            Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Error::InvalidArgument(msg) => write!(f, "Invalid Argument: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::InternalError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::InternalError(err.to_string())
    }
}

impl From<Error> for crate::common::errors::Error {
    fn from(app_err: Error) -> Self {
        match app_err {
            Error::NotAuthorized(msg) => crate::common::errors::Error::NotAuthorized(msg),
            Error::NotFound(msg) => crate::common::errors::Error::NotFound(msg),
            Error::InternalError(msg) => crate::common::errors::Error::InternalError(msg),
            Error::InvalidArgument(msg) => crate::common::errors::Error::InvalidArgument(msg),
        }
    }
}

