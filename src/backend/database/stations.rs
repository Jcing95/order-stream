use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};

use crate::backend::errors::{AppError, AppResult};
use crate::common::types;

use super::{Database, validators};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationRecord {
    pub id: Thing,
    pub name: String,
    pub category_ids: Vec<String>,       // JSON array in database
    pub input_statuses: Vec<types::OrderStatus>, // JSON array in database  
    pub output_status: types::OrderStatus,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<StationRecord> for types::Station {
    fn from(record: StationRecord) -> Self {
        Self {
            id: record.id.id.to_string(), // Extract just the UUID part
            name: record.name,
            category_ids: record.category_ids,
            input_statuses: record.input_statuses,
            output_status: record.output_status,
        }
    }
}

pub async fn create_station(db: &Database, request: types::CreateStationRequest) -> AppResult<types::Station> {
    #[derive(serde::Serialize)]
    struct CreateStationData {
        name: String,
        category_ids: Vec<String>,
        input_statuses: Vec<types::OrderStatus>,
        output_status: types::OrderStatus,
        created_at: Datetime,
        updated_at: Datetime,
    }

    let station: Option<StationRecord> = db
        .create("stations")
        .content(CreateStationData {
            name: request.name,
            category_ids: request.category_ids,
            input_statuses: request.input_statuses,
            output_status: request.output_status,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create station: {}", e)))?;

    station.map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to create station: no record returned from database".to_string()))
}

pub async fn get_stations(db: &Database) -> AppResult<Vec<types::Station>> {
    let stations: Vec<StationRecord> = db
        .select("stations")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get stations: {}", e)))?;

    Ok(stations.into_iter().map(|record| record.into()).collect())
}

pub async fn get_station(db: &Database, id: &str) -> AppResult<Option<types::Station>> {
    let station: Option<StationRecord> = db
        .select(("stations", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get station: {}", e)))?;

    Ok(station.map(|record| record.into()))
}

pub async fn get_station_by_name(db: &Database, name: &str) -> AppResult<Option<types::Station>> {
    // Try exact match first (for backward compatibility)
    let query = "SELECT * FROM stations WHERE name = $name LIMIT 1";
    let mut response = db
        .query(query)
        .bind(("name", name.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get station by name: {}", e)))?;

    let stations: Vec<StationRecord> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse station query result: {}", e)))?;

    // If exact match found, return it
    if let Some(station) = stations.into_iter().next() {
        return Ok(Some(station.into()));
    }

    // If no exact match, try case-insensitive match
    let query = "SELECT * FROM stations WHERE string::lowercase(name) = string::lowercase($name) LIMIT 1";
    let mut response = db
        .query(query)
        .bind(("name", name.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get station by name (case-insensitive): {}", e)))?;

    let stations: Vec<StationRecord> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse station query result (case-insensitive): {}", e)))?;

    Ok(stations.into_iter().next().map(|record| record.into()))
}

pub async fn update_station(
    db: &Database,
    id: &str,
    request: types::UpdateStationRequest,
) -> AppResult<types::Station> {
    // First check if station exists
    let existing: Option<StationRecord> = db
        .select(("stations", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get station: {}", e)))?;

    let mut existing = existing
        .ok_or_else(|| AppError::NotFound(format!("Station with id {} not found", id)))?;

    // Update fields if provided
    if let Some(name) = request.name {
        validators::non_empty_string(&name, "Name")?;
        existing.name = name;
    }

    if let Some(category_ids) = request.category_ids {
        existing.category_ids = category_ids;
    }

    if let Some(input_statuses) = request.input_statuses {
        existing.input_statuses = input_statuses;
    }

    if let Some(output_status) = request.output_status {
        existing.output_status = output_status;
    }

    existing.updated_at = Datetime::default();

    let updated: Option<StationRecord> = db
        .update(("stations", id))
        .content(existing)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update station: {}", e)))?;

    updated
        .map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to update station: no record returned from database".to_string()))
}

pub async fn delete_station(db: &Database, id: &str) -> AppResult<()> {
    let deleted: Option<StationRecord> = db
        .delete(("stations", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete station: {}", e)))?;

    if deleted.is_none() {
        return Err(AppError::NotFound(format!("Station with id {} not found", id)));
    }

    Ok(())
}