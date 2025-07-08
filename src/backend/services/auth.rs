use leptos::prelude::*;
use crate::common::{
    types::{RegisterRequest, LoginRequest, AuthResponse, User, UserSecurityInfo},
};

#[cfg(feature = "ssr")]
use {
    crate::{
        backend::{
            database::{get_db_connection, users::{
                create_user, get_user_by_email, create_session, 
                get_failed_login_attempts, reset_failed_login_attempts,
                update_session_activity
            }},
            errors::AppError,
            services::utils::*,
        },
        common::types::UserRole,
    },
    bcrypt::{hash, verify, DEFAULT_COST},
    uuid::Uuid,
    leptos_axum::extract,
    tower_cookies::{Cookie, Cookies},
};

// Cookie configuration helper to eliminate redundancy
#[cfg(feature = "ssr")]
fn create_session_cookie(token: String, max_age: tower_cookies::cookie::time::Duration) -> Cookie<'static> {
    let mut cookie = Cookie::new("session_token", token);
    cookie.set_http_only(true);
    cookie.set_secure(false); // Set to true in production with HTTPS
    cookie.set_path("/");
    cookie.set_max_age(Some(max_age));
    cookie
}

// Admin authentication helper to eliminate massive duplication
#[cfg(feature = "ssr")]
async fn with_admin_auth_and_db<T, F, Fut>(operation: F) -> Result<T, ServerFnError>
where
    F: FnOnce(crate::backend::database::Database, User) -> Fut,
    Fut: std::future::Future<Output = Result<T, ServerFnError>>,
{
    let cookies = extract::<Cookies>().await
        .map_err(|_| ServerFnError::new(SERVICE_UNAVAILABLE))?;

    let current_user = get_authenticated_user_from_request(&cookies).await
        .map_err(|_| ServerFnError::new(AUTH_REQUIRED))?
        .ok_or_else(|| ServerFnError::new(AUTH_REQUIRED))?;

    require_role(&current_user, UserRole::Admin)?;

    let db = get_db_connection().await
        .map_err(|_| ServerFnError::new(SERVICE_UNAVAILABLE))?;

    operation(db, current_user).await
}

// User fetching helper to eliminate repeated pattern
#[cfg(feature = "ssr")]
async fn get_user_by_email_or_error(
    db: &crate::backend::database::Database, 
    email: &str
) -> Result<crate::backend::database::users::UserRecord, ServerFnError> {
    get_user_by_email(db, email).await
        .map_err(|_| ServerFnError::new(SERVICE_UNAVAILABLE))?
        .ok_or_else(|| ServerFnError::new("User not found"))
}

#[server(RegisterUser, "/api")]
pub async fn register_user(request: RegisterRequest) -> Result<AuthResponse, ServerFnError> {
    request.validate().map_err(|_| ServerFnError::new("Invalid request"))?;

    let db = get_db_connection().await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    // Check if user already exists
    match get_user_by_email(&db, &request.email).await {
        Ok(Some(_)) => return Err(ServerFnError::new("Email already registered")),
        Ok(None) => {}, // Continue with registration
        Err(_) => return Err(ServerFnError::new("Service temporarily unavailable")),
    }

    // Hash password
    let password_hash = hash(&request.password, DEFAULT_COST)
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    // Create user
    let user_record = create_user(&db, request.email.clone(), password_hash, request.role).await
        .map_err(|_| ServerFnError::new("Failed to create account"))?;

    // Record successful registration as a login attempt
    let _ = crate::backend::database::users::record_login_attempt(
        &db, 
        request.email, 
        "unknown".to_string(), 
        true, 
        None
    ).await;

    // Create session (1 week duration)
    let session_token = Uuid::new_v4().to_string();

    create_session(&db, user_record.id.clone(), session_token.clone(), None, None).await
        .map_err(|_| ServerFnError::new("Failed to create session"))?;

    // Set secure cookie
    let cookies = extract::<Cookies>().await
        .map_err(|_| ServerFnError::new("Failed to set session cookie"))?;

    cookies.add(create_session_cookie(session_token.clone(), tower_cookies::cookie::time::Duration::weeks(1)));

    Ok(AuthResponse {
        user: user_record.into(),
        session_token,
    })
}

#[server(LoginUser, "/api")]
pub async fn login_user(request: LoginRequest) -> Result<AuthResponse, ServerFnError> {
    request.validate().map_err(|_| ServerFnError::new("Invalid request"))?;

    let db = get_db_connection().await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    // Get user by email (this already checks for account locks and active status)
    let user_record = match get_user_by_email(&db, &request.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Simulate password verification timing to prevent timing attacks
            let _ = hash("dummy_password", DEFAULT_COST);
            return Err(ServerFnError::new("Invalid email or password"));
        }
        Err(_) => return Err(ServerFnError::new("Service temporarily unavailable")),
    };

    // Check failed login attempts and implement exponential backoff (last 1 hour)
    let failed_attempts = get_failed_login_attempts(&db, &request.email, surrealdb::sql::Duration::from_hours(1).unwrap_or(surrealdb::sql::Duration::from_secs(3600))).await
        .unwrap_or_else(|_| Vec::new());
    
    let recent_failures = failed_attempts.len() as u32;
    if recent_failures >= 3 {
        // Instead of locking, just return a rate limiting error
        return Err(ServerFnError::new("Too many failed attempts. Please wait before trying again."));
    }

    // Verify password
    let password_valid = verify(&request.password, &user_record.password_hash)
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    if !password_valid {
        // Record failed login attempt (without sensitive details since we don't track IPs)
        let _ = crate::backend::database::users::record_login_attempt(
            &db, 
            request.email, 
            "unknown".to_string(), // placeholder since we don't track IPs
            false, 
            None
        ).await;
        
        return Err(ServerFnError::new("Invalid email or password"));
    }

    if !user_record.active {
        return Err(ServerFnError::new("Account is disabled"));
    }

    // Success! Record successful login for audit purposes
    let _ = crate::backend::database::users::record_login_attempt(
        &db, 
        request.email, 
        "unknown".to_string(), 
        true, 
        None
    ).await;

    // Create session (1 week duration)
    let session_token = Uuid::new_v4().to_string();

    create_session(&db, user_record.id.clone(), session_token.clone(), None, None).await
        .map_err(|_| ServerFnError::new("Failed to create session"))?;

    // Set secure cookie
    let cookies = extract::<Cookies>().await
        .map_err(|_| ServerFnError::new("Failed to set session cookie"))?;

    cookies.add(create_session_cookie(session_token.clone(), tower_cookies::cookie::time::Duration::weeks(1)));

    Ok(AuthResponse {
        user: user_record.into(),
        session_token,
    })
}

#[server(LogoutUser, "/api")]
pub async fn logout_user() -> Result<(), ServerFnError> {
    let cookies = extract::<Cookies>().await
        .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;

    if let Some(session_cookie) = cookies.get("session_token") {
        let db = get_db_connection().await
            .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

        // Delete session from database
        crate::backend::database::users::delete_session(&db, session_cookie.value()).await
            .map_err(|e| ServerFnError::new(format!("Failed to delete session: {}", e)))?;
    }

    // Remove cookie  
    cookies.add(create_session_cookie("".to_string(), tower_cookies::cookie::time::Duration::seconds(-1)));

    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    let cookies = extract::<Cookies>().await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    let session_token = match cookies.get("session_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Ok(None),
    };

    let db = get_db_connection().await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    // Get session
    let session_record = crate::backend::database::users::get_session_by_token(&db, &session_token).await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    let session_record = match session_record {
        Some(session) => session,
        None => return Ok(None), // Invalid or expired session
    };

    // Update session activity (don't fail if this fails)
    let _ = update_session_activity(&db, &session_token).await;

    // Get user
    let user_record = crate::backend::database::users::get_user_by_id(&db, &session_record.user_id).await
        .map_err(|_| ServerFnError::new("Service temporarily unavailable"))?;

    match user_record {
        Some(user) if user.active => Ok(Some(user.into())),
        _ => Ok(None),
    }
}

#[cfg(feature = "ssr")]
pub async fn get_authenticated_user_from_request(cookies: &Cookies) -> Result<Option<User>, AppError> {
    let session_token = match cookies.get("session_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Ok(None),
    };

    let db = get_db_connection().await?;

    // Get session
    let session_record = crate::backend::database::users::get_session_by_token(&db, &session_token).await
        .map_err(|e| AppError::DatabaseError(format!("Database error: {}", e)))?;

    let session_record = match session_record {
        Some(session) => session,
        None => return Ok(None), // Invalid or expired session
    };

    // Update session activity (don't fail if this fails)
    let _ = update_session_activity(&db, &session_token).await;

    // Get user
    let user_record = crate::backend::database::users::get_user_by_id(&db, &session_record.user_id).await
        .map_err(|e| AppError::DatabaseError(format!("Database error: {}", e)))?;

    match user_record {
        Some(user) if user.active => Ok(Some(user.into())),
        _ => Ok(None),
    }
}

#[cfg(feature = "ssr")]
pub fn require_auth(user: Option<User>) -> Result<User, ServerFnError> {
    user.ok_or_else(|| ServerFnError::new("Authentication required"))
}

#[cfg(feature = "ssr")]
pub fn require_role(user: &User, required_role: UserRole) -> Result<(), ServerFnError> {
    match (required_role, user.role) {
        // Admin can access everything
        (_, UserRole::Admin) => Ok(()),
        
        // Cashier can access cashier functions and staff functions
        (UserRole::Cashier, UserRole::Cashier) => Ok(()),
        (UserRole::Staff, UserRole::Cashier) => Ok(()),
        
        // Staff can only access staff functions
        (UserRole::Staff, UserRole::Staff) => Ok(()),
        
        // Otherwise, access denied
        _ => Err(ServerFnError::new("Insufficient permissions")),
    }
}

// Admin-only security management endpoints

/// Revoke all sessions for a user - requires admin access
#[server(RevokeUserSessions, "/api")]
pub async fn revoke_user_sessions(user_email: String) -> Result<(), ServerFnError> {
    with_admin_auth_and_db(|db, _current_user| async move {
        let user_record = get_user_by_email_or_error(&db, &user_email).await?;

        crate::backend::database::users::revoke_all_user_sessions(&db, user_record.id).await
            .map_err(|_| ServerFnError::new("Failed to revoke sessions"))
    }).await
}

/// Lock a user account for a specified duration - requires admin access
#[server(LockUserAccount, "/api")]
pub async fn admin_lock_user_account(user_email: String, duration_hours: u32) -> Result<(), ServerFnError> {
    with_admin_auth_and_db(|db, current_user| async move {
        // Prevent self-locking
        if current_user.email == user_email {
            return Err(ServerFnError::new("Cannot lock your own account"));
        }

        let user_record = get_user_by_email_or_error(&db, &user_email).await?;

        // Convert hours to seconds
        let duration_seconds = (duration_hours as u64) * 3600;

        // Lock the account
        crate::backend::database::users::lock_user_account(&db, user_record.id.clone(), duration_seconds).await
            .map_err(|_| ServerFnError::new("Failed to lock account"))?;

        // Also revoke all existing sessions
        let _ = crate::backend::database::users::revoke_all_user_sessions(&db, user_record.id).await;

        Ok(())
    }).await
}

/// Unlock a user account - requires admin access
#[server(UnlockUserAccount, "/api")]
pub async fn unlock_user_account(user_email: String) -> Result<(), ServerFnError> {
    with_admin_auth_and_db(|db, _current_user| async move {
        let user_record = get_user_by_email_or_error(&db, &user_email).await?;

        // Reset failed login attempts (this also clears the lock)
        reset_failed_login_attempts(&db, user_record.id).await
            .map_err(|_| ServerFnError::new("Failed to unlock account"))
    }).await
}

/// Get security information for a user - requires admin access
#[server(GetUserSecurityInfo, "/api")]
pub async fn get_user_security_info(user_email: String) -> Result<UserSecurityInfo, ServerFnError> {
    with_admin_auth_and_db(|db, _current_user| async move {
        let user_record = get_user_by_email_or_error(&db, &user_email).await?;

        // Get active sessions count
        let active_sessions = crate::backend::database::users::get_active_sessions_count(&db, user_record.id.clone()).await
            .unwrap_or(0);

        // Get recent failed login attempts (last 24 hours)
        let failed_attempts = get_failed_login_attempts(&db, &user_email, surrealdb::sql::Duration::from_hours(24).unwrap_or(surrealdb::sql::Duration::from_secs(86400))).await
            .unwrap_or_else(|_| Vec::new());

        Ok(UserSecurityInfo {
            email: user_record.email,
            active: user_record.active,
            failed_login_attempts: user_record.failed_login_attempts,
            locked_until: user_record.locked_until.map(|_| "Account is locked".to_string()),
            active_sessions_count: active_sessions,
            recent_failed_attempts_count: failed_attempts.len() as u32,
            last_login: user_record.last_login_at.map(|_| "Has logged in".to_string()),
        })
    }).await
}

// Authentication utility functions

#[cfg(feature = "ssr")]
pub async fn require_authenticated_user(cookies: &Cookies) -> Result<User, ServerFnError> {
    get_authenticated_user_from_request(cookies).await
        .map_err(|_| ServerFnError::new("Authentication required"))?
        .ok_or_else(|| ServerFnError::new("Authentication required"))
}

#[cfg(feature = "ssr")]
pub async fn require_admin_user(cookies: &Cookies) -> Result<User, ServerFnError> {
    let user = require_authenticated_user(cookies).await?;
    require_role(&user, UserRole::Admin)?;
    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn require_cashier_user(cookies: &Cookies) -> Result<User, ServerFnError> {
    let user = require_authenticated_user(cookies).await?;
    require_role(&user, UserRole::Cashier)?;
    Ok(user)
}

#[cfg(feature = "ssr")]
pub async fn require_staff_user(cookies: &Cookies) -> Result<User, ServerFnError> {
    let user = require_authenticated_user(cookies).await?;
    require_role(&user, UserRole::Staff)?;
    Ok(user)
}

// Session management utilities

/// Cleanup expired sessions - requires admin access
#[server(CleanupExpiredSessions, "/api")]
pub async fn cleanup_expired_sessions() -> Result<(), ServerFnError> {
    with_admin_auth_and_db(|db, _current_user| async move {
        crate::backend::database::users::cleanup_expired_sessions(&db).await
            .map_err(|_| ServerFnError::new("Failed to cleanup sessions"))
    }).await
}

/// Initialize database schema - requires admin access
#[server(InitializeDatabaseSchema, "/api")]
pub async fn initialize_database_schema() -> Result<(), ServerFnError> {
    with_admin_auth_and_db(|db, _current_user| async move {
        crate::backend::database::users::initialize_schema(&db).await
            .map_err(|_| ServerFnError::new("Failed to initialize schema"))
    }).await
}