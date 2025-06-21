use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};
use crate::backend::errors::{AppError, AppResult};
use crate::common::types;
use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRecord {
    pub id: Thing,
    pub order_id: String,     // FK to OrderRecord
    pub item_id: String,      // FK to ItemRecord  
    pub quantity: u32,
    pub price: f64,          // Snapshot of item price at order time
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<OrderItemRecord> for types::OrderItem {
    fn from(record: OrderItemRecord) -> Self {
        Self {
            id: record.id.to_string(),
            order_id: record.order_id,
            item_id: record.item_id,
            quantity: record.quantity,
            price: record.price,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecord {
    pub id: Thing,
    pub name: String,
    pub category_id: String,
    pub price: f64,
    pub active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

pub async fn create_order_item(db: &Database, request: types::CreateOrderItemRequest) -> AppResult<types::OrderItem> {
    // Get current item price for price snapshotting
    let item: Option<ItemRecord> = db
        .select(("items", &request.item_id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

    let item = item.ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", request.item_id)))?;

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
        .map_err(|e| AppError::DatabaseError(format!("Failed to create order item: {}", e)))?;

    order_item.map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to create order item: no record returned from database".to_string()))
}

pub async fn get_order_items(db: &Database, order_id: &str) -> AppResult<Vec<types::OrderItem>> {
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
        let item: Option<ItemRecord> = db
            .select(("items", &item_id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

        let item = item.ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", item_id)))?;

        existing.item_id = item_id;
        existing.price = item.price; // Update price when item changes
    }

    if let Some(quantity) = request.quantity {
        if quantity == 0 {
            return Err(AppError::ValidationError("Quantity must be greater than 0".to_string()));
        }
        existing.quantity = quantity;
    }

    existing.updated_at = Datetime::default();

    let updated: Option<OrderItemRecord> = db
        .update(("order_items", id))
        .content(existing)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update order item: {}", e)))?;

    updated
        .map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to update order item: no record returned from database".to_string()))
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