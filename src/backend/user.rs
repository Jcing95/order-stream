use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::ssr::*;
    pub const USERS: &str = "users";

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
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateUser, "/api/user")]
pub async fn create_user(req: requests::user::Create) -> Result<types::User, ServerFnError> {
    DB.create(USERS)
        .content(User {
            id: None,
            email: req.email,
            password_hash: req.password,
            role: types::Role::Staff,
        })
        .await?
        .ok_or_else(|| ServerError("Failed to create user".into()))
}

#[server(GetUser, "/api/user")]
pub async fn get_user(email: String) -> Result<types::User, ServerFnError> {
    DB.select((USERS, email))
        .await?
        .ok_or_else(|| ServerError("User not found".into()))
}

#[server(UpdateUser, "/api/user")]
pub async fn update_user(
    id: String,
    update: requests::user::Update,
) -> Result<types::User, ServerFnError> {
    DB.update((USERS, &id))
        .merge(update)
        .await?
        .ok_or_else(|| ServerError("Failed to update user".into()))
}

#[server(DeleteUser, "/api/user")]
pub async fn delete_user(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<User> = DB.delete((USERS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("User with id {} not found", id)));
    }
    Ok(())
}
