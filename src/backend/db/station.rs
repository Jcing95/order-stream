use super::Database;
use crate::backend::error::Error;
use crate::common::{types, requests};
use serde::{Deserialize, Serialize};
use validator::Validate;

const STATIONS: &str = "stations";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Station {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<types::OrderStatus>,
    pub output_status: types::OrderStatus,
}

impl From<Station> for types::Station {
    fn from(station: Station) -> Self {
        Self {
            name: station.name,
            category_ids: station.category_ids,
            input_statuses: station.input_statuses,
            output_status: station.output_status,
        }
    }
}

pub async fn create_station(
    db: &Database,
    request: requests::station::Create,
) -> Result<types::Station, Error> {
    db.create((STATIONS, &request.name))
        .content(Station {
            name: request.name,
            category_ids: request.category_ids,
            input_statuses: request.input_statuses,
            output_status: request.output_status,
        })
        .await?
        .ok_or_else(|| Error::InternalError(format!("Failed to create station.")))
}

pub async fn get_stations(db: &Database) -> Result<Vec<types::Station>, Error> {
    let stations: Vec<Station> = db.select(STATIONS).await?;
    Ok(stations.into_iter().map(Into::into).collect())
}

pub async fn get_station(db: &Database, name: &str) -> Result<types::Station, Error> {
    db.select((STATIONS, name))
        .await?
        .ok_or_else(|| Error::NotFound(format!("Not Found")))
}

pub async fn update_station(
    db: &Database,
    name: &str,
    update: requests::station::Update,
) -> Result<types::Station, Error> {
    db.update((STATIONS, name))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError(format!("Failed to update station.")))
}

pub async fn delete_station(db: &Database, id: &str) -> Result<(), Error> {
    let deleted: Option<Station> = db.delete((STATIONS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Station with id {} not found", id)));
    }
    Ok(())
}
