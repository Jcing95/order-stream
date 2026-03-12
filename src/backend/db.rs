use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool, pooled_connection::AsyncDieselConnectionManager};
use std::sync::OnceLock;

pub type DbPool = Pool<AsyncPgConnection>;

static POOL: OnceLock<DbPool> = OnceLock::new();

pub async fn initialize_database() -> Result<(), String> {
    use crate::backend::config::AppConfig;

    let config = AppConfig::from_env().map_err(|e| format!("Could not load env: {}", e))?;

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
    let pool = Pool::builder(manager)
        .build()
        .map_err(|e| format!("Failed to build connection pool: {}", e))?;

    // Verify connectivity
    pool.get()
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    POOL.set(pool).map_err(|_| "Pool already initialized".to_string())?;

    Ok(())
}

pub fn get_pool() -> &'static DbPool {
    POOL.get().expect("Database pool not initialized")
}
