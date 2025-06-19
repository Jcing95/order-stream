use crate::common::errors::{AppError, AppResult};
use crate::common::types;
use crate::backend::database::model::category::CategoryRecord;
use crate::backend::database::Database;
use surrealdb::sql::Datetime;

pub struct Dao;
impl Dao {
    pub async fn create_category(db: &Database, request: types::CreateCategoryRequest) -> AppResult<types::Category> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e))?;

        #[derive(serde::Serialize)]
        struct CreateCategoryData {
            name: String,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let category: Option<CategoryRecord> = db
            .create("categories")
            .content(CreateCategoryData {
                name: request.name,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create category: {}", e)))?;

        category.map(|record| record.into())
            .ok_or_else(|| AppError::InternalError("Failed to create category: no record returned from database".to_string()))
    }

    pub async fn get_categories(db: &Database) -> AppResult<Vec<types::Category>> {
        let categories: Vec<CategoryRecord> = db
            .select("categories")
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get categories: {}", e)))?;

        Ok(categories.into_iter().map(|record| record.into()).collect())
    }

    pub async fn get_category(db: &Database, id: &str) -> AppResult<Option<types::Category>> {
        let category: Option<CategoryRecord> = db
            .select(("categories", id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get category: {}", e)))?;

        Ok(category.map(|record| record.into()))
    }

    pub async fn update_category(
        db: &Database,
        id: &str,
        request: types::UpdateCategoryRequest,
    ) -> AppResult<types::Category> {
        // First check if category exists
        let existing: Option<CategoryRecord> = db
            .select(("categories", id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get category: {}", e)))?;

        let mut existing =
            existing.ok_or_else(|| AppError::NotFound(format!("Category with id {} not found", id)))?;

        // Update fields if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Name cannot be empty".to_string(),
                ));
            }
            existing.name = name;
        }

        existing.updated_at = Datetime::default();

        let updated: Option<CategoryRecord> = db
            .update(("categories", id))
            .content(existing)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update category: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| AppError::InternalError("Failed to update category: no record returned from database".to_string()))
    }

    pub async fn delete_category(db: &Database, id: &str) -> AppResult<()> {
        let deleted: Option<CategoryRecord> = db
            .delete(("categories", id))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete category: {}", e)))?;

        if deleted.is_none() {
            return Err(AppError::NotFound(format!("Category with id {} not found", id)));
        }

        Ok(())
    }
}