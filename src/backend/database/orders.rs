use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};
use crate::backend::errors::{AppError, AppResult};
use crate::common::types;
use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRecord {
    pub id: Thing,
    pub sequential_id: u32,    // Auto-generated incremental ID
    pub event_id: String,      // Backend metadata for future multi-event support
    pub total_price: f64,
    pub status: types::OrderStatus,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<OrderRecord> for types::Order {
    fn from(record: OrderRecord) -> Self {
        Self {
            id: record.id.to_string(),
            sequential_id: record.sequential_id,
            total_price: record.total_price,
            status: record.status,
        }
    }
}

async fn get_next_sequential_id(db: &Database) -> AppResult<u32> {
    // Query for the maximum sequential_id
    let query = "SELECT VALUE sequential_id FROM orders ORDER BY sequential_id DESC LIMIT 1";
    let mut response = db
        .query(query)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get next sequential ID: {}", e)))?;

    let max_id: Option<u32> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse sequential ID query result: {}", e)))?;

    Ok(max_id.map(|id| id + 1).unwrap_or(1))
}

pub async fn create_order(db: &Database) -> AppResult<types::Order> {
    // Get next sequential ID
    let sequential_id = get_next_sequential_id(db).await?;

    #[derive(serde::Serialize)]
    struct CreateOrderData {
        sequential_id: u32,
        event_id: String,
        total_price: f64,
        status: types::OrderStatus,
        created_at: Datetime,
        updated_at: Datetime,
    }

    let order: Option<OrderRecord> = db
        .create("orders")
        .content(CreateOrderData {
            sequential_id,
            event_id: "default".to_string(), // Future multi-event support
            total_price: 0.0,
            status: types::OrderStatus::Draft,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create order: {}", e)))?;

    order.map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to create order: no record returned from database".to_string()))
}

pub async fn get_orders(db: &Database) -> AppResult<Vec<types::Order>> {
    let orders: Vec<OrderRecord> = db
        .select("orders")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get orders: {}", e)))?;

    Ok(orders.into_iter().map(|record| record.into()).collect())
}

pub async fn get_order(db: &Database, id: &str) -> AppResult<Option<types::Order>> {
    let order: Option<OrderRecord> = db
        .select(("orders", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order: {}", e)))?;

    Ok(order.map(|record| record.into()))
}

pub async fn update_order(
    db: &Database,
    id: &str,
    request: types::UpdateOrderRequest,
) -> AppResult<types::Order> {
    // First check if order exists
    let existing: Option<OrderRecord> = db
        .select(("orders", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get order: {}", e)))?;

    let mut existing = existing
        .ok_or_else(|| AppError::NotFound(format!("Order with id {} not found", id)))?;

    // Update fields if provided
    if let Some(status) = request.status {
        existing.status = status;
    }

    existing.updated_at = Datetime::default();

    let updated: Option<OrderRecord> = db
        .update(("orders", id))
        .content(existing)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update order: {}", e)))?;

    updated
        .map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to update order: no record returned from database".to_string()))
}

pub async fn delete_order(db: &Database, id: &str) -> AppResult<()> {
    let deleted: Option<OrderRecord> = db
        .delete(("orders", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete order: {}", e)))?;

    if deleted.is_none() {
        return Err(AppError::NotFound(format!("Order with id {} not found", id)));
    }

    Ok(())
}