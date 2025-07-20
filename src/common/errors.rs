use serde::{Deserialize, Serialize};

// Frontend-safe error types that can be shared between client and server
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
            Error::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(feature = "ssr")]
impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        Error::InternalError(error.to_string())
    }
}