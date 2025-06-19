use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub db: String,
    pub ns: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let database = DatabaseConfig {
            url: std::env::var("SURREAL_URL")
                .unwrap_or_else(|_| "ws://127.0.0.1:8000/rpc".to_string()),
            user: std::env::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string()),
            pass: std::env::var("SURREAL_PASS").unwrap_or_else(|_| "root".to_string()),
            db: std::env::var("SURREAL_DB").unwrap_or_else(|_| "orderstream".to_string()),
            ns: std::env::var("SURREAL_NS").unwrap_or_else(|_| "production".to_string()),
        };

        Ok(AppConfig { database })
    }
}
