use leptos::prelude::*;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest, BulkOrderItemUpdate};

#[cfg(feature = "ssr")]
use crate::backend::{database, services::utils::*};

/// Get all order items for a specific order - requires staff level access
#[server(GetOrderItems, "/api")]
pub async fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::order_items::get_order_items(&db, &order_id)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get all order items across all orders - requires staff level access
#[server(GetAllOrderItems, "/api")]
pub async fn get_all_order_items() -> Result<Vec<OrderItem>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::order_items::get_all_order_items(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Create a new order item - requires cashier level access
#[server(CreateOrderItem, "/api")]
pub async fn create_order_item(request: CreateOrderItemRequest) -> Result<OrderItem, ServerFnError> {
    // Validate request
    request.validate().map_err(|e| ServerFnError::new(e))?;
    
    with_cashier_auth(|db, _user| async move {
        database::order_items::create_order_item(&db, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get a specific order item by ID - requires staff level access
#[server(GetOrderItem, "/api")]
pub async fn get_order_item(id: String) -> Result<OrderItem, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let order_item = database::order_items::get_order_item(&db, &id)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(order_item)
    }).await
}

/// Update an existing order item - requires staff level access (staff can update status)
#[server(UpdateOrderItem, "/api")]
pub async fn update_order_item(id: String, request: UpdateOrderItemRequest) -> Result<OrderItem, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::order_items::update_order_item(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Delete an order item - requires cashier level access
#[server(DeleteOrderItem, "/api")]
pub async fn delete_order_item(id: String) -> Result<(), ServerFnError> {
    with_cashier_auth(|db, _user| async move {
        database::order_items::delete_order_item(&db, &id)
            .await
            .map_err(map_db_error)
    }).await
}

/// Bulk update order items - requires staff level access (staff can bulk update)
#[server(BulkUpdateOrderItems, "/api")]
pub async fn bulk_update_order_items(update: BulkOrderItemUpdate) -> Result<Vec<OrderItem>, ServerFnError> {
    // Validate request
    update.validate().map_err(|e| ServerFnError::new(e))?;
    
    with_staff_auth(|db, _user| async move {
        database::order_items::bulk_update_order_items(&db, update)
            .await
            .map_err(map_db_error)
    }).await
}