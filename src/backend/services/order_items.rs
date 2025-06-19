use crate::common::errors::AppResult;
use crate::common::types::{CreateOrderItemRequest, OrderItem, UpdateOrderItemRequest};
use crate::backend::database::Database;
use crate::backend::database::dao::order_item;

pub struct Service;

impl Service {
    pub async fn create_order_item(db: &Database, request: CreateOrderItemRequest) -> AppResult<OrderItem> {
        order_item::Dao::create_order_item(db, request).await
    }

    pub async fn get_order_items(db: &Database, order_id: &str) -> AppResult<Vec<OrderItem>> {
        order_item::Dao::get_order_items(db, order_id).await
    }

    pub async fn get_order_item(db: &Database, id: &str) -> AppResult<Option<OrderItem>> {
        order_item::Dao::get_order_item(db, id).await
    }

    pub async fn update_order_item(db: &Database, id: &str, request: UpdateOrderItemRequest) -> AppResult<OrderItem> {
        order_item::Dao::update_order_item(db, id, request).await
    }

    pub async fn delete_order_item(db: &Database, id: &str) -> AppResult<()> {
        order_item::Dao::delete_order_item(db, id).await
    }
}