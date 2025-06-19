use leptos::prelude::*;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest};

#[cfg(feature = "ssr")]
use crate::common::errors::{AppError, AppResult};
#[cfg(feature = "ssr")]
use crate::backend::database::model::order_item::OrderItemRecord;
#[cfg(feature = "ssr")]
use crate::backend::database::model::item::ItemRecord;
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

#[server(GetOrderItems, "/api")]
pub async fn get_order_items(order_id: String) -> Result<Vec<OrderItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let query = "SELECT * FROM order_items WHERE order_id = $order_id";
        let mut response = db
            .query(query)
            .bind(("order_id", order_id.to_string()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get order items: {}", e)))?;

        let order_items: Vec<OrderItemRecord> = response
            .take(0)
            .map_err(|e| ServerFnError::new(format!("Failed to parse order items query result: {}", e)))?;

        Ok(order_items.into_iter().map(|record| record.into()).collect())
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
        let db = get_db_connection().await?;
        
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        // Get current item price for price snapshotting
        let item: Option<ItemRecord> = db
            .select(("items", &request.item_id))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get item: {}", e)))?;

        let item = item.ok_or_else(|| ServerFnError::new(format!("Item with id {} not found", request.item_id)))?;

        #[derive(serde::Serialize)]
        struct CreateOrderItemData {
            order_id: String,
            item_id: String,
            quantity: u32,
            price: f64,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let order_item: Option<OrderItemRecord> = db
            .create("order_items")
            .content(CreateOrderItemData {
                order_id: request.order_id,
                item_id: request.item_id,
                quantity: request.quantity,
                price: item.price, // Snapshot current item price
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create order item: {}", e)))?;

        order_item.map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to create order item: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let order_item: Option<OrderItemRecord> = db
            .select(("order_items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get order item: {}", e)))?;

        match order_item {
            Some(record) => Ok(record.into()),
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
        let db = get_db_connection().await?;
        
        // First check if order item exists
        let existing: Option<OrderItemRecord> = db
            .select(("order_items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get order item: {}", e)))?;

        let mut existing = existing
            .ok_or_else(|| ServerFnError::new(format!("Order item with id {} not found", id)))?;

        // Update fields if provided
        if let Some(item_id) = request.item_id {
            // Get new item price for price snapshotting
            let item: Option<ItemRecord> = db
                .select(("items", &item_id))
                .await
                .map_err(|e| ServerFnError::new(format!("Failed to get item: {}", e)))?;

            let item = item.ok_or_else(|| ServerFnError::new(format!("Item with id {} not found", item_id)))?;

            existing.item_id = item_id;
            existing.price = item.price; // Update price when item changes
        }

        if let Some(quantity) = request.quantity {
            if quantity == 0 {
                return Err(ServerFnError::new("Quantity must be greater than 0".to_string()));
            }
            existing.quantity = quantity;
        }

        existing.updated_at = Datetime::default();

        let updated: Option<OrderItemRecord> = db
            .update(("order_items", id.as_str()))
            .content(existing)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to update order item: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to update order item: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let deleted: Option<OrderItemRecord> = db
            .delete(("order_items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to delete order item: {}", e)))?;

        if deleted.is_none() {
            return Err(ServerFnError::new(format!("Order item with id {} not found", id)));
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}