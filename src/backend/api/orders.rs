use leptos::prelude::*;
use crate::common::types::{Order, UpdateOrderRequest};


#[server(GetOrders, "/api")]
pub async fn get_orders() -> Result<Vec<Order>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::orders;
        
        let db = super::get_db_connection().await?;
        let orders = orders::Service::get_orders(&db).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(orders)
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
        use crate::common::errors::AppError;
        use crate::backend::services::orders;
        
        let db = super::get_db_connection().await?;
        let order = orders::Service::create_order(&db).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(order)
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
        use crate::common::errors::AppError;
        use crate::backend::services::orders;
        
        let db = super::get_db_connection().await?;
        let order = orders::Service::get_order(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        match order {
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
        use crate::common::errors::AppError;
        use crate::backend::services::orders;
        
        let db = super::get_db_connection().await?;
        let order = orders::Service::update_order(&db, &id, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(order)
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
        use crate::common::errors::AppError;
        use crate::backend::services::orders;
        
        let db = super::get_db_connection().await?;
        orders::Service::delete_order(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}