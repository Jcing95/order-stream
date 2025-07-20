use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

use crate::common::{errors::Error, requests, types};

use super::DB;
const STATIONS: &str = "stations";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Station {
    pub id: Option<Thing>,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<types::OrderStatus>,
    pub output_status: types::OrderStatus,
}

impl From<Station> for types::Station {
    fn from(station: Station) -> Self {
        Self {
            id: station.id.unwrap().id.to_string(),
            name: station.name,
            category_ids: station.category_ids,
            input_statuses: station.input_statuses,
            output_status: station.output_status,
        }
    }
}

pub async fn create_station(request: requests::station::Create) -> Result<types::Station, Error> {
    DB.create((STATIONS, &request.name))
        .content(Station {
            id: None,
            name: request.name,
            category_ids: request.category_ids,
            input_statuses: request.input_statuses,
            output_status: request.output_status,
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create station".into()))
}

pub async fn get_stations() -> Result<Vec<types::Station>, Error> {
    let stations: Vec<Station> = DB.select(STATIONS).await?;
    Ok(stations.into_iter().map(Into::into).collect())
}

pub async fn get_station(name: &str) -> Result<types::Station, Error> {
    DB.select((STATIONS, name))
        .await?
        .ok_or_else(|| Error::NotFound("Station not found".into()))
}

pub async fn update_station(
    name: &str,
    update: requests::station::Update,
) -> Result<types::Station, Error> {
    DB.update((STATIONS, name))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update station".into()))
}

pub async fn delete_station(id: &str) -> Result<(), Error> {
    let deleted: Option<Station> = DB.delete((STATIONS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Station with id {} not found", id)));
    }
    Ok(())
}
