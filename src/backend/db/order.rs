use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};
use crate::backend::error::Error;
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
            id: record.id.id.to_string(), // Extract just the UUID part
            sequential_id: record.sequential_id,
            total_price: record.total_price,
            status: record.status,
        }
    }
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
        .map_err(|e| Error::InternalError(format!("Failed to create order: {}", e)))?;

    order.map(|record| record.into())
        .ok_or_else(|| Error::InternalError("Failed to create order: no record returned from database".to_string()))
}

pub async fn get_orders(db: &Database) -> AppResult<Vec<types::Order>> {
    let orders: Vec<OrderRecord> = db
        .select("orders")
        .await
        .map_err(|e| Error::InternalError(format!("Failed to get orders: {}", e)))?;

    Ok(orders.into_iter().map(|record| record.into()).collect())
}

pub async fn get_order(db: &Database, id: &str) -> AppResult<Option<types::Order>> {
    let order: Option<OrderRecord> = db
        .select(("orders", id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to get order: {}", e)))?;

    Ok(order.map(|record| record.into()))
}

pub async fn update_order(
    db: &Database,
    id: &str,
    request: types::UpdateOrderRequest,
) -> AppResult<types::Order> {
    update_order_internal(db, id, request, true).await
}

pub async fn update_order_without_cascade(
    db: &Database,
    id: &str,
    request: types::UpdateOrderRequest,
) -> AppResult<types::Order> {
    update_order_internal(db, id, request, false).await
}

async fn update_order_internal(
    db: &Database,
    id: &str,
    request: types::UpdateOrderRequest,
    cascade_to_items: bool,
) -> AppResult<types::Order> {
    // First check if order exists
    let existing: Option<OrderRecord> = db
        .select(("orders", id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to get order: {}", e)))?;

    let mut existing = existing
        .ok_or_else(|| Error::NotFound(format!("Order with id {} not found", id)))?;

    // Update fields if provided
    if let Some(status) = request.status {
        let old_status = existing.status;
        existing.status = status;
        
        // When moving to ordered status, update all order items to ordered as well
        // But only if this isn't being called from recalculation to avoid infinite loop
        if cascade_to_items && status == types::OrderStatus::Ordered && old_status != types::OrderStatus::Ordered {
            // The order_id in order_items is stored as just the UUID, not "orders:uuid"
            let update_query = "UPDATE order_items SET status = $new_status, updated_at = time::now() WHERE order_id = $order_id";
            
            let result = db.query(update_query)
                .bind(("new_status", types::OrderStatus::Ordered))
                .bind(("order_id", id.to_string()))
                .await
                .map_err(|e| Error::InternalError(format!("Failed to update order items status: {}", e)))?;
                
            // Log how many items were updated for debugging
            leptos::logging::log!("Updated order items for order {}: {:?}", id, result);
        }
    }

    existing.updated_at = Datetime::default();

    let updated: Option<OrderRecord> = db
        .update(("orders", id))
        .content(existing)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update order: {}", e)))?;

    updated
        .map(|record| record.into())
        .ok_or_else(|| Error::InternalError("Failed to update order: no record returned from database".to_string()))
}

pub async fn delete_order(db: &Database, id: &str) -> AppResult<()> {
    let deleted: Option<OrderRecord> = db
        .delete(("orders", id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to delete order: {}", e)))?;

    if deleted.is_none() {
        return Err(Error::NotFound(format!("Order with id {} not found", id)));
    }

    Ok(())
}