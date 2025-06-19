
// Backend-only error types and conversions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
    InternalError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// SurrealDB error conversion
impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

// Config error conversion  
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// Conversion from server-side AppError to frontend-safe common Error
impl From<AppError> for crate::common::errors::Error {
    fn from(app_err: AppError) -> Self {
        match app_err {
            AppError::DatabaseError(msg) => crate::common::errors::Error::InternalError(msg),
            AppError::NotFound(msg) => crate::common::errors::Error::NotFound(msg),
            AppError::ValidationError(msg) => crate::common::errors::Error::ValidationError(msg),
            AppError::InternalError(msg) => crate::common::errors::Error::InternalError(msg),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;