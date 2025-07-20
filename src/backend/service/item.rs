use crate::common::{types::Item, requests::item};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::backend::db;
#[cfg(feature = "ssr")]
use crate::backend::error::Error;

#[server(GetItems, "/api")]
pub async fn get_items(order_id: String) -> Result<Vec<Item>, ServerFnError> {
    let db = db::get_db_connection()
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?;

    db::item::get_order_items(&db, &order_id)
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))
}

#[server(CreateItem, "/api")]
pub async fn create_item(request: item::Create) -> Result<Item, ServerFnError> {
    // Validation happens in service layer
    request.validate().map_err(|e| ServerFnError::new(e))?;

    let db = db::get_db_connection()
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?;

    db::item::create_order_item(&db, request)
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))
}

#[server(GetItem, "/api")]
pub async fn get_item(id: String) -> Result<Item, ServerFnError> {
    let db = db::get_db_connection()
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?;

    match db::item::get_order_item(&db, &id)
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?
    {
        Some(order_item) => Ok(order_item),
        None => Err(ServerFnError::new(format!(
            "Order item with id {} not found",
            id
        ))),
    }
}

#[server(UpdateItem, "/api")]
pub async fn update_item(
    id: String,
    request: item::Update,
) -> Result<Item, ServerFnError> {
    let db = db::get_db_connection()
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?;

    db::item::update_order_item(&db, &id, request)
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))
}

#[server(DeleteItem, "/api")]
pub async fn delete_item(id: String) -> Result<(), ServerFnError> {
    let db = db::get_db_connection()
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))?;

    db::item::delete_item(&db, &id)
        .await
        .map_err(|e: Error| ServerFnError::new(e.to_string()))
}
