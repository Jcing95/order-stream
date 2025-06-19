use crate::common::errors::AppResult;
use crate::common::types::{CreateCategoryRequest, Category, UpdateCategoryRequest};
use crate::backend::database::Database;
use crate::backend::database::dao::category;

pub struct Service;

impl Service {
    pub async fn create_category(db: &Database, request: CreateCategoryRequest) -> AppResult<Category> {
        category::Dao::create_category(db, request).await
    }

    pub async fn get_categories(db: &Database) -> AppResult<Vec<Category>> {
        category::Dao::get_categories(db).await
    }

    pub async fn get_category(db: &Database, id: &str) -> AppResult<Option<Category>> {
        category::Dao::get_category(db, id).await
    }

    pub async fn update_category(db: &Database, id: &str, request: UpdateCategoryRequest) -> AppResult<Category> {
        category::Dao::update_category(db, id, request).await
    }

    pub async fn delete_category(db: &Database, id: &str) -> AppResult<()> {
        category::Dao::delete_category(db, id).await
    }
}