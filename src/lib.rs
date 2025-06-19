pub mod app;
pub mod common;
pub mod frontend;

#[cfg(feature = "ssr")]
pub mod backend;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "ssr")]
pub async fn setup_database() -> Result<crate::backend::database::Database, Box<dyn std::error::Error>> {
    use crate::backend::config::AppConfig;
    use crate::backend::database::{connect_database, initialize_database};

    // Load configuration
    let config = AppConfig::from_env()?;
    
    // Connect to database
    let db = connect_database(&config.database).await?;
    
    // Initialize database schema
    initialize_database(&db).await?;
    
    println!("Database connected and initialized");
    Ok(db)
}

#[cfg(feature = "ssr")]
pub fn add_api_routes(
    db: crate::backend::database::Database
) -> axum::Router {
    use crate::backend::api::items::items_router;
    
    let api_routes = items_router().with_state(db);
    return api_routes;
}