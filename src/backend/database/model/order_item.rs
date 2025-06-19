use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};

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

impl From<OrderItemRecord> for crate::common::types::OrderItem {
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