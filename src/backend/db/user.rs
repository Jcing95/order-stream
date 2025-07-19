use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::Database;
use crate::backend::error::Error;
use crate::common::types;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Option<Thing>,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub role: types::Role,
}

impl From<User> for types::User {
    fn from(user: User) -> Self {
        types::User {
            id: user.id.unwrap().id.to_string(),
            email: user.email,
            role: user.role,
        }
    }
}

pub async fn create_user(
    db: &Database,
    email: String,
    password_hash: String,
    role: types::Role,
) -> Result<types::User, Error> {
    db.create("user")
        .content(User {
            id: None,
            email,
            password_hash,
            role,
        })
        .await?
        .ok_or_else(|| Error::InternalError(format!("Failed to create User.")))
}

pub async fn get_user(db: &Database, email: String) -> Result<User, Error> {
    db.select(("user", email))
        .await?
        .ok_or_else(|| Error::NotFound("User".into()))
}
