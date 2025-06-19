use crate::common::errors::AppResult;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};
use crate::backend::database::Database;
use crate::backend::database::dao::item;

pub struct Service;

impl Service {
    pub async fn create_item(db: &Database, request: CreateItemRequest) -> AppResult<Item> {
        item::Dao::create_item(db, request).await
    }

    pub async fn get_items(db: &Database) -> AppResult<Vec<Item>> {
        item::Dao::get_items(db).await
    }

    pub async fn get_item(db: &Database, id: &str) -> AppResult<Option<Item>> {
        item::Dao::get_item(db, id).await
    }

    pub async fn update_item(db: &Database, id: &str, request: UpdateItemRequest) -> AppResult<Item> {
        item::Dao::update_item(db, id, request).await
    }

    pub async fn delete_item(db: &Database, id: &str) -> AppResult<()> {
        item::Dao::delete_item(db, id).await
    }
}