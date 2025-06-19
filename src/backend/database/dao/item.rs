use crate::common::errors::{AppError, AppResult};
use crate::common::types;
use crate::backend::database::model::item::ItemRecord;
use crate::backend::database::Database;
use chrono::Utc;
use uuid::Uuid;

pub struct Dao;
impl Dao {
    pub async fn create_item(db: &Database, request: types::CreateItemRequest) -> AppResult<types::Item> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

        let id = format!("item:{}", Uuid::new_v4());
        let now = Utc::now();

        let item: Option<ItemRecord> = db
            .create(("items", id.clone()))
            .content(ItemRecord {
                id: id.parse().unwrap(),
                name: request.name,
                category: request.category,
                price: request.price,
                active: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create item: {}", e)))?;

        item.map(|record| record.into())
            .ok_or_else(|| AppError::InternalError("Failed to create item".to_string()))
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

        let mut existing =
            existing.ok_or_else(|| AppError::NotFound(format!("Item with id {} not found", id)))?;

        // Update fields if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Name cannot be empty".to_string(),
                ));
            }
            existing.name = name;
        }
        if let Some(category) = request.category {
            if category.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Category cannot be empty".to_string(),
                ));
            }
            existing.category = category;
        }
        if let Some(price) = request.price {
            if price < 0.0 {
                return Err(AppError::ValidationError(
                    "Price cannot be negative".to_string(),
                ));
            }
            existing.price = price;
        }
        if let Some(active) = request.active {
            existing.active = active;
        }

        existing.updated_at = Utc::now();

        let updated: Option<ItemRecord> = db
            .update(("items", id))
            .content(existing)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update item: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| AppError::InternalError("Failed to update item".to_string()))
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
}
