use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

use crate::common::{errors::Error, requests, types};

use super::DB;
const EVENTS: &str = "events";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Event {
    pub id: Option<Thing>,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
}

impl From<Event> for types::Event {
    fn from(event: Event) -> Self {
        Self {
            id: event.id.unwrap().id.to_string(),
            name: event.name,
        }
    }
}

pub async fn create_event(req: requests::event::Create) -> Result<types::Event, Error> {
    DB.create(EVENTS)
        .content(Event {
            id: None,
            name: req.name,
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create event".into()))
}

pub async fn get_events() -> Result<Vec<types::Event>, Error> {
    let events: Vec<Event> = DB.select(EVENTS).await?;
    Ok(events.into_iter().map(Into::into).collect())
}

pub async fn get_event(id: &str) -> Result<types::Event, Error> {
    DB.select((EVENTS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Event not found".into()))
}

pub async fn update_event(
    id: &str,
    update: requests::event::Update,
) -> Result<types::Event, Error> {
    DB.update((EVENTS, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update event".into()))
}

pub async fn delete_event(id: &str) -> Result<(), Error> {
    let deleted: Option<Event> = DB.delete((EVENTS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Event with id {} not found", id)));
    }
    Ok(())
}
