use leptos::prelude::*;
use crate::common::types::{Station, CreateStationRequest, StationUpdate};

#[cfg(feature = "ssr")]
use crate::backend::error::Error;
#[cfg(feature = "ssr")]
use crate::backend::db;

#[server(GetStations, "/api")]
pub async fn get_stations() -> Result<Vec<Station>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::station::get_stations(&db)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateStation, "/api")]
pub async fn create_station(request: CreateStationRequest) -> Result<Station, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // Validation happens in service layer
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::station::create_station(&db, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetStation, "/api")]
pub async fn get_station(id: String) -> Result<Station, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        match db::station::get_station(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?
        {
            Some(station) => Ok(station),
            None => Err(ServerFnError::new(format!("Station with id {} not found", id))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetStationByName, "/api")]
pub async fn get_station_by_name(name: String) -> Result<Station, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        match db::station::get_station_by_name(&db, &name)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?
        {
            Some(station) => Ok(station),
            None => Err(ServerFnError::new(format!("Station with name '{}' not found", name))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateStation, "/api")]
pub async fn update_station(id: String, request: StationUpdate) -> Result<Station, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::station::update_station(&db, &id, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(DeleteStation, "/api")]
pub async fn delete_station(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::station::delete_station(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}