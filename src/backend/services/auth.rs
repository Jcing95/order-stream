use leptos::prelude::*;
use crate::common::{
    types::{RegisterRequest, LoginRequest, AuthResponse, User},
};

#[cfg(feature = "ssr")]
use {
    crate::{
        backend::{
            database::{get_db_connection, users::{create_user, get_user_by_email, create_session}},
            errors::AppError,
        },
        common::types::UserRole,
    },
    bcrypt::{hash, verify, DEFAULT_COST},
    uuid::Uuid,
    chrono::{Duration, Utc},
    leptos_axum::extract,
    tower_cookies::{Cookie, Cookies},
};

#[server(RegisterUser, "/api")]
pub async fn register_user(request: RegisterRequest) -> Result<AuthResponse, ServerFnError> {
    request.validate().map_err(|e| ServerFnError::new(e))?;

    let db = get_db_connection().await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    // Check if user already exists
    if let Ok(Some(_)) = get_user_by_email(&db, &request.email).await {
        return Err(ServerFnError::new("User already exists with this email"));
    }

    // Hash password
    let password_hash = hash(&request.password, DEFAULT_COST)
        .map_err(|e| ServerFnError::new(format!("Password hashing failed: {}", e)))?;

    // Create user
    let user_record = create_user(&db, request.email, password_hash, request.role).await
        .map_err(|e| ServerFnError::new(format!("Failed to create user: {}", e)))?;

    // Create session
    let session_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::days(30); // 30-day session

    create_session(&db, user_record.id.to_string(), session_token.clone(), expires_at).await
        .map_err(|e| ServerFnError::new(format!("Failed to create session: {}", e)))?;

    // Set secure cookie
    let cookies = extract::<Cookies>().await
        .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;

    let mut cookie = Cookie::new("session_token", session_token.clone());
    cookie.set_http_only(true);
    cookie.set_secure(false); // Set to true in production with HTTPS
    cookie.set_path("/");
    cookie.set_max_age(Some(tower_cookies::cookie::time::Duration::days(30)));
    cookies.add(cookie);

    Ok(AuthResponse {
        user: user_record.into(),
        session_token,
    })
}

#[server(LoginUser, "/api")]
pub async fn login_user(request: LoginRequest) -> Result<AuthResponse, ServerFnError> {
    request.validate().map_err(|e| ServerFnError::new(e))?;

    let db = get_db_connection().await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    // Get user by email
    let user_record = get_user_by_email(&db, &request.email).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?
        .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

    // Verify password
    let password_valid = verify(&request.password, &user_record.password_hash)
        .map_err(|e| ServerFnError::new(format!("Password verification failed: {}", e)))?;

    if !password_valid {
        return Err(ServerFnError::new("Invalid email or password"));
    }

    if !user_record.active {
        return Err(ServerFnError::new("User account is disabled"));
    }

    // Create session
    let session_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::days(30); // 30-day session

    create_session(&db, user_record.id.to_string(), session_token.clone(), expires_at).await
        .map_err(|e| ServerFnError::new(format!("Failed to create session: {}", e)))?;

    // Set secure cookie
    let cookies = extract::<Cookies>().await
        .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;

    let mut cookie = Cookie::new("session_token", session_token.clone());
    cookie.set_http_only(true);
    cookie.set_secure(false); // Set to true in production with HTTPS
    cookie.set_path("/");
    cookie.set_max_age(Some(tower_cookies::cookie::time::Duration::days(30)));
    cookies.add(cookie);

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
    let mut cookie = Cookie::new("session_token", "");
    cookie.set_http_only(true);
    cookie.set_secure(false); // Set to true in production with HTTPS
    cookie.set_path("/");
    cookie.set_max_age(Some(tower_cookies::cookie::time::Duration::seconds(-1))); // Expire immediately
    cookies.add(cookie);

    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    let cookies = extract::<Cookies>().await
        .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;

    let session_token = match cookies.get("session_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Ok(None),
    };

    let db = get_db_connection().await
        .map_err(|e| ServerFnError::new(format!("Database connection failed: {}", e)))?;

    // Get session
    let session_record = crate::backend::database::users::get_session_by_token(&db, &session_token).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let session_record = match session_record {
        Some(session) => session,
        None => return Ok(None), // Invalid or expired session
    };

    // Get user
    let user_record = crate::backend::database::users::get_user_by_id(&db, &session_record.user_id).await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

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