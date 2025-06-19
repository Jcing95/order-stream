use crate::backend::config::DatabaseConfig;
use crate::common::errors::{AppError, AppResult};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

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

    Ok(db)
}

pub async fn initialize_database(db: &Database) -> AppResult<()> {
    // Create the items table if it doesn't exist
    let _: Vec<surrealdb::sql::Value> = db
        .query("DEFINE TABLE items SCHEMAFULL")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create items table: {}", e)))?
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute table creation: {}", e)))?;

    // Define table fields
    let _: Vec<surrealdb::sql::Value> = db
        .query(r#"
            DEFINE FIELD name ON items TYPE string ASSERT $value != NONE AND string::len($value) > 0;
            DEFINE FIELD category ON items TYPE string ASSERT $value != NONE AND string::len($value) > 0;
            DEFINE FIELD price ON items TYPE number ASSERT $value != NONE AND $value >= 0;
            DEFINE FIELD active ON items TYPE bool ASSERT $value != NONE;
            DEFINE FIELD created_at ON items TYPE datetime ASSERT $value != NONE;
            DEFINE FIELD updated_at ON items TYPE datetime ASSERT $value != NONE;
        "#)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to define table fields: {}", e)))?
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute field definitions: {}", e)))?;

    Ok(())
}