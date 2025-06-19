use serde::{Deserialize, Serialize};

// Frontend-safe error types that can be shared between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    NotFound(String),
    ValidationError(String),
    InternalError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}