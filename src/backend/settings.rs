use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::db::DB;
    pub use crate::backend::websocket::{broadcast_add, broadcast_delete, broadcast_update};
    pub use crate::common::types;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::Thing;
    use surrealdb::RecordId;
    pub use validator::Validate;

    pub const SETTINGS: &str = "settings";
    pub const SETTINGS_ID: &str = "global";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Settings {
        pub id: Option<RecordId>,
        pub active_event_id: Option<String>,
    }

    impl From<Settings> for types::Settings {
        fn from(settings: Settings) -> Self {
            Self {
                id: settings.id.unwrap().key().to_string(),
                active_event_id: settings.active_event_id,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(GetSettings, "/api/settings")]
pub async fn get_settings() -> Result<types::Settings, ServerFnError> {
    // Try to get existing settings
    let existing: Option<Settings> = DB.select((SETTINGS, SETTINGS_ID)).await?;
    
    if let Some(settings) = existing {
        Ok(settings.into())
    } else {
        // Create default settings if they don't exist
        let default_settings: Option<Settings> = DB
            .create((SETTINGS, SETTINGS_ID))
            .content(Settings {
                id: None,
                active_event_id: None,
            })
            .await?;
            
        default_settings
            .map(Into::into)
            .ok_or_else(|| ServerError("Failed to create default settings".into()))
    }
}

#[server(UpdateSettings, "/api/settings")]
pub async fn update_settings(
    update: requests::settings::Update,
) -> Result<types::Settings, ServerFnError> {
    // Get existing settings or create default
    let current_settings = get_settings().await?;
    
    let updated = Settings {
        id: None, // Will be ignored by SurrealDB for updates
        active_event_id: update.active_event_id.or(current_settings.active_event_id),
    };
    
    // Update the settings in the database
    let updated_settings: Option<Settings> = DB
        .update((SETTINGS, SETTINGS_ID))
        .content(updated)
        .await?;
    
    if let Some(settings) = updated_settings {
        let result: types::Settings = settings.clone().into();
        broadcast_update(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to update settings".into()))
    }
}

#[server(SetActiveEvent, "/api/settings")]
pub async fn set_active_event(event_id: String) -> Result<types::Settings, ServerFnError> {
    let update = requests::settings::Update {
        active_event_id: Some(event_id),
    };
    update_settings(update).await
}

#[server(GetActiveEvent, "/api/settings")]
pub async fn get_active_event() -> Result<Option<String>, ServerFnError> {
    let settings = get_settings().await?;
    Ok(settings.active_event_id)
}