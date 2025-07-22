use crate::backend::db::DB;
use crate::common::errors::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::RecordId;
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| Error::InternalError(format!("Password hashing failed: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| Error::InternalError(format!("Invalid password hash: {}", e)))?;

    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: String,
}

impl SessionData {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

pub fn should_extend_session(expiry_date: OffsetDateTime) -> bool {
    let now = OffsetDateTime::now_utc();
    let twelve_hours = time::Duration::hours(12);
    expiry_date - now < twelve_hours
}

pub fn get_extended_expiry() -> OffsetDateTime {
    OffsetDateTime::now_utc() + time::Duration::hours(24)
}

#[cfg(feature = "ssr")]
pub fn handle_session_extension(session: &tower_sessions::Session) {
    if let Some(expiry) = session.expiry() {
        if let tower_sessions::Expiry::AtDateTime(expiry_time) = expiry {
            if should_extend_session(expiry_time) {
                session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(
                    get_extended_expiry(),
                )));
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionRecord {
    id: Option<RecordId>,
    session_id: String,
    data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SurrealSessionStore;

impl SurrealSessionStore {
    pub fn new() -> Self {
        Self
    }
}

const SESSIONS_TABLE: &str = "sessions";

#[async_trait]
impl SessionStore for SurrealSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let session_record = SessionRecord {
            id: None,
            session_id: record.id.to_string(),
            data: record.data.clone(),
        };

        let _: Option<SessionRecord> = DB
            .create((SESSIONS_TABLE, &record.id.to_string()))
            .content(session_record)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let session_record = SessionRecord {
            id: None,
            session_id: record.id.to_string(),
            data: record.data.clone(),
        };

        let _: Option<SessionRecord> = DB
            .update((SESSIONS_TABLE, &record.id.to_string()))
            .content(session_record)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let session_record: Option<SessionRecord> = DB
            .select((SESSIONS_TABLE, &session_id.to_string()))
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match session_record {
            Some(record) => Ok(Some(Record {
                id: session_id.clone(),
                data: record.data,
                expiry_date: OffsetDateTime::now_utc() + time::Duration::days(30),
            })),
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let deleted: Option<SessionRecord> = DB
            .delete((SESSIONS_TABLE, &session_id.to_string()))
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;
        if deleted.is_none() {
            return Err(session_store::Error::Backend(
                "Could not delete Session.".into(),
            ));
        }
        Ok(())
    }
}

pub async fn get_authenticated_user(
    session: &tower_sessions::Session,
) -> Result<crate::common::types::User, leptos::prelude::ServerFnError> {
    use crate::backend::user::ssr::{User, USERS};
    use leptos::logging::log;
    log!("authenticating user...");
    let session_data: Option<SessionData> = session.get("user").await?;

    let session_data =
        session_data.ok_or_else(|| Error::NotAuthorized("Not authenticated".to_string()))?;
    log!("Session data {:?}", session_data);

    handle_session_extension(session);
    log!("Handled session extension...");

    let user: Option<User> = DB.select((USERS, &session_data.user_id)).await?;
    log!("User data {:?}", user);

    let user = user.ok_or_else(|| {
        let _ = session.delete();
        Error::NotAuthorized("User not found".to_string())
    })?;
    log!("returning authenticated user...");
    Ok(user.into())
}

#[macro_export]
macro_rules! requireUserOrRole {
    ($user_id:expr $(, $role:ident)*) => {
        let session: tower_sessions::Session = leptos_axum::extract().await?;
        let current_user = $crate::backend::auth::get_authenticated_user(&session).await?;

        let is_target_user = current_user.id == $user_id;
        let has_required_role = false $(|| current_user.role == $crate::common::types::Role::$role)*;

        if !is_target_user && !has_required_role {
            return Err($crate::common::errors::Error::NotAuthorized("Access denied: must be the user or have required role".to_string()).into());
        }
    };
}

#[macro_export]
macro_rules! roles {
    (Public) => {
        // No auth check for public endpoints
    };
    ($($role:ident),+) => {
        let session: tower_sessions::Session = leptos_axum::extract().await?;
        let current_user = $crate::backend::auth::get_authenticated_user(&session).await?;

        let has_required_role = $(current_user.role == $crate::common::types::Role::$role)||+ || current_user.role == $crate::common::types::Role::Admin;

        if !has_required_role {
            return Err($crate::common::errors::Error::NotAuthorized("Insufficient permissions".to_string()).into());
        }
    };
}
