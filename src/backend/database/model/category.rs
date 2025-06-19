use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRecord {
    pub id: Thing,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl From<CategoryRecord> for crate::common::types::Category {
    fn from(record: CategoryRecord) -> Self {
        Self {
            id: record.id.to_string(),
            name: record.name,
        }
    }
}