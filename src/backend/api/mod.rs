pub mod items;
pub mod categories;
pub mod orders;
pub mod order_items;

#[cfg(feature = "ssr")]
pub async fn get_db_connection() -> Result<crate::backend::database::Database, leptos::prelude::ServerFnError> {
    use crate::backend::database::connect_database;
    use crate::backend::config::AppConfig;
    
    let config = AppConfig::from_env().map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;
    let db = connect_database(&config.database).await.map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;
    Ok(db)
}