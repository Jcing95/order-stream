use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(GetSettings, "/api/settings")]
pub async fn get_settings() -> Result<types::Settings, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbSettings;
    use crate::backend::schema::settings::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let s: DbSettings = settings
        .select(DbSettings::as_select())
        .first(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to load settings: {}", e)))?;

    Ok(s.into())
}

#[server(UpdateSettings, "/api/settings")]
pub async fn update_settings(
    update: requests::settings::Update,
) -> Result<types::Settings, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbSettings, UpdateSettings};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::settings::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    // Convert Option<String> → Option<Option<i32>>
    // Some("123") → Some(Some(123))
    // Some("") or None → Some(None) clears the value
    let new_event_id: Option<Option<i32>> = match update.active_event_id {
        Some(ref s) if !s.is_empty() => {
            let eid: i32 = s.parse()
                .map_err(|_| Error::InternalError("Invalid event ID".to_string()))?;
            Some(Some(eid))
        }
        Some(_) => Some(None), // empty string clears the event
        None => None,          // no change
    };

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let s: DbSettings = diesel::update(settings.filter(id.eq(1)))
        .set(UpdateSettings { active_event_id: new_event_id })
        .returning(DbSettings::as_select())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update settings: {}", e)))?;

    let result: types::Settings = s.into();
    broadcast_update(result.clone());
    Ok(result)
}

#[server(SetActiveEvent, "/api/settings")]
pub async fn set_active_event(event_id: String) -> Result<types::Settings, ServerFnError> {
    update_settings(requests::settings::Update {
        active_event_id: Some(event_id),
    })
    .await
}

#[server(GetActiveEvent, "/api/settings")]
pub async fn get_active_event() -> Result<Option<String>, ServerFnError> {
    let settings = get_settings().await?;
    Ok(settings.active_event_id)
}