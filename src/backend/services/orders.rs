use crate::common::errors::AppResult;
use crate::common::types::{Order, UpdateOrderRequest};
use crate::backend::database::Database;
use crate::backend::database::dao::order;

pub struct Service;

impl Service {
    pub async fn create_order(db: &Database) -> AppResult<Order> {
        order::Dao::create_order(db).await
    }

    pub async fn get_orders(db: &Database) -> AppResult<Vec<Order>> {
        order::Dao::get_orders(db).await
    }

    pub async fn get_order(db: &Database, id: &str) -> AppResult<Option<Order>> {
        order::Dao::get_order(db, id).await
    }

    pub async fn update_order(db: &Database, id: &str, request: UpdateOrderRequest) -> AppResult<Order> {
        order::Dao::update_order(db, id, request).await
    }

    pub async fn delete_order(db: &Database, id: &str) -> AppResult<()> {
        order::Dao::delete_order(db, id).await
    }
}