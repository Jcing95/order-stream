use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};
use crate::common::types::OrderStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRecord {
    pub id: Thing,
    pub sequential_id: u32,    // Auto-generated incremental ID
    pub event_id: String,      // Backend metadata for future multi-event support
    pub total_price: f64,
    pub status: OrderStatus,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<OrderRecord> for crate::common::types::Order {
    fn from(record: OrderRecord) -> Self {
        Self {
            id: record.id.to_string(),
            sequential_id: record.sequential_id,
            total_price: record.total_price,
            status: record.status,
        }
    }
}