use leptos::prelude::*;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest};


#[server(GetOrderItems, "/api")]
pub async fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::order_items;
        
        let db = super::get_db_connection().await?;
        let order_items = order_items::Service::get_order_items(&db, &order_id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(order_items)
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
        use crate::common::errors::AppError;
        use crate::backend::services::order_items;
        
        let db = super::get_db_connection().await?;
        let order_item = order_items::Service::create_order_item(&db, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(order_item)
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
        use crate::common::errors::AppError;
        use crate::backend::services::order_items;
        
        let db = super::get_db_connection().await?;
        let order_item = order_items::Service::get_order_item(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        match order_item {
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
        use crate::common::errors::AppError;
        use crate::backend::services::order_items;
        
        let db = super::get_db_connection().await?;
        let order_item = order_items::Service::update_order_item(&db, &id, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(order_item)
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
        use crate::common::errors::AppError;
        use crate::backend::services::order_items;
        
        let db = super::get_db_connection().await?;
        order_items::Service::delete_order_item(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}