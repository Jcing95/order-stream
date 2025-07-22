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
    pub const STATIONS: &str = "stations";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Station {
        pub id: Option<RecordId>,
        #[validate(length(min = 1, max = 64))]
        pub name: String,
        pub category_ids: Vec<String>,
        pub input_statuses: Vec<types::OrderStatus>,
        pub output_status: types::OrderStatus,
    }

    impl From<Station> for types::Station {
        fn from(station: Station) -> Self {
            Self {
                id: station.id.unwrap().key().to_string(),
                name: station.name,
                category_ids: station.category_ids,
                input_statuses: station.input_statuses,
                output_status: station.output_status,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateStation, "/api/station")]
pub async fn create_station(
    request: requests::station::Create,
) -> Result<types::Station, ServerFnError> {
    let s: Option<Station> = DB.create((STATIONS, &request.name))
        .content(Station {
            id: None,
            name: request.name,
            category_ids: request.category_ids,
            input_statuses: request.input_statuses,
            output_status: request.output_status,
        })
        .await?;
    s.map(Into::into).ok_or_else(|| ServerError("Failed to create station".into()))
}

#[server(GetStations, "/api/station")]
pub async fn get_stations() -> Result<Vec<types::Station>, ServerFnError> {
    let stations: Vec<Station> = DB.select(STATIONS).await?;
    Ok(stations.into_iter().map(Into::into).collect())
}

#[server(GetStation, "/api/station")]
pub async fn get_station(name: String) -> Result<types::Station, ServerFnError> {
    DB.select((STATIONS, &name))
        .await?
        .ok_or_else(|| ServerError("Station not found".into()))
}

#[server(UpdateStation, "/api/station")]
pub async fn update_station(
    name: String,
    update: requests::station::Update,
) -> Result<types::Station, ServerFnError> {
    DB.update((STATIONS, &name))
        .merge(update)
        .await?
        .ok_or_else(|| ServerError("Failed to update station".into()))
}

#[server(DeleteStation, "/api/station")]
pub async fn delete_station(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Station> = DB.delete((STATIONS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Station with id {} not found", id)));
    }
    Ok(())
}
