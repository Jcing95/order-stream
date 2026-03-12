use crate::backend::db::get_pool;
use crate::common::errors::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};
use uuid::Uuid;

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

fn get_extended_expiry_chrono() -> chrono::DateTime<Utc> {
    Utc::now() + Duration::hours(24)
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

#[derive(Debug, Clone)]
pub struct PostgresSessionStore;

impl PostgresSessionStore {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SessionStore for PostgresSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        use crate::backend::schema::sessions::dsl::*;
        use crate::backend::models::NewSession;

        let user_id_val: Uuid = extract_user_id(record)
            .map_err(|e| session_store::Error::Backend(e))?;

        let new_session = NewSession {
            id: record.id.to_string(),
            user_id: user_id_val,
            expiry_date: get_extended_expiry_chrono(),
        };

        let pool = get_pool();
        let mut conn = pool.get().await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        diesel::insert_into(sessions)
            .values(&new_session)
            .execute(&mut conn)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        use crate::backend::schema::sessions::dsl::*;

        let user_id_val: Uuid = extract_user_id(record)
            .map_err(|e| session_store::Error::Backend(e))?;

        let pool = get_pool();
        let mut conn = pool.get().await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        diesel::update(sessions.filter(id.eq(record.id.to_string())))
            .set((
                user_id.eq(user_id_val),
                expiry_date.eq(get_extended_expiry_chrono()),
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        use crate::backend::schema::sessions::dsl::*;
        use crate::backend::models::DbSession;

        let pool = get_pool();
        let mut conn = pool.get().await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        let session: Option<DbSession> = sessions
            .filter(id.eq(session_id.to_string()))
            .filter(expiry_date.gt(diesel::dsl::now))
            .select(DbSession::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match session {
            Some(s) => {
                let mut data = std::collections::HashMap::new();
                let session_data = SessionData::new(s.user_id.to_string());
                data.insert(
                    "user".to_string(),
                    serde_json::to_value(session_data)
                        .map_err(|e| session_store::Error::Backend(e.to_string()))?,
                );

                let expiry = OffsetDateTime::from_unix_timestamp(s.expiry_date.timestamp())
                    .unwrap_or_else(|_| get_extended_expiry());

                Ok(Some(Record {
                    id: session_id.clone(),
                    data,
                    expiry_date: expiry,
                }))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        use crate::backend::schema::sessions::dsl::*;

        let pool = get_pool();
        let mut conn = pool.get().await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        diesel::delete(sessions.filter(id.eq(session_id.to_string())))
            .execute(&mut conn)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}

fn extract_user_id(record: &Record) -> Result<Uuid, String> {
    let session_data: SessionData = record
        .data
        .get("user")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| "Missing user session data".to_string())?;

    Uuid::parse_str(&session_data.user_id)
        .map_err(|e| format!("Invalid user UUID in session: {}", e))
}

pub async fn get_authenticated_user(
    session: &tower_sessions::Session,
) -> Result<crate::common::types::User, leptos::prelude::ServerFnError> {
    use crate::backend::models::{DbUser, ParseRole};
    use crate::backend::schema::users::dsl::*;

    let session_data: Option<SessionData> = session.get("user").await?;
    let session_data =
        session_data.ok_or_else(|| Error::NotAuthorized("Not authenticated".to_string()))?;

    handle_session_extension(session);

    let user_uuid = Uuid::parse_str(&session_data.user_id)
        .map_err(|_| Error::NotAuthorized("Invalid session".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let user: Option<DbUser> = users
        .filter(id.eq(user_uuid))
        .select(DbUser::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let user = user.ok_or_else(|| {
        let _ = session.delete();
        Error::NotAuthorized("User not found".to_string())
    })?;

    Ok(crate::common::types::User {
        id: user.id.to_string(),
        email: user.email,
        role: user.role.parse_role(),
    })
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
