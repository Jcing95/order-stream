use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateEvent, "/api/event")]
pub async fn create_event(req: requests::event::Create) -> Result<types::Event, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbEvent, NewEvent};
    use crate::backend::websocket::broadcast_add;
    use crate::backend::schema::events::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let event: DbEvent = diesel::insert_into(events)
        .values(NewEvent { name: &req.name })
        .returning(DbEvent::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create event: {}", e)))?;

    let result: types::Event = event.into();
    broadcast_add(result.clone());
    Ok(result)
}

#[server(GetEvents, "/api/event")]
pub async fn get_events() -> Result<Vec<types::Event>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbEvent;
    use crate::backend::schema::events::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbEvent> = events
        .select(DbEvent::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetEvent, "/api/event")]
pub async fn get_event(event_id: String) -> Result<types::Event, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbEvent;
    use crate::backend::schema::events::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let eid: i32 = event_id.parse()
        .map_err(|_| Error::InternalError("Invalid event ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let event: Option<DbEvent> = events
        .filter(id.eq(eid))
        .select(DbEvent::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    event.map(Into::into)
        .ok_or_else(|| Error::InternalError("Event not found".to_string()).into())
}

#[server(UpdateEvent, "/api/event")]
pub async fn update_event(
    event_id: String,
    update: requests::event::Update,
) -> Result<types::Event, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbEvent, UpdateEvent};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::events::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let eid: i32 = event_id.parse()
        .map_err(|_| Error::InternalError("Invalid event ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let event: DbEvent = diesel::update(events.filter(id.eq(eid)))
        .set(UpdateEvent { name: update.name })
        .returning(DbEvent::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update event: {}", e)))?;

    let result: types::Event = event.into();
    broadcast_update(result.clone());
    Ok(result)
}

#[server(DeleteEvent, "/api/event")]
pub async fn delete_event(event_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::events::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let eid: i32 = event_id.parse()
        .map_err(|_| Error::InternalError("Invalid event ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(events.filter(id.eq(eid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Event {} not found", event_id)).into());
    }
    broadcast_delete::<types::Event>(event_id);
    Ok(())
}
