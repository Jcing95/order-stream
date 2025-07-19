use super::Database;
use crate::backend::error::Error;
use crate::common::{types, requests};
use serde::{Deserialize, Serialize};
use validator::Validate;
use surrealdb::sql::Thing;

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
            name: event.name,
        }
    }
}

pub async fn create_event(
    db: &Database,
    req: requests::event::Create,
) -> Result<types::Event, Error> {
    db.create(EVENTS)
        .content(Event {
            id: None,
            name: req.name,
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create event".into()))
}

pub async fn get_events(db: &Database) -> Result<Vec<types::Event>, Error> {
    let events: Vec<Event> = db.select(EVENTS).await?;
    Ok(events.into_iter().map(Into::into).collect())
}

pub async fn get_event(db: &Database, id: &str) -> Result<types::Event, Error> {
    db.select((EVENTS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Event not found".into()))
}

pub async fn update_event(
    db: &Database,
    id: &str,
    update: requests::event::Update,
) -> Result<types::Event, Error> {
    db.update((EVENTS, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update event".into()))
}

pub async fn delete_event(db: &Database, id: &str) -> Result<(), Error> {
    let deleted: Option<Event> = db.delete((EVENTS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Event with id {} not found", id)));
    }
    Ok(())
}