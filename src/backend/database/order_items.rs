use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};

use crate::backend::errors::{AppError, AppResult};
use crate::common::types;

use super::{Database, validators};
use super::items::ItemRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRecord {
    pub id: Thing,
    pub order_id: String,     // FK to OrderRecord
    pub item_id: String,      // FK to ItemRecord  
    pub quantity: u32,
    pub price: f64,          // Snapshot of item price at order time
    pub status: types::OrderStatus, // Individual item status
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<OrderItemRecord> for types::OrderItem {
    fn from(record: OrderItemRecord) -> Self {
        Self {
            id: record.id.id.to_string(), // Extract just the UUID part
            order_id: record.order_id,
            item_id: record.item_id,
            quantity: record.quantity,
            price: record.price,
            status: record.status,
        }
    }
}


pub async fn create_order_item(db: &Database, request: types::CreateOrderItemRequest) -> AppResult<types::OrderItem> {
    // Get current item price for price snapshotting
    // Use item ID directly since it's now a clean UUID
    let item: Option<ItemRecord> = db
        .select(("items", &request.item_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

    let item = item.ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", request.item_id)))?;
    
    // Get order to determine initial OrderItem status
    let order: Option<super::orders::OrderRecord> = db
        .select(("orders", &request.order_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order: {}", e)))?;

    let order = order.ok_or_else(|| AppError::NotFound(format!("Order with id {} not found", request.order_id)))?;
    
    // Set OrderItem status based on Order status
    let item_status = match order.status {
        types::OrderStatus::Draft => types::OrderStatus::Draft,
        types::OrderStatus::Ordered => types::OrderStatus::Ordered,
        types::OrderStatus::Ready => types::OrderStatus::Ready,
        types::OrderStatus::Completed => types::OrderStatus::Completed,
        types::OrderStatus::Cancelled => types::OrderStatus::Cancelled,
    };

    #[derive(serde::Serialize)]
    struct CreateOrderItemData {
        order_id: String,
        item_id: String,
        quantity: u32,
        price: f64,
        status: types::OrderStatus,
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
            status: item_status, // Match order status
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create order item: {}", e)))?;

    order_item.map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to create order item: no record returned from database".to_string()))
}

pub async fn get_order_items(db: &Database, order_id: &str) -> AppResult<Vec<types::OrderItem>> {
    // Note: SurrealDB doesn't have native WHERE filtering on select() for simple fields
    // We keep the query approach for filtering, but standardize the error handling
    let query = "SELECT * FROM order_items WHERE order_id = $order_id";
    let mut response = db
        .query(query)
        .bind(("order_id", order_id.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order items: {}", e)))?;

    let order_items: Vec<OrderItemRecord> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse order items query result: {}", e)))?;

    Ok(order_items.into_iter().map(|record| record.into()).collect())
}

pub async fn get_all_order_items(db: &Database) -> AppResult<Vec<types::OrderItem>> {
    let order_items: Vec<OrderItemRecord> = db
        .select("order_items")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get all order items: {}", e)))?;

    Ok(order_items.into_iter().map(|record| record.into()).collect())
}

pub async fn get_order_item(db: &Database, id: &str) -> AppResult<Option<types::OrderItem>> {
    let order_item: Option<OrderItemRecord> = db
        .select(("order_items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order item: {}", e)))?;

    Ok(order_item.map(|record| record.into()))
}

pub async fn update_order_item(
    db: &Database,
    id: &str,
    request: types::UpdateOrderItemRequest,
) -> AppResult<types::OrderItem> {
    // First check if order item exists
    let existing: Option<OrderItemRecord> = db
        .select(("order_items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order item: {}", e)))?;

    let mut existing = existing
        .ok_or_else(|| AppError::NotFound(format!("Order item with id {} not found", id)))?;

    // Update fields if provided
    if let Some(item_id) = request.item_id {
        // Get new item price for price snapshotting
        // Use item ID directly since it's now a clean UUID
        let item: Option<ItemRecord> = db
            .select(("items", &item_id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

        let item = item.ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", item_id)))?;

        existing.item_id = item_id;
        existing.price = item.price; // Update price when item changes
    }

    if let Some(quantity) = request.quantity {
        validators::positive_quantity(quantity, "Quantity")?;
        existing.quantity = quantity;
    }

    if let Some(status) = request.status {
        existing.status = status;
    }

    existing.updated_at = Datetime::default();

    // Store order_id before moving existing
    let order_id = existing.order_id.clone();
    
    let updated: Option<OrderItemRecord> = db
        .update(("order_items", id))
        .content(existing)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update order item: {}", e)))?;

    let result = updated
        .map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to update order item: no record returned from database".to_string()))?;
    
    // Auto-update order status based on all OrderItems
    recalculate_order_status(db, &order_id).await?;
    
    Ok(result)
}

pub async fn delete_order_item(db: &Database, id: &str) -> AppResult<()> {
    let deleted: Option<OrderItemRecord> = db
        .delete(("order_items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete order item: {}", e)))?;

    if deleted.is_none() {
        return Err(AppError::NotFound(format!("Order item with id {} not found", id)));
    }

    Ok(())
}

async fn recalculate_order_status(db: &Database, order_id: &str) -> AppResult<()> {
    // Get all OrderItems for this order
    let query = "SELECT * FROM order_items WHERE order_id = $order_id";
    let mut response = db
        .query(query)
        .bind(("order_id", order_id.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order items for status calculation: {}", e)))?;

    let order_items: Vec<OrderItemRecord> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse order items query result: {}", e)))?;

    if order_items.is_empty() {
        return Ok(()); // No items, no status update needed
    }

    // Calculate new order status based on priority: Draft < Ordered < Ready < Completed
    let new_order_status = order_items.iter()
        .map(|item| &item.status)
        .max_by_key(|status| match status {
            types::OrderStatus::Draft => 0,
            types::OrderStatus::Ordered => 1,
            types::OrderStatus::Ready => 2,
            types::OrderStatus::Completed => 3,
            types::OrderStatus::Cancelled => 4,
        })
        .cloned()
        .unwrap_or(types::OrderStatus::Draft);

    // Update the order status without cascading to avoid infinite loop
    let order_update_request = types::UpdateOrderRequest {
        status: Some(new_order_status),
    };

    super::orders::update_order_without_cascade(db, order_id, order_update_request).await?;
    
    Ok(())
}

pub async fn bulk_update_order_items(db: &Database, update: types::BulkOrderItemUpdate) -> AppResult<Vec<types::OrderItem>> {
    if update.order_item_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut updated_items = Vec::new();
    let mut affected_orders = std::collections::HashSet::new();
    
    // Update each OrderItem individually (safer approach)
    for order_item_id in &update.order_item_ids {
        
        // Get the OrderItem first to track which orders are affected
        let existing: Option<OrderItemRecord> = db
            .select(("order_items", order_item_id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get order item: {}", e)))?;
            
        if let Some(existing_item) = existing {
            affected_orders.insert(existing_item.order_id.clone());
            
            // Update the item without auto-recalculation to batch it
            let mut updated_item = existing_item;
            updated_item.status = update.new_status;
            updated_item.updated_at = Datetime::default();
            
            let updated: Option<OrderItemRecord> = db
                .update(("order_items", order_item_id))
                .content(updated_item)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update order item: {}", e)))?;
                
            if let Some(record) = updated {
                updated_items.push(record.into());
            }
        }
    }
    
    // Recalculate order status for all affected orders
    for order_id in affected_orders {
        recalculate_order_status(db, &order_id).await?;
    }
    
    Ok(updated_items)
}

