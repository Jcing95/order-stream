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
    // Define the table
    db.query("DEFINE TABLE IF NOT EXISTS items SCHEMAFULL")
        .await?
        .check()?;

    // Define each field separately
    let field_defs = [
        r#"DEFINE FIELD IF NOT EXISTS name ON items TYPE string ASSERT $value != NONE AND string::len($value) > 0;"#,
        r#"DEFINE FIELD IF NOT EXISTS category ON items TYPE string ASSERT $value != NONE AND string::len($value) > 0;"#,
        r#"DEFINE FIELD IF NOT EXISTS price ON items TYPE number ASSERT $value != NONE AND $value >= 0;"#,
        r#"DEFINE FIELD IF NOT EXISTS active ON items TYPE bool ASSERT $value != NONE;"#,
        r#"DEFINE FIELD IF NOT EXISTS created_at ON items TYPE datetime ASSERT $value != NONE;"#,
        r#"DEFINE FIELD IF NOT EXISTS updated_at ON items TYPE datetime ASSERT $value != NONE;"#,
    ];

    for def in field_defs {
        db.query(def)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Field definition failed: {}", e)))?
            .check()
            .map_err(|e| AppError::DatabaseError(format!("Field check failed: {}", e)))?;
    }

    Ok(())
}
