use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecord {
    pub id: Thing,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<ItemRecord> for crate::common::types::Item {
    fn from(record: ItemRecord) -> Self {
        Self {
            id: record.id.to_string(),
            name: record.name,
            category: record.category,
            price: record.price,
            active: record.active,
        }
    }
}