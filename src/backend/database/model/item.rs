use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecord {
    pub id: Thing,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ItemRecord> for crate::common::types::Item {
    fn from(record: ItemRecord) -> Self {
        Self {
            id: record.id.to_string(),
            name: record.name,
            category: record.category,
            price: record.price,
            active: record.active,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}