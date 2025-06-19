use crate::common::errors::AppResult;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};
use crate::backend::database::service::connection::Database;
use crate::backend::database::service::item::ItemService;

pub struct ItemBusinessService;

impl ItemBusinessService {
    pub async fn create_item(db: &Database, request: CreateItemRequest) -> AppResult<Item> {
        ItemService::create_item(db, request).await
    }

    pub async fn get_items(db: &Database) -> AppResult<Vec<Item>> {
        ItemService::get_items(db).await
    }

    pub async fn get_item(db: &Database, id: &str) -> AppResult<Option<Item>> {
        ItemService::get_item(db, id).await
    }

    pub async fn update_item(db: &Database, id: &str, request: UpdateItemRequest) -> AppResult<Item> {
        ItemService::update_item(db, id, request).await
    }

    pub async fn delete_item(db: &Database, id: &str) -> AppResult<()> {
        ItemService::delete_item(db, id).await
    }
}