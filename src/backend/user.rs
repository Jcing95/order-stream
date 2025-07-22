use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::{
        auth::{get_extended_expiry, should_extend_session, verify_password, SessionData},
        db::DB,
    };
    pub use crate::common::types;
    pub use leptos::logging::log;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use leptos_axum::extract;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::{Datetime, Thing};
    use surrealdb::RecordId;
    pub use tower_sessions::Session;
    pub use validator::Validate;
    pub const USERS: &str = "users";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct User {
        pub id: Option<RecordId>,
        #[validate(email)]
        pub email: String,
        pub password_hash: String,
        pub role: types::Role,
    }

    impl From<User> for types::User {
        fn from(user: User) -> Self {
            types::User {
                id: user.id.unwrap().key().to_string(),
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
    use crate::backend::auth::hash_password;

    let password_hash = hash_password(&req.password);
    if password_hash.is_err() {
        return Err(ServerError(format!("Password hashing failed.")));
    }
    let password_hash = password_hash.unwrap();

    let u: Option<User> = DB
        .create(USERS)
        .content(User {
            id: None,
            email: req.email,
            password_hash,
            role: types::Role::Staff,
        })
        .await?;
    u.map(Into::into)
        .ok_or_else(|| ServerError("Failed to create user".into()))
}

#[server(GetUser, "/api/user")]
pub async fn get_user(id: String) -> Result<types::User, ServerFnError> {
    DB.select((USERS, id))
        .await?
        .ok_or_else(|| ServerError("User not found".into()))
}

#[cfg(feature = "ssr")]
pub async fn get_user_by_email(email: String) -> Result<Option<User>, surrealdb::Error> {
    let mut result = DB
        .query("SELECT * FROM users WHERE email = $email")
        .bind(("email", email))
        .await?;
    let user: Option<User> = result.take(0)?;
    Ok(user)
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

#[server(Login, "/api/user")]
pub async fn login(email: String, password: String) -> Result<types::User, ServerFnError> {
    log!("Logging in...");
    let session: Session = extract().await?;
    log!("Session: {:?}", session);
    // Find user by email
    let user = get_user_by_email(email.clone()).await
        .map_err(|e| Error::InternalError(format!("Database error: {}", e)))?;
    log!("User: {:?}", user);

    let user = user.ok_or_else(|| Error::NotAuthorized("Invalid credentials".to_string()))?;
    log!("User unwrapped: {:?}", user);

    // Verify password
    if !verify_password(&password, &user.password_hash)? {
        return Err(Error::NotAuthorized("Invalid credentials".to_string()).into());
    }
    log!("password verified!");

    // Create session data
    let session_data = SessionData::new(user.id.as_ref().unwrap().key().to_string());
    log!("Session Data: {:?}", session_data);

    // Store in session
    session.insert("user", session_data).await?;

    // Set session expiry to 24 hours
    session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(
        get_extended_expiry(),
    )));
    log!("expiry set");

    Ok(user.into())
}

#[server(Logout, "/api/user")]
pub async fn logout() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let _ = session.delete().await;
    Ok(())
}

#[server(GetCurrentUser, "/api/user")]
pub async fn get_current_user() -> Result<types::User, ServerFnError> {
    let session: Session = extract().await?;
    log!("Session: {:?}", session);
    crate::backend::auth::get_authenticated_user(&session).await
}
