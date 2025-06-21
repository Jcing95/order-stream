use leptos::prelude::*;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest};

#[cfg(feature = "ssr")]
use crate::backend::errors::AppError;
#[cfg(feature = "ssr")]
use crate::backend::database;

#[server(GetOrderItems, "/api")]
pub async fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::order_items::get_order_items(&db, &order_id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::order_items::get_all_order_items(&db)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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

        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::order_items::create_order_item(&db, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        match database::order_items::get_order_item(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::order_items::update_order_item(&db, &id, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::order_items::delete_order_item(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}