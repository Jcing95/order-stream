use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Datetime};
use crate::backend::errors::{AppError, AppResult};
use crate::common::types;
use super::Database;

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

impl From<ItemRecord> for types::Item {
    fn from(record: ItemRecord) -> Self {
        Self {
            id: record.id.id.to_string(), // Extract just the UUID part
            name: record.name,
            category_id: record.category_id,
            price: record.price,
            active: record.active,
        }
    }
}

pub async fn create_item(db: &Database, request: types::CreateItemRequest) -> AppResult<types::Item> {
    #[derive(serde::Serialize)]
    struct CreateItemData {
        name: String,
        category_id: String,
        price: f64,
        active: bool,
        created_at: Datetime,
        updated_at: Datetime,
    }

    let item: Option<ItemRecord> = db
        .create("items")
        .content(CreateItemData {
            name: request.name,
            category_id: request.category_id,
            price: request.price,
            active: true,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        })
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create item: {}", e)))?;

    item.map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to create item: no record returned from database".to_string()))
}

pub async fn get_items(db: &Database) -> AppResult<Vec<types::Item>> {
    let items: Vec<ItemRecord> = db
        .select("items")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get items: {}", e)))?;

    Ok(items.into_iter().map(|record| record.into()).collect())
}

pub async fn get_item(db: &Database, id: &str) -> AppResult<Option<types::Item>> {
    let item: Option<ItemRecord> = db
        .select(("items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

    Ok(item.map(|record| record.into()))
}

pub async fn update_item(
    db: &Database,
    id: &str,
    request: types::UpdateItemRequest,
) -> AppResult<types::Item> {
    // First check if item exists
    let existing: Option<ItemRecord> = db
        .select(("items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get item: {}", e)))?;

    let mut existing = existing
        .ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", id)))?;

    // Update fields if provided
    if let Some(name) = request.name {
        if name.trim().is_empty() {
            return Err(AppError::ValidationError("Name cannot be empty".to_string()));
        }
        existing.name = name;
    }
    if let Some(category_id) = request.category_id {
        if category_id.trim().is_empty() {
            return Err(AppError::ValidationError("Category ID cannot be empty".to_string()));
        }
        existing.category_id = category_id;
    }
    if let Some(price) = request.price {
        if price < 0.0 {
            return Err(AppError::ValidationError("Price cannot be negative".to_string()));
        }
        existing.price = price;
    }
    if let Some(active) = request.active {
        existing.active = active;
    }

    existing.updated_at = Datetime::default();

    let updated: Option<ItemRecord> = db
        .update(("items", id))
        .content(existing)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update item: {}", e)))?;

    updated
        .map(|record| record.into())
        .ok_or_else(|| AppError::InternalError("Failed to update item: no record returned from database".to_string()))
}

pub async fn delete_item(db: &Database, id: &str) -> AppResult<()> {
    let deleted: Option<ItemRecord> = db
        .delete(("items", id))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete item: {}", e)))?;

    if deleted.is_none() {
        return Err(AppError::NotFound(format!("Item with id {} not found", id)));
    }

    Ok(())
}