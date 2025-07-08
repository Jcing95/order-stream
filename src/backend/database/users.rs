use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime, Duration};

use crate::backend::errors::{AppError, AppResult};
use crate::common::types::{User, UserRole};

use super::{Database, validators};

#[cfg(feature = "ssr")]
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct UserRecord {
    pub id: Thing,
    #[cfg_attr(feature = "ssr", validate(email, length(min = 1, max = 255)))]
    pub email: String,
    #[cfg_attr(feature = "ssr", validate(length(min = 1)))]
    pub password_hash: String,
    pub role: UserRole,
    pub active: bool,
    pub failed_login_attempts: u32,
    pub locked_until: Option<Datetime>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub last_login_at: Option<Datetime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct SessionRecord {
    pub id: Thing,
    pub user_id: Thing,  // Proper SurrealDB relation
    #[cfg_attr(feature = "ssr", validate(length(min = 1)))]
    pub session_token: String,
    pub expires_at: Datetime,
    pub created_at: Datetime,
    pub last_activity: Datetime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub revoked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttemptRecord {
    pub id: Thing,
    pub email: String,
    pub ip_address: String,
    pub success: bool,
    pub attempted_at: Datetime,
    pub user_agent: Option<String>,
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

pub async fn create_user(
    db: &Database,
    email: String,
    password_hash: String,
    role: UserRole,
) -> AppResult<UserRecord> {
    #[cfg(feature = "ssr")]
    {
        // Validate email format
        validators::email_format(&email)?;
    }

    let user: Option<UserRecord> = db
        .create("users")
        .content(UserRecord {
            id: Thing::from(("users", "")), // Let SurrealDB auto-generate ID
            email,
            password_hash,
            role,
            active: true,
            failed_login_attempts: 0,
            locked_until: None,
            created_at: Datetime::default(), // Uses SurrealDB's time::now()
            updated_at: Datetime::default(),
            last_login_at: None,
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create user: {}", e)))?;

    user.ok_or_else(|| AppError::InternalError("Failed to create user: no record returned from database".to_string()))
}

pub async fn get_user_by_email(
    db: &Database,
    email: &str,
) -> AppResult<Option<UserRecord>> {
    let mut result = db
        .query("SELECT * FROM users WHERE email = $email AND active = true AND (locked_until IS NONE OR locked_until < time::now())")
        .bind(("email", email.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user by email: {}", e)))?;

    let users: Vec<UserRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse user query result: {}", e)))?;
    
    Ok(users.into_iter().next())
}

pub async fn get_user_by_id(
    db: &Database,
    user_id: &Thing,
) -> AppResult<Option<UserRecord>> {
    let mut result = db
        .query("SELECT * FROM $user_id")
        .bind(("user_id", user_id.clone()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user by id: {}", e)))?;

    let users: Vec<UserRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse user query result: {}", e)))?;
    
    Ok(users.into_iter().next())
}

pub async fn create_session(
    db: &Database,
    user_id: Thing,
    token: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<SessionRecord> {
    // Use SurrealDB query with time functions for 1 week expiry
    let mut result = db
        .query("CREATE sessions CONTENT {
            user_id: $user_id,
            session_token: $session_token,
            expires_at: time::now() + 1w,
            created_at: time::now(),
            last_activity: time::now(),
            ip_address: $ip_address,
            user_agent: $user_agent,
            revoked: false
        }")
        .bind(("user_id", user_id))
        .bind(("session_token", token))
        .bind(("ip_address", ip_address))
        .bind(("user_agent", user_agent))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create session: {}", e)))?;

    let sessions: Vec<SessionRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse session query result: {}", e)))?;
    
    sessions.into_iter().next().ok_or_else(|| {
        AppError::InternalError("Failed to create session: no record returned from database".to_string())
    })
}

pub async fn get_session_by_token(
    db: &Database,
    token: &str,
) -> AppResult<Option<SessionRecord>> {
    let mut result = db
        .query("SELECT * FROM sessions WHERE session_token = $session_token AND expires_at > time::now() AND revoked = false")
        .bind(("session_token", token.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get session by token: {}", e)))?;

    let sessions: Vec<SessionRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse session query result: {}", e)))?;
    
    Ok(sessions.into_iter().next())
}

pub async fn delete_session(
    db: &Database,
    token: &str,
) -> AppResult<()> {
    db.query("DELETE FROM sessions WHERE session_token = $session_token")
        .bind(("session_token", token.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete session: {}", e)))?;

    Ok(())
}

pub async fn cleanup_expired_sessions(
    db: &Database,
) -> AppResult<()> {
    // Delete sessions that are older than 30 days to maintain audit trail
    db.query("DELETE FROM sessions WHERE expires_at <= time::now() - 30d")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to cleanup expired sessions: {}", e)))?;

    Ok(())
}

// New functions for enhanced security

pub async fn record_login_attempt(
    db: &Database,
    email: String,
    ip_address: String,
    success: bool,
    user_agent: Option<String>,
) -> AppResult<LoginAttemptRecord> {
    let attempt: Option<LoginAttemptRecord> = db
        .create("login_attempts")
        .content(LoginAttemptRecord {
            id: Thing::from(("login_attempts", "")), // Let SurrealDB auto-generate ID
            email,
            ip_address,
            success,
            attempted_at: Datetime::default(),
            user_agent,
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to record login attempt: {}", e)))?;

    attempt.ok_or_else(|| {
        AppError::InternalError("Failed to record login attempt: no record returned from database".to_string())
    })
}

pub async fn get_failed_login_attempts(
    db: &Database,
    email: &str,
    since: Duration,
) -> AppResult<Vec<LoginAttemptRecord>> {
    let mut result = db
        .query("SELECT * FROM login_attempts WHERE email = $email AND success = false AND attempted_at > time::now() - $since ORDER BY attempted_at DESC")
        .bind(("email", email.to_string()))
        .bind(("since", since))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get failed login attempts: {}", e)))?;

    let attempts: Vec<LoginAttemptRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse login attempts query result: {}", e)))?;
    
    Ok(attempts)
}

pub async fn get_failed_login_attempts_by_ip(
    db: &Database,
    ip_address: &str,
    since: Duration,
) -> AppResult<Vec<LoginAttemptRecord>> {
    let mut result = db
        .query("SELECT * FROM login_attempts WHERE ip_address = $ip_address AND success = false AND attempted_at > time::now() - $since ORDER BY attempted_at DESC")
        .bind(("ip_address", ip_address.to_string()))
        .bind(("since", since))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get failed login attempts by IP: {}", e)))?;

    let attempts: Vec<LoginAttemptRecord> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse login attempts query result: {}", e)))?;
    
    Ok(attempts)
}

pub async fn lock_user_account(
    db: &Database,
    user_id: Thing,
    lock_duration_seconds: u64,
) -> AppResult<()> {
    let duration = Duration::from_secs(lock_duration_seconds);
    db.query("UPDATE $user_id SET locked_until = time::now() + $lock_duration, failed_login_attempts = failed_login_attempts + 1")
        .bind(("user_id", user_id))
        .bind(("lock_duration", duration))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to lock user account: {}", e)))?;

    Ok(())
}

pub async fn reset_failed_login_attempts(
    db: &Database,
    user_id: Thing,
) -> AppResult<()> {
    db.query("UPDATE $user_id SET failed_login_attempts = 0, locked_until = NONE, last_login_at = time::now()")
        .bind(("user_id", user_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to reset failed login attempts: {}", e)))?;

    Ok(())
}

pub async fn update_session_activity(
    db: &Database,
    session_token: &str,
) -> AppResult<()> {
    db.query("UPDATE sessions SET last_activity = time::now() WHERE session_token = $session_token")
        .bind(("session_token", session_token.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update session activity: {}", e)))?;

    Ok(())
}

pub async fn revoke_all_user_sessions(
    db: &Database,
    user_id: Thing,
) -> AppResult<()> {
    db.query("UPDATE sessions SET revoked = true WHERE user_id = $user_id")
        .bind(("user_id", user_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to revoke user sessions: {}", e)))?;

    Ok(())
}

pub async fn get_active_sessions_count(
    db: &Database,
    user_id: Thing,
) -> AppResult<usize> {
    let mut result = db
        .query("SELECT count() FROM sessions WHERE user_id = $user_id AND expires_at > time::now() AND revoked = false GROUP ALL")
        .bind(("user_id", user_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get active sessions count: {}", e)))?;

    let count: Option<usize> = result.take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse sessions count query result: {}", e)))?;
    
    Ok(count.unwrap_or(0))
}

// Initialize database schema and indexes
pub async fn initialize_schema(
    db: &Database,
) -> AppResult<()> {
    // Create indexes for performance
    db.query("DEFINE INDEX idx_users_email ON users COLUMNS email UNIQUE").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create users email index: {}", e)))?;
    db.query("DEFINE INDEX idx_users_active ON users COLUMNS active").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create users active index: {}", e)))?;
    db.query("DEFINE INDEX idx_sessions_token ON sessions COLUMNS session_token UNIQUE").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create sessions token index: {}", e)))?;
    db.query("DEFINE INDEX idx_sessions_user_id ON sessions COLUMNS user_id").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create sessions user_id index: {}", e)))?;
    db.query("DEFINE INDEX idx_sessions_expires_at ON sessions COLUMNS expires_at").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create sessions expires_at index: {}", e)))?;
    db.query("DEFINE INDEX idx_sessions_revoked ON sessions COLUMNS revoked").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create sessions revoked index: {}", e)))?;
    db.query("DEFINE INDEX idx_login_attempts_email ON login_attempts COLUMNS email").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create login_attempts email index: {}", e)))?;
    db.query("DEFINE INDEX idx_login_attempts_ip ON login_attempts COLUMNS ip_address").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create login_attempts ip index: {}", e)))?;
    db.query("DEFINE INDEX idx_login_attempts_time ON login_attempts COLUMNS attempted_at").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create login_attempts time index: {}", e)))?;
    
    // Define database-level constraints
    db.query("DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value) AND string::len($value) <= 255").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users email field: {}", e)))?;
    db.query("DEFINE FIELD password_hash ON users TYPE string ASSERT string::len($value) > 0").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users password_hash field: {}", e)))?;
    db.query("DEFINE FIELD role ON users TYPE string ASSERT $value IN ['Admin', 'Cashier', 'Staff']").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users role field: {}", e)))?;
    db.query("DEFINE FIELD active ON users TYPE bool").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users active field: {}", e)))?;
    db.query("DEFINE FIELD failed_login_attempts ON users TYPE number DEFAULT 0").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users failed_login_attempts field: {}", e)))?;
    db.query("DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now()").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users created_at field: {}", e)))?;
    db.query("DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now()").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define users updated_at field: {}", e)))?;
    
    db.query("DEFINE FIELD user_id ON sessions TYPE record(users)").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions user_id field: {}", e)))?;
    db.query("DEFINE FIELD session_token ON sessions TYPE string ASSERT string::len($value) > 0").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions session_token field: {}", e)))?;
    db.query("DEFINE FIELD expires_at ON sessions TYPE datetime").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions expires_at field: {}", e)))?;
    db.query("DEFINE FIELD created_at ON sessions TYPE datetime DEFAULT time::now()").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions created_at field: {}", e)))?;
    db.query("DEFINE FIELD last_activity ON sessions TYPE datetime DEFAULT time::now()").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions last_activity field: {}", e)))?;
    db.query("DEFINE FIELD revoked ON sessions TYPE bool DEFAULT false").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define sessions revoked field: {}", e)))?;
    
    db.query("DEFINE FIELD email ON login_attempts TYPE string ASSERT string::is::email($value)").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define login_attempts email field: {}", e)))?;
    db.query("DEFINE FIELD ip_address ON login_attempts TYPE string ASSERT string::len($value) > 0").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define login_attempts ip_address field: {}", e)))?;
    db.query("DEFINE FIELD success ON login_attempts TYPE bool").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define login_attempts success field: {}", e)))?;
    db.query("DEFINE FIELD attempted_at ON login_attempts TYPE datetime DEFAULT time::now()").await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define login_attempts attempted_at field: {}", e)))?;
    
    Ok(())
}

// Exponential backoff calculation (returns seconds)
pub fn calculate_lockout_duration_seconds(failed_attempts: u32) -> u64 {
    match failed_attempts {
        0..=2 => 0,                          // No lockout for first 3 attempts
        3 => 60,                             // 1 minute
        4 => 300,                            // 5 minutes
        5 => 900,                            // 15 minutes
        6 => 3600,                           // 1 hour
        7 => 14400,                          // 4 hours
        8 => 43200,                          // 12 hours
        _ => 86400,                          // 1 day for 9+ attempts
    }
}
