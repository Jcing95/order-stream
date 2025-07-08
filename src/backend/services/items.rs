use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};

#[cfg(feature = "ssr")]
use crate::backend::{database, services::utils::*};

/// Get all items - requires staff level access
#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::items::get_items(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Create a new item - requires admin access
#[server(CreateItem, "/api")]
pub async fn create_item(request: CreateItemRequest) -> Result<Item, ServerFnError> {
    // Validate request
    request.validate().map_err(|e| ServerFnError::new(e))?;
    
    with_admin_auth(|db, _user| async move {
        database::items::create_item(&db, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get a specific item by ID - requires staff level access
#[server(GetItem, "/api")]
pub async fn get_item(id: String) -> Result<Item, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let item = database::items::get_item(&db, &id)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(item)
    }).await
}

/// Update an existing item - requires admin access
#[server(UpdateItem, "/api")]
pub async fn update_item(id: String, request: UpdateItemRequest) -> Result<Item, ServerFnError> {
    with_admin_auth(|db, _user| async move {
        database::items::update_item(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Delete an item - requires admin access
#[server(DeleteItem, "/api")]
pub async fn delete_item(id: String) -> Result<(), ServerFnError> {
    with_admin_auth(|db, _user| async move {
        database::items::delete_item(&db, &id)
            .await
            .map_err(map_db_error)
    }).await
}