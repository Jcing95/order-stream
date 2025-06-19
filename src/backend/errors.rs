
use crate::common::errors::{AppError};
use surrealdb;

#[cfg(feature = "ssr")]
impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}