
use crate::backend::config::DatabaseConfig;
use crate::backend::errors::{AppError, AppResult};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub mod categories;
pub mod items;
pub mod orders;
pub mod order_items;

pub type Database = Surreal<Client>;

pub async fn connect_database(config: &DatabaseConfig) -> AppResult<Database> {
    let db = Surreal::new::<Ws>(&config.url)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))?;

    db.signin(Root {
        username: &config.user,
        password: &config.pass,
    })
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to sign in: {}", e)))?;

    db.use_ns(&config.ns)
        .use_db(&config.db)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to select namespace/database: {}", e)))?;

    // SurrealDB will automatically create table schema based on our model usage
    Ok(db)
}

// Helper function for services to get database connection
pub async fn get_db_connection() -> AppResult<Database> {
    use crate::backend::config::AppConfig;
    
    let config = AppConfig::from_env()?;
    connect_database(&config.database).await
}
