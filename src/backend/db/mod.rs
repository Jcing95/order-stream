use std::sync::LazyLock;
use crate::backend::config::DatabaseConfig;
use crate::common::errors::Error;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub mod category;
pub mod event;
pub mod item;
pub mod order;
pub mod product;
pub mod station;
pub mod user;

pub type Database = Surreal<Client>;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn initialize_database() -> Result<(), Error> {
    use crate::backend::config::AppConfig;

    let config = AppConfig::from_env()
        .map_err(|e| Error::InternalError(format!("Could not load env: {}", e)))?;

    DB.connect::<Ws>(&config.database.url)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to connect to database: {}", e)))?;

    DB.signin(Root {
        username: &config.database.user,
        password: &config.database.pass,
    })
    .await
    .map_err(|e| Error::InternalError(format!("Failed to sign in: {}", e)))?;

    DB.use_ns(&config.database.ns)
        .use_db(&config.database.db)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to select namespace/database: {}", e)))?;

    Ok(())
}

#[deprecated(note = "Use static DB instance instead")]
pub async fn get_db_connection() -> Result<Database, Error> {
    use crate::backend::config::AppConfig;

    let config = AppConfig::from_env()
        .map_err(|e| Error::InternalError(format!("Could not load env: {}", e)))?;
    connect_database(&config.database).await
}

#[deprecated(note = "Use initialize_database() instead")]
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
