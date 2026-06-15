use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateUser, "/api/user")]
pub async fn create_user(req: requests::user::Create) -> Result<types::User, ServerFnError> {
    use crate::backend::auth::{hash_password, SessionData, get_extended_expiry};
    use crate::backend::db::get_pool;
    use crate::backend::models::{NewUser, DbUser, role_str};
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use tower_sessions::Session;

    let pw_hash = hash_password(&req.password)
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let count: i64 = users
        .count()
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let assigned_role = if count == 0 { types::Role::Admin } else { types::Role::Visitor };

    let new_user = NewUser {
        email: &req.email,
        password_hash: &pw_hash,
        role: role_str(&assigned_role),
    };

    let user: DbUser = diesel::insert_into(users)
        .values(&new_user)
        .returning(DbUser::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create user: {}", e)))?;

    // Auto-login: create session for the new user
    let session: Session = leptos_axum::extract().await?;
    let session_data = SessionData::new(user.id.to_string());
    session.insert("user", session_data).await
        .map_err(|e| Error::InternalError(format!("Session error: {}", e)))?;
    session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(get_extended_expiry())));

    Ok(user.into())
}

#[server(GetAllUsers, "/api/users")]
pub async fn get_all_users() -> Result<Vec<types::User>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbUser;
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbUser> = users
        .select(DbUser::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetUser, "/api/user")]
pub async fn get_user(user_id: String) -> Result<types::User, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbUser;
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use uuid::Uuid;

    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| Error::InternalError("Invalid user ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let user: Option<DbUser> = users
        .filter(id.eq(uuid))
        .select(DbUser::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    user.map(Into::into)
        .ok_or_else(|| Error::InternalError("User not found".to_string()).into())
}

#[cfg(feature = "ssr")]
pub async fn get_user_by_email(email_val: &str) -> Result<Option<crate::backend::models::DbUser>, diesel::result::Error> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbUser;
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.expect("pool unavailable");

    users
        .filter(email.eq(email_val))
        .select(DbUser::as_select())
        .first(&mut conn)
        .await
        .optional()
}

#[server(UpdateUser, "/api/user")]
pub async fn update_user(
    user_id: String,
    update: requests::user::Update,
) -> Result<types::User, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbUser, UpdateUser, role_str};
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use uuid::Uuid;

    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| Error::InternalError("Invalid user ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let changes = UpdateUser {
        email: update.email,
        role: update.role.as_ref().map(|r| role_str(r).to_string()),
    };

    let user: DbUser = diesel::update(users.filter(id.eq(uuid)))
        .set(&changes)
        .returning(DbUser::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update user: {}", e)))?;

    Ok(user.into())
}

#[server(DeleteUser, "/api/user")]
pub async fn delete_user(user_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use uuid::Uuid;

    let uuid = Uuid::parse_str(&user_id)
        .map_err(|_| Error::InternalError("Invalid user ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(users.filter(id.eq(uuid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("User {} not found", user_id)).into());
    }
    Ok(())
}

#[server(Login, "/api/user")]
pub async fn login(email_val: String, password: String) -> Result<types::User, ServerFnError> {
    use crate::backend::auth::{verify_password, SessionData, get_extended_expiry};
    use crate::backend::models::ParseRole;
    use tower_sessions::Session;

    let session: Session = leptos_axum::extract().await?;

    let user = get_user_by_email(&email_val)
        .await
        .map_err(|e| Error::InternalError(format!("Database error: {}", e)))?
        .ok_or_else(|| Error::NotAuthorized("Invalid credentials".to_string()))?;

    if !verify_password(&password, &user.password_hash)? {
        return Err(Error::NotAuthorized("Invalid credentials".to_string()).into());
    }

    let session_data = SessionData::new(user.id.to_string());
    session.insert("user", session_data).await?;
    session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(get_extended_expiry())));

    Ok(crate::common::types::User {
        id: user.id.to_string(),
        email: user.email,
        role: user.role.parse_role(),
    })
}

#[server(Logout, "/api/user")]
pub async fn logout() -> Result<(), ServerFnError> {
    use tower_sessions::Session;
    let session: Session = leptos_axum::extract().await?;
    let _ = session.delete().await;
    Ok(())
}

#[server(GetCurrentUser, "/api/user")]
pub async fn get_current_user() -> Result<types::User, ServerFnError> {
    use tower_sessions::Session;
    let session: Session = leptos_axum::extract().await?;
    crate::backend::auth::get_authenticated_user(&session).await
}
