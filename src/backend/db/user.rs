use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::backend::error::Error;
use crate::common::{types, requests};
use validator::Validate;

use super::DB;
const USERS: &str = "users";

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
    req: requests::user::Create,
) -> Result<types::User, Error> {
    DB.create(USERS)
        .content(User {
            id: None,
            email: req.email,
            password_hash: req.password,
            role: types::Role::Staff,
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create user".into()))
}

pub async fn get_user(email: String) -> Result<User, Error> {
    DB.select((USERS, email))
        .await?
        .ok_or_else(|| Error::NotFound("User".into()))
}

pub async fn update_user(
    id: &str,
    update: requests::user::Update,
) -> Result<types::User, Error> {
    DB
        .update((USERS, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update product".into()))
}

pub async fn delete_user(id: &str) -> Result<(), Error> {
    let deleted: Option<User> = DB
        .delete((USERS, id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to delete product with id: {}", id)))?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Product with id {} not found", id)));
    }
    Ok(())
}
