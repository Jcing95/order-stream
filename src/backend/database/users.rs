use surrealdb::{Connection, Surreal, sql::{Thing, Datetime, Duration}};
use serde::{Deserialize, Serialize};
use crate::common::types::{User, UserRole};

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

pub async fn create_user<C: Connection>(
    db: &Surreal<C>,
    email: String,
    password_hash: String,
    role: UserRole,
) -> Result<UserRecord, surrealdb::Error> {
    #[cfg(feature = "ssr")]
    {
        // Validate email format
        if !email.contains('@') || email.len() > 255 {
            return Err(surrealdb::Error::Api(surrealdb::error::Api::Query(
                "Invalid email format".to_string(),
            )));
        }
    }

    let user: Option<UserRecord> = db
        .create("users")
        .content(UserRecord {
            id: Thing::from(("users", surrealdb::sql::Id::rand())),
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
        .query("SELECT * FROM users WHERE email = $email AND active = true AND (locked_until IS NONE OR locked_until < time::now())")
        .bind(("email", email.to_string()))
        .await?;

    let users: Vec<UserRecord> = result.take(0)?;
    Ok(users.into_iter().next())
}

pub async fn get_user_by_id<C: Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
) -> Result<Option<UserRecord>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM $user_id")
        .bind(("user_id", user_id.clone()))
        .await?;

    let users: Vec<UserRecord> = result.take(0)?;
    Ok(users.into_iter().next())
}

pub async fn create_session<C: Connection>(
    db: &Surreal<C>,
    user_id: Thing,
    token: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<SessionRecord, surrealdb::Error> {
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
        .await?;

    let sessions: Vec<SessionRecord> = result.take(0)?;
    sessions.into_iter().next().ok_or_else(|| {
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
        .query("SELECT * FROM sessions WHERE session_token = $session_token AND expires_at > time::now() AND revoked = false")
        .bind(("session_token", token.to_string()))
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
    // Delete sessions that are older than 30 days to maintain audit trail
    db.query("DELETE FROM sessions WHERE expires_at <= time::now() - 30d")
        .await?;

    Ok(())
}

// New functions for enhanced security

pub async fn record_login_attempt<C: Connection>(
    db: &Surreal<C>,
    email: String,
    ip_address: String,
    success: bool,
    user_agent: Option<String>,
) -> Result<LoginAttemptRecord, surrealdb::Error> {
    let attempt: Option<LoginAttemptRecord> = db
        .create("login_attempts")
        .content(LoginAttemptRecord {
            id: Thing::from(("login_attempts", surrealdb::sql::Id::rand())),
            email,
            ip_address,
            success,
            attempted_at: Datetime::default(),
            user_agent,
        })
        .await?;

    attempt.ok_or_else(|| {
        surrealdb::Error::Api(surrealdb::error::Api::Query(
            "Failed to record login attempt".to_string(),
        ))
    })
}

pub async fn get_failed_login_attempts<C: Connection>(
    db: &Surreal<C>,
    email: &str,
    since: Duration,
) -> Result<Vec<LoginAttemptRecord>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM login_attempts WHERE email = $email AND success = false AND attempted_at > time::now() - $since ORDER BY attempted_at DESC")
        .bind(("email", email.to_string()))
        .bind(("since", since))
        .await?;

    let attempts: Vec<LoginAttemptRecord> = result.take(0)?;
    Ok(attempts)
}

pub async fn get_failed_login_attempts_by_ip<C: Connection>(
    db: &Surreal<C>,
    ip_address: &str,
    since: Duration,
) -> Result<Vec<LoginAttemptRecord>, surrealdb::Error> {
    let mut result = db
        .query("SELECT * FROM login_attempts WHERE ip_address = $ip_address AND success = false AND attempted_at > time::now() - $since ORDER BY attempted_at DESC")
        .bind(("ip_address", ip_address.to_string()))
        .bind(("since", since))
        .await?;

    let attempts: Vec<LoginAttemptRecord> = result.take(0)?;
    Ok(attempts)
}

pub async fn lock_user_account<C: Connection>(
    db: &Surreal<C>,
    user_id: Thing,
    lock_duration_seconds: u64,
) -> Result<(), surrealdb::Error> {
    db.query("UPDATE $user_id SET locked_until = time::now() + $lock_duration, failed_login_attempts = failed_login_attempts + 1")
        .bind(("user_id", user_id))
        .bind(("lock_duration", format!("{}s", lock_duration_seconds)))
        .await?;

    Ok(())
}

pub async fn reset_failed_login_attempts<C: Connection>(
    db: &Surreal<C>,
    user_id: Thing,
) -> Result<(), surrealdb::Error> {
    db.query("UPDATE $user_id SET failed_login_attempts = 0, locked_until = NONE, last_login_at = time::now()")
        .bind(("user_id", user_id))
        .await?;

    Ok(())
}

pub async fn update_session_activity<C: Connection>(
    db: &Surreal<C>,
    session_token: &str,
) -> Result<(), surrealdb::Error> {
    db.query("UPDATE sessions SET last_activity = time::now() WHERE session_token = $session_token")
        .bind(("session_token", session_token.to_string()))
        .await?;

    Ok(())
}

pub async fn revoke_all_user_sessions<C: Connection>(
    db: &Surreal<C>,
    user_id: Thing,
) -> Result<(), surrealdb::Error> {
    db.query("UPDATE sessions SET revoked = true WHERE user_id = $user_id")
        .bind(("user_id", user_id))
        .await?;

    Ok(())
}

pub async fn get_active_sessions_count<C: Connection>(
    db: &Surreal<C>,
    user_id: Thing,
) -> Result<usize, surrealdb::Error> {
    let mut result = db
        .query("SELECT count() FROM sessions WHERE user_id = $user_id AND expires_at > time::now() AND revoked = false GROUP ALL")
        .bind(("user_id", user_id))
        .await?;

    let count: Option<usize> = result.take(0)?;
    Ok(count.unwrap_or(0))
}

// Initialize database schema and indexes
pub async fn initialize_schema<C: Connection>(
    db: &Surreal<C>,
) -> Result<(), surrealdb::Error> {
    // Create indexes for performance
    db.query("DEFINE INDEX idx_users_email ON users COLUMNS email UNIQUE").await?;
    db.query("DEFINE INDEX idx_users_active ON users COLUMNS active").await?;
    db.query("DEFINE INDEX idx_sessions_token ON sessions COLUMNS session_token UNIQUE").await?;
    db.query("DEFINE INDEX idx_sessions_user_id ON sessions COLUMNS user_id").await?;
    db.query("DEFINE INDEX idx_sessions_expires_at ON sessions COLUMNS expires_at").await?;
    db.query("DEFINE INDEX idx_sessions_revoked ON sessions COLUMNS revoked").await?;
    db.query("DEFINE INDEX idx_login_attempts_email ON login_attempts COLUMNS email").await?;
    db.query("DEFINE INDEX idx_login_attempts_ip ON login_attempts COLUMNS ip_address").await?;
    db.query("DEFINE INDEX idx_login_attempts_time ON login_attempts COLUMNS attempted_at").await?;
    
    // Define database-level constraints
    db.query("DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value) AND string::len($value) <= 255").await?;
    db.query("DEFINE FIELD password_hash ON users TYPE string ASSERT string::len($value) > 0").await?;
    db.query("DEFINE FIELD role ON users TYPE string ASSERT $value IN ['Admin', 'Cashier', 'Staff']").await?;
    db.query("DEFINE FIELD active ON users TYPE bool").await?;
    db.query("DEFINE FIELD failed_login_attempts ON users TYPE number DEFAULT 0").await?;
    db.query("DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now()").await?;
    db.query("DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now()").await?;
    
    db.query("DEFINE FIELD user_id ON sessions TYPE record(users)").await?;
    db.query("DEFINE FIELD session_token ON sessions TYPE string ASSERT string::len($value) > 0").await?;
    db.query("DEFINE FIELD expires_at ON sessions TYPE datetime").await?;
    db.query("DEFINE FIELD created_at ON sessions TYPE datetime DEFAULT time::now()").await?;
    db.query("DEFINE FIELD last_activity ON sessions TYPE datetime DEFAULT time::now()").await?;
    db.query("DEFINE FIELD revoked ON sessions TYPE bool DEFAULT false").await?;
    
    db.query("DEFINE FIELD email ON login_attempts TYPE string ASSERT string::is::email($value)").await?;
    db.query("DEFINE FIELD ip_address ON login_attempts TYPE string ASSERT string::len($value) > 0").await?;
    db.query("DEFINE FIELD success ON login_attempts TYPE bool").await?;
    db.query("DEFINE FIELD attempted_at ON login_attempts TYPE datetime DEFAULT time::now()").await?;
    
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
