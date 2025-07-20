use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type Database = Surreal<Client>;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn initialize_database() -> Result<(), String> {
    use crate::backend::config::AppConfig;

    let config = AppConfig::from_env().map_err(|e| format!("Could not load env: {}", e))?;

    DB.connect::<Ws>(&config.database.url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    DB.signin(Root {
        username: &config.database.user,
        password: &config.database.pass,
    })
    .await
    .map_err(|e| format!("Failed to sign in: {}", e))?;

    DB.use_ns(&config.database.ns)
        .use_db(&config.database.db)
        .await
        .map_err(|e| format!("Failed to select namespace/database: {}", e))?;

    Ok(())
}
