use leptos::prelude::*;

use crate::common::types;

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
    name: String,
    category_ids_json: String,
    input_statuses_json: String,
    output_status: types::OrderStatus,
) -> Result<types::Station, ServerFnError> {
    leptos::logging::log!("Creating station with name: {}", name);
    leptos::logging::log!("Raw category_ids_json: {}", category_ids_json);
    leptos::logging::log!("Raw input_statuses_json: {}", input_statuses_json);
    
    // Deserialize the JSON arrays
    let category_ids: Vec<String> = if category_ids_json.is_empty() {
        Vec::new()
    } else {
        match serde_json::from_str(&category_ids_json) {
            Ok(ids) => ids,
            Err(_) => {
                leptos::logging::log!("Failed to parse category_ids_json: {}", category_ids_json);
                return Err(ServerError("Failed to parse category_ids".into()));
            }
        }
    };
    
    let input_statuses: Vec<types::OrderStatus> = if input_statuses_json.is_empty() {
        Vec::new()
    } else {
        match serde_json::from_str(&input_statuses_json) {
            Ok(statuses) => statuses,
            Err(_) => {
                leptos::logging::log!("Failed to parse input_statuses_json: {}", input_statuses_json);
                return Err(ServerError("Failed to parse input_statuses".into()));
            }
        }
    };
    
    leptos::logging::log!("Parsed category_ids: {:?}", category_ids);
    leptos::logging::log!("Parsed input_statuses: {:?}", input_statuses);
    
    let s: Option<Station> = DB.create(STATIONS)
        .content(Station {
            id: None,
            name: name.clone(),
            category_ids,
            input_statuses,
            output_status,
        })
        .await?;
    if let Some(station) = s {
        let result: types::Station = station.into();
        broadcast_add(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to create station".into()))
    }
}

#[server(GetStations, "/api/station")]
pub async fn get_stations() -> Result<Vec<types::Station>, ServerFnError> {
    let stations: Vec<Station> = DB.select(STATIONS).await?;
    Ok(stations.into_iter().map(Into::into).collect())
}

#[server(GetStation, "/api/station")]
pub async fn get_station(name: String) -> Result<types::Station, ServerFnError> {
    let station: Option<Station> = DB.select((STATIONS, &name)).await?;
    station
        .map(Into::into)
        .ok_or_else(|| ServerError("Station not found".into()))
}

#[server(UpdateStation, "/api/station")]
pub async fn update_station(
    id: String,
    name: String,
    category_ids_json: String,
    input_statuses_json: String,
    output_status: types::OrderStatus,
) -> Result<types::Station, ServerFnError> {
    leptos::logging::log!("Updating station with id: {}, name: {}", id, name);
    leptos::logging::log!("Raw category_ids_json: {}", category_ids_json);
    leptos::logging::log!("Raw input_statuses_json: {}", input_statuses_json);
    
    // Deserialize the JSON arrays
    let category_ids: Vec<String> = if category_ids_json.is_empty() {
        Vec::new()
    } else {
        match serde_json::from_str(&category_ids_json) {
            Ok(ids) => ids,
            Err(_) => {
                leptos::logging::log!("Failed to parse category_ids_json: {}", category_ids_json);
                return Err(ServerError("Failed to parse category_ids".into()));
            }
        }
    };
    
    let input_statuses: Vec<types::OrderStatus> = if input_statuses_json.is_empty() {
        Vec::new()
    } else {
        match serde_json::from_str(&input_statuses_json) {
            Ok(statuses) => statuses,
            Err(_) => {
                leptos::logging::log!("Failed to parse input_statuses_json: {}", input_statuses_json);
                return Err(ServerError("Failed to parse input_statuses".into()));
            }
        }
    };
    
    leptos::logging::log!("Parsed category_ids: {:?}", category_ids);
    leptos::logging::log!("Parsed input_statuses: {:?}", input_statuses);
    
    // Get the existing station
    let existing_station: Option<Station> = DB.select((STATIONS, &id)).await?;
    if existing_station.is_none() {
        return Err(ServerError("Station not found".into()));
    }
    let station = existing_station.unwrap();
    let updated = Station {
        id: station.id,
        name, // Allow name to be updated
        category_ids,
        input_statuses,
        output_status,
    };
    // Update the station in the database
    let updated_station: Option<Station> = DB
        .update((STATIONS, &id))
        .content(updated)
        .await?;
        
    if let Some(station) = updated_station {
        let result: types::Station = station.into();
        broadcast_update(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to update station".into()))
    }
}

#[server(DeleteStation, "/api/station")]
pub async fn delete_station(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Station> = DB.delete((STATIONS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Station with id {} not found", id)));
    }
    broadcast_delete::<types::Station>(id);
    Ok(())
}
