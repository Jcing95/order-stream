use leptos::prelude::*;

use crate::common::types;

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateStation, "/api/station")]
pub async fn create_station(
    name: String,
    category_ids_json: String,
    input_statuses_json: String,
    output_status: types::OrderStatus,
) -> Result<types::Station, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbStation, NewStation, StationCategory, StationInputStatus, station_from_parts, order_status_str};
    use crate::backend::websocket::broadcast_add;
    use crate::backend::schema::stations::dsl::stations;
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let category_ids: Vec<String> = if category_ids_json.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&category_ids_json)
            .map_err(|_| Error::InternalError("Failed to parse category_ids".to_string()))?
    };

    let input_statuses: Vec<types::OrderStatus> = if input_statuses_json.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&input_statuses_json)
            .map_err(|_| Error::InternalError("Failed to parse input_statuses".to_string()))?
    };

    let cat_ids_i32: Vec<i32> = category_ids.iter()
        .map(|s| s.parse().map_err(|_| Error::InternalError("Invalid category ID".to_string())))
        .collect::<Result<_, _>>()?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let station: DbStation = diesel::insert_into(stations)
        .values(NewStation { name: &name, output_status: order_status_str(output_status) })
        .returning(DbStation::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create station: {}", e)))?;

    // Insert junction rows
    let sc_rows: Vec<StationCategory> = cat_ids_i32.iter()
        .map(|&cid| StationCategory { station_id: station.id, category_id: cid })
        .collect();
    if !sc_rows.is_empty() {
        diesel::insert_into(sc_dsl::station_categories)
            .values(&sc_rows)
            .execute(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;
    }

    let sis_rows: Vec<StationInputStatus> = input_statuses.iter()
        .map(|&s| StationInputStatus { station_id: station.id, status: order_status_str(s).to_string() })
        .collect();
    if !sis_rows.is_empty() {
        diesel::insert_into(sis_dsl::station_input_statuses)
            .values(&sis_rows)
            .execute(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;
    }

    let result = station_from_parts(
        station,
        cat_ids_i32,
        input_statuses.iter().map(|&s| order_status_str(s).to_string()).collect(),
    );
    broadcast_add(result.clone());
    Ok(result)
}

#[server(GetStations, "/api/station")]
pub async fn get_stations() -> Result<Vec<types::Station>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbStation, station_from_parts};
    use crate::backend::schema::stations::dsl::stations;
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let db_stations: Vec<DbStation> = stations
        .select(DbStation::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let mut result = Vec::new();
    for station in db_stations {
        let cat_ids: Vec<i32> = sc_dsl::station_categories
            .filter(sc_dsl::station_id.eq(station.id))
            .select(sc_dsl::category_id)
            .load(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;

        let input_status_strs: Vec<String> = sis_dsl::station_input_statuses
            .filter(sis_dsl::station_id.eq(station.id))
            .select(sis_dsl::status)
            .load(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;

        result.push(station_from_parts(station, cat_ids, input_status_strs));
    }
    Ok(result)
}

#[server(GetStation, "/api/station")]
pub async fn get_station(station_id: String) -> Result<types::Station, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbStation, station_from_parts};
    use crate::backend::schema::stations::dsl::{stations, id};
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let sid: i32 = station_id.parse()
        .map_err(|_| Error::InternalError("Invalid station ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let station: Option<DbStation> = stations
        .filter(id.eq(sid))
        .select(DbStation::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let station = station.ok_or_else(|| Error::InternalError("Station not found".to_string()))?;

    let cat_ids: Vec<i32> = sc_dsl::station_categories
        .filter(sc_dsl::station_id.eq(sid))
        .select(sc_dsl::category_id)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let input_status_strs: Vec<String> = sis_dsl::station_input_statuses
        .filter(sis_dsl::station_id.eq(sid))
        .select(sis_dsl::status)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(station_from_parts(station, cat_ids, input_status_strs))
}

#[server(UpdateStation, "/api/station")]
pub async fn update_station(
    station_id: String,
    name: String,
    category_ids_json: String,
    input_statuses_json: String,
    output_status: types::OrderStatus,
) -> Result<types::Station, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbStation, UpdateStation, StationCategory, StationInputStatus, station_from_parts, order_status_str};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::stations::dsl::{stations, id};
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let sid: i32 = station_id.parse()
        .map_err(|_| Error::InternalError("Invalid station ID".to_string()))?;

    let category_ids: Vec<String> = if category_ids_json.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&category_ids_json)
            .map_err(|_| Error::InternalError("Failed to parse category_ids".to_string()))?
    };

    let input_statuses: Vec<types::OrderStatus> = if input_statuses_json.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&input_statuses_json)
            .map_err(|_| Error::InternalError("Failed to parse input_statuses".to_string()))?
    };

    let cat_ids_i32: Vec<i32> = category_ids.iter()
        .map(|s| s.parse().map_err(|_| Error::InternalError("Invalid category ID".to_string())))
        .collect::<Result<_, _>>()?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let station: DbStation = diesel::update(stations.filter(id.eq(sid)))
        .set(UpdateStation {
            name: Some(name),
            output_status: Some(order_status_str(output_status).to_string()),
        })
        .returning(DbStation::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update station: {}", e)))?;

    // Replace junction rows
    diesel::delete(sc_dsl::station_categories.filter(sc_dsl::station_id.eq(sid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    diesel::delete(sis_dsl::station_input_statuses.filter(sis_dsl::station_id.eq(sid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let sc_rows: Vec<StationCategory> = cat_ids_i32.iter()
        .map(|&cid| StationCategory { station_id: sid, category_id: cid })
        .collect();
    if !sc_rows.is_empty() {
        diesel::insert_into(sc_dsl::station_categories)
            .values(&sc_rows)
            .execute(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;
    }

    let sis_rows: Vec<StationInputStatus> = input_statuses.iter()
        .map(|&s| StationInputStatus { station_id: sid, status: order_status_str(s).to_string() })
        .collect();
    if !sis_rows.is_empty() {
        diesel::insert_into(sis_dsl::station_input_statuses)
            .values(&sis_rows)
            .execute(&mut conn)
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;
    }

    let result = station_from_parts(
        station,
        cat_ids_i32,
        input_statuses.iter().map(|&s| order_status_str(s).to_string()).collect(),
    );
    broadcast_update(result.clone());
    Ok(result)
}

#[server(DeleteStation, "/api/station")]
pub async fn delete_station(station_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::stations::dsl::{stations, id};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let sid: i32 = station_id.parse()
        .map_err(|_| Error::InternalError("Invalid station ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(stations.filter(id.eq(sid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Station {} not found", station_id)).into());
    }
    broadcast_delete::<types::Station>(station_id);
    Ok(())
}
