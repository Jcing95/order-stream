
use crate::backend::config::DatabaseConfig;
use crate::backend::error::Error;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub mod category;
pub mod product;
pub mod order;
pub mod item;
pub mod station;
pub mod user;
pub mod event;

pub type Database = Surreal<Client>;

pub async fn connect_database(config: &DatabaseConfig) -> Result<Database, Error> {
    let db = Surreal::new::<Ws>(&config.url)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to connect to database: {}", e)))?;

    db.signin(Root {
        username: &config.user,
        password: &config.pass,
    })
    .await
    .map_err(|e| Error::InternalError(format!("Failed to sign in: {}", e)))?;

    db.use_ns(&config.ns)
        .use_db(&config.db)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to select namespace/database: {}", e)))?;
    Ok(db)
}

pub async fn get_db_connection() -> Result<Database, Error> {
    use crate::backend::config::AppConfig;
    
    let config = AppConfig::from_env()?;
    connect_database(&config.database).await
}
