use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool, pooled_connection::AsyncDieselConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::pg::PgConnection;
use diesel::Connection;
use std::sync::OnceLock;

pub type DbPool = Pool<AsyncPgConnection>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static POOL: OnceLock<DbPool> = OnceLock::new();

pub async fn initialize_database() -> Result<(), String> {
    use crate::backend::config::AppConfig;

    let config = AppConfig::from_env().map_err(|e| format!("Could not load env: {}", e))?;

    // Run migrations synchronously (diesel_migrations requires a sync connection)
    let mut sync_conn = PgConnection::establish(&config.database_url)
        .map_err(|e| format!("Failed to connect for migrations: {}", e))?;
    sync_conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    // Build async connection pool
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
    let pool = Pool::builder(manager)
        .build()
        .map_err(|e| format!("Failed to build connection pool: {}", e))?;

    // Verify connectivity
    let _ = pool.get()
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    POOL.set(pool).map_err(|_| "Pool already initialized".to_string())?;

    Ok(())
}

pub fn get_pool() -> &'static DbPool {
    POOL.get().expect("Database pool not initialized")
}
