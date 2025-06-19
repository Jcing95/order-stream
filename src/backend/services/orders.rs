use leptos::prelude::*;
use crate::common::types::{Order, UpdateOrderRequest};

#[cfg(feature = "ssr")]
use crate::backend::database::model::order::OrderRecord;
#[cfg(feature = "ssr")]
use crate::backend::database::Database;
#[cfg(feature = "ssr")]
use surrealdb::sql::Datetime;

#[cfg(feature = "ssr")]
async fn get_db_connection() -> Result<Database, ServerFnError> {
    use crate::backend::database::connect_database;
    use crate::backend::config::AppConfig;
    
    let config = AppConfig::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;
    let db = connect_database(&config.database).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(db)
}

#[cfg(feature = "ssr")]
async fn get_next_sequential_id(db: &Database) -> Result<u32, ServerFnError> {
    // Query for the maximum sequential_id
    let query = "SELECT VALUE sequential_id FROM orders ORDER BY sequential_id DESC LIMIT 1";
    let mut response = db
        .query(query)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get next sequential ID: {}", e)))?;

    let max_id: Option<u32> = response
        .take(0)
        .map_err(|e| ServerFnError::new(format!("Failed to parse sequential ID query result: {}", e)))?;

    Ok(max_id.map(|id| id + 1).unwrap_or(1))
}

#[server(GetOrders, "/api")]
pub async fn get_orders() -> Result<Vec<Order>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let orders: Vec<OrderRecord> = db
            .select("orders")
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get orders: {}", e)))?;

        Ok(orders.into_iter().map(|record| record.into()).collect())
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
        use crate::common::types::OrderStatus;
        
        let db = get_db_connection().await?;
        
        // Get next sequential ID
        let sequential_id = get_next_sequential_id(&db).await?;

        #[derive(serde::Serialize)]
        struct CreateOrderData {
            sequential_id: u32,
            event_id: String,
            total_price: f64,
            status: OrderStatus,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let order: Option<OrderRecord> = db
            .create("orders")
            .content(CreateOrderData {
                sequential_id,
                event_id: "default".to_string(), // Future multi-event support
                total_price: 0.0,
                status: OrderStatus::Draft,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create order: {}", e)))?;

        order.map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to create order: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let order: Option<OrderRecord> = db
            .select(("orders", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get order: {}", e)))?;

        match order {
            Some(record) => Ok(record.into()),
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
        let db = get_db_connection().await?;
        
        // First check if order exists
        let existing: Option<OrderRecord> = db
            .select(("orders", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get order: {}", e)))?;

        let mut existing = existing
            .ok_or_else(|| ServerFnError::new(format!("Order with id {} not found", id)))?;

        // Update fields if provided
        if let Some(status) = request.status {
            existing.status = status;
        }

        existing.updated_at = Datetime::default();

        let updated: Option<OrderRecord> = db
            .update(("orders", id.as_str()))
            .content(existing)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to update order: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to update order: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let deleted: Option<OrderRecord> = db
            .delete(("orders", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to delete order: {}", e)))?;

        if deleted.is_none() {
            return Err(ServerFnError::new(format!("Order with id {} not found", id)));
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}