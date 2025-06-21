use leptos::prelude::*;
use crate::common::types::{Order, UpdateOrderRequest, OrderStatus};

#[cfg(feature = "ssr")]
use crate::backend::errors::AppError;
#[cfg(feature = "ssr")]
use crate::backend::database;

#[server(GetOrders, "/api")]
pub async fn get_orders() -> Result<Vec<Order>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::orders::get_orders(&db)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateOrder, "/api")]
pub async fn create_order() -> Result<Order, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::orders::create_order(&db)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetOrder, "/api")]
pub async fn get_order(id: String) -> Result<Order, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        match database::orders::get_order(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?
        {
            Some(order) => Ok(order),
            None => Err(ServerFnError::new(format!("Order with id {} not found", id))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateOrder, "/api")]
pub async fn update_order(id: String, request: UpdateOrderRequest) -> Result<Order, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::orders::update_order(&db, &id, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateOrderStatus, "/api")]
pub async fn update_order_status(id: String, status: OrderStatus) -> Result<Order, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        let request = UpdateOrderRequest {
            status: Some(status),
        };
        
        database::orders::update_order(&db, &id, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(DeleteOrder, "/api")]
pub async fn delete_order(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::orders::delete_order(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}