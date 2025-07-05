use surrealdb::{Connection, Surreal};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::common::types::{User, UserRole};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: surrealdb::sql::Thing,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: surrealdb::sql::Thing,
    pub user_id: String,
    pub session_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        User {
            id: record.id.to_string(),
            email: record.email,
            role: record.role,
            active: record.active,
        }
    }
}

pub async fn create_user<C: Connection>(
    db: &Surreal<C>,
    email: String,
    password_hash: String,
    role: UserRole,
) -> Result<UserRecord, surrealdb::Error> {
    let now = Utc::now();

    let user: Option<UserRecord> = db
        .create("users")
        .content(UserRecord {
            id: surrealdb::sql::Thing::from(("users", surrealdb::sql::Id::rand())),
            email,
            password_hash,
            role,
            active: true,
            created_at: now,
            updated_at: now,
        })
        .await?;

    user.ok_or_else(|| {
        surrealdb::Error::Api(surrealdb::error::Api::Query(
            "Failed to create user".to_string(),
        ))
    })
}

pub async fn get_user_by_email<C: Connection>(
    db: &Surreal<C>,
    email: &str,
) -> Result<Option<UserRecord>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM users WHERE email = $email AND active = true")
        .bind(("email", email.to_string()))
        .await?;

    let users: Vec<UserRecord> = result.take(0)?;
    Ok(users.into_iter().next())
}

pub async fn get_user_by_id<C: Connection>(
    db: &Surreal<C>,
    user_id: &str,
) -> Result<Option<UserRecord>, surrealdb::Error> {
    let user: Option<UserRecord> = db.select(("users", user_id)).await?;

    Ok(user)
}

pub async fn create_session<C: Connection>(
    db: &Surreal<C>,
    user_id: String,
    token: String,
    expires_at: DateTime<Utc>,
) -> Result<SessionRecord, surrealdb::Error> {
    let session: Option<SessionRecord> = db
        .create("sessions")
        .content(SessionRecord {
            id: surrealdb::sql::Thing::from(("sessions", surrealdb::sql::Id::rand())),
            user_id,
            session_token: token,
            expires_at,
            created_at: Utc::now(),
        })
        .await?;

    session.ok_or_else(|| {
        surrealdb::Error::Api(surrealdb::error::Api::Query(
            "Failed to create session".to_string(),
        ))
    })
}

pub async fn get_session_by_token<C: Connection>(
    db: &Surreal<C>,
    token: &str,
) -> Result<Option<SessionRecord>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM sessions WHERE session_token = $session_token AND expires_at > $now")
        .bind(("session_token", token.to_string()))
        .bind(("now", Utc::now()))
        .await?;

    let sessions: Vec<SessionRecord> = result.take(0)?;
    Ok(sessions.into_iter().next())
}

pub async fn delete_session<C: Connection>(
    db: &Surreal<C>,
    token: &str,
) -> Result<(), surrealdb::Error> {
    db.query("DELETE FROM sessions WHERE session_token = $session_token")
        .bind(("session_token", token.to_string()))
        .await?;

    Ok(())
}

pub async fn cleanup_expired_sessions<C: Connection>(
    db: &Surreal<C>,
) -> Result<(), surrealdb::Error> {
    db.query("DELETE FROM sessions WHERE expires_at <= $now")
        .bind(("now", Utc::now()))
        .await?;

    Ok(())
}
