use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::db::DB;
    pub use crate::common::types;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::Thing;
    use surrealdb::RecordId;
    pub use validator::Validate;
    pub const EVENTS: &str = "events";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Event {
        pub id: Option<RecordId>,
        #[validate(length(min = 1, max = 64))]
        pub name: String,
    }
    impl From<Event> for types::Event {
        fn from(event: Event) -> Self {
            Self {
                id: event.id.unwrap().to_string(),
                name: event.name,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateEvent, "/api/event")]
pub async fn create_event(req: requests::event::Create) -> Result<types::Event, ServerFnError> {
    let e: Option<Event> = DB.create(EVENTS)
        .content(Event {
            id: None,
            name: req.name,
        })
        .await?;
    e.map(Into::into).ok_or_else(|| ServerError("Failed to create event".into()))
}

#[server(GetEvents, "/api/event")]
pub async fn get_events() -> Result<Vec<types::Event>, ServerFnError> {
    let events: Vec<Event> = DB.select(EVENTS).await?;
    Ok(events.into_iter().map(Into::into).collect())
}

#[server(GetEvent, "/api/event")]
pub async fn get_event(id: String) -> Result<types::Event, ServerFnError> {
    DB.select((EVENTS, &id))
        .await?
        .ok_or_else(|| ServerError("Event not found".into()))
}

#[server(UpdateEvent, "/api/event")]
pub async fn update_event(
    id: String,
    update: requests::event::Update,
) -> Result<types::Event, ServerFnError> {
    DB.update((EVENTS, &id))
        .merge(update)
        .await?
        .ok_or_else(|| ServerError("Failed to update event".into()))
}

#[server(DeleteEvent, "/api/event")]
pub async fn delete_event(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Event> = DB.delete((EVENTS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Event with id {} not found", id)));
    }
    Ok(())
}
