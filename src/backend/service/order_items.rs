use leptos::prelude::*;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest, BulkOrderItemUpdate};

#[cfg(feature = "ssr")]
use crate::backend::error::Error;
#[cfg(feature = "ssr")]
use crate::backend::db;

#[server(GetOrderItems, "/api")]
pub async fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::get_order_items(&db, &order_id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetAllOrderItems, "/api")]
pub async fn get_all_order_items() -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::get_all_order_items(&db)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateOrderItem, "/api")]
pub async fn create_order_item(request: CreateOrderItemRequest) -> Result<OrderItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // Validation happens in service layer
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::create_order_item(&db, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetOrderItem, "/api")]
pub async fn get_order_item(id: String) -> Result<OrderItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        match db::item::get_order_item(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?
        {
            Some(order_item) => Ok(order_item),
            None => Err(ServerFnError::new(format!("Order item with id {} not found", id))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateOrderItem, "/api")]
pub async fn update_order_item(id: String, request: UpdateOrderItemRequest) -> Result<OrderItem, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::update_order_item(&db, &id, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(DeleteOrderItem, "/api")]
pub async fn delete_order_item(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::delete_order_item(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(BulkUpdateOrderItems, "/api")]
pub async fn bulk_update_order_items(update: BulkOrderItemUpdate) -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // Validation happens in service layer
        update
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::item::bulk_update_order_items(&db, update)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}