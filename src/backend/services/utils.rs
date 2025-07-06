use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
    crate::{
        backend::{
            database::{self, Database},
            services::auth::{require_admin_user, require_cashier_user, require_staff_user},
        },
        common::types::{User, UserRole},
    },
    leptos_axum::extract,
    tower_cookies::Cookies,
};

// Standard error messages
pub const AUTH_REQUIRED: &str = "Authentication required";
pub const SERVICE_UNAVAILABLE: &str = "Service temporarily unavailable"; 
pub const NOT_FOUND: &str = "Not found";
pub const INVALID_REQUEST: &str = "Invalid request";

/// Combined authentication and database connection helper
/// Eliminates the repeated auth + db connection pattern in every service function
#[cfg(feature = "ssr")]
pub async fn with_auth_and_db<T, F, Fut>(
    role: UserRole,
    operation: F,
) -> Result<T, ServerFnError>
where
    F: FnOnce(Database, User) -> Fut,
    Fut: std::future::Future<Output = Result<T, ServerFnError>>,
{
    // Extract cookies
    let cookies = extract::<Cookies>().await
        .map_err(|_| ServerFnError::new(AUTH_REQUIRED))?;
    
    // Get authenticated user with required role
    let user = match role {
        UserRole::Admin => require_admin_user(&cookies).await?,
        UserRole::Cashier => require_cashier_user(&cookies).await?,
        UserRole::Staff => require_staff_user(&cookies).await?,
    };
    
    // Get database connection
    let db = database::get_db_connection()
        .await
        .map_err(|_| ServerFnError::new(SERVICE_UNAVAILABLE))?;
    
    // Execute the operation with db and user
    operation(db, user).await
}

/// Helper for operations that only need staff level access
#[cfg(feature = "ssr")]
pub async fn with_staff_auth<T, F, Fut>(operation: F) -> Result<T, ServerFnError>
where
    F: FnOnce(Database, User) -> Fut,
    Fut: std::future::Future<Output = Result<T, ServerFnError>>,
{
    with_auth_and_db(UserRole::Staff, operation).await
}

/// Helper for operations that need cashier level access
#[cfg(feature = "ssr")]
pub async fn with_cashier_auth<T, F, Fut>(operation: F) -> Result<T, ServerFnError>
where
    F: FnOnce(Database, User) -> Fut,
    Fut: std::future::Future<Output = Result<T, ServerFnError>>,
{
    with_auth_and_db(UserRole::Cashier, operation).await
}

/// Helper for operations that need admin level access
#[cfg(feature = "ssr")]
pub async fn with_admin_auth<T, F, Fut>(operation: F) -> Result<T, ServerFnError>
where
    F: FnOnce(Database, User) -> Fut,
    Fut: std::future::Future<Output = Result<T, ServerFnError>>,
{
    with_auth_and_db(UserRole::Admin, operation).await
}

/// Standard database error mapping
#[cfg(feature = "ssr")]
pub fn map_db_error<T>(_err: T) -> ServerFnError {
    ServerFnError::new(SERVICE_UNAVAILABLE)
}

/// Handle Option<T> with standard "Not found" error
pub fn ok_or_not_found<T>(option: Option<T>) -> Result<T, ServerFnError> {
    option.ok_or_else(|| ServerFnError::new(NOT_FOUND))
}