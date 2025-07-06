use leptos::prelude::*;
use crate::common::types::{Station, CreateStationRequest, UpdateStationRequest};

#[cfg(feature = "ssr")]
use crate::backend::{database, services::utils::*};

/// Get all stations - requires staff level access
#[server(GetStations, "/api")]
pub async fn get_stations() -> Result<Vec<Station>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::stations::get_stations(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Create a new station - requires admin access
#[server(CreateStation, "/api")]
pub async fn create_station(request: CreateStationRequest) -> Result<Station, ServerFnError> {
    // Validate request
    request.validate().map_err(|e| ServerFnError::new(e))?;
    
    with_admin_auth(|db, _user| async move {
        database::stations::create_station(&db, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get a specific station by ID - requires staff level access
#[server(GetStation, "/api")]
pub async fn get_station(id: String) -> Result<Station, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let station = database::stations::get_station(&db, &id)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(station)
    }).await
}

/// Get a station by name - requires staff level access
#[server(GetStationByName, "/api")]
pub async fn get_station_by_name(name: String) -> Result<Station, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let station = database::stations::get_station_by_name(&db, &name)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(station)
    }).await
}

/// Update an existing station - requires admin access
#[server(UpdateStation, "/api")]
pub async fn update_station(id: String, request: UpdateStationRequest) -> Result<Station, ServerFnError> {
    with_admin_auth(|db, _user| async move {
        database::stations::update_station(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Delete a station - requires admin access
#[server(DeleteStation, "/api")]
pub async fn delete_station(id: String) -> Result<(), ServerFnError> {
    with_admin_auth(|db, _user| async move {
        database::stations::delete_station(&db, &id)
            .await
            .map_err(map_db_error)
    }).await
}