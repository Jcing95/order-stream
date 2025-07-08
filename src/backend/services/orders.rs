use leptos::prelude::*;
use crate::common::types::{Order, UpdateOrderRequest, OrderStatus};

#[cfg(feature = "ssr")]
use crate::backend::{database, services::utils::*};

/// Get all orders - requires staff level access
#[server(GetOrders, "/api")]
pub async fn get_orders() -> Result<Vec<Order>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::orders::get_orders(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Create a new order - requires cashier level access
#[server(CreateOrder, "/api")]
pub async fn create_order() -> Result<Order, ServerFnError> {
    with_cashier_auth(|db, _user| async move {
        database::orders::create_order(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get a specific order by ID - requires staff level access
#[server(GetOrder, "/api")]
pub async fn get_order(id: String) -> Result<Order, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let order = database::orders::get_order(&db, &id)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(order)
    }).await
}

/// Update an existing order - requires cashier level access
#[server(UpdateOrder, "/api")]
pub async fn update_order(id: String, request: UpdateOrderRequest) -> Result<Order, ServerFnError> {
    with_cashier_auth(|db, _user| async move {
        database::orders::update_order(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Update order status - requires staff level access (staff can update status)
#[server(UpdateOrderStatus, "/api")]
pub async fn update_order_status(id: String, status: OrderStatus) -> Result<Order, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let request = UpdateOrderRequest {
            status: Some(status),
        };
        
        database::orders::update_order(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Delete an order - requires cashier level access
#[server(DeleteOrder, "/api")]
pub async fn delete_order(id: String) -> Result<(), ServerFnError> {
    with_cashier_auth(|db, _user| async move {
        database::orders::delete_order(&db, &id)
            .await
            .map_err(map_db_error)
    }).await
}