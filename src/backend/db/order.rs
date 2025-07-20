use crate::backend::error::Error;
use crate::common::{requests, types};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use validator::Validate;

use super::DB;
const ORDERS: &str = "orders";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Order {
    pub id: Option<Thing>,
    #[validate(length(min = 1))]
    pub event: String,
    pub created_at: Datetime,
}

impl From<Order> for types::Order {
    fn from(record: Order) -> Self {
        Self {
            id: record.id.unwrap().id.to_string(),
        }
    }
}

pub async fn create_order(req: requests::order::Create) -> Result<types::Order, Error> {
    DB
        .create(ORDERS)
        .content(Order {
            id: None,
            event: req.event,
            created_at: Datetime::default(),
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create order".into()))
}

pub async fn get_orders() -> Result<Vec<types::Order>, Error> {
    let orders: Vec<Order> = DB.select(ORDERS).await?;
    Ok(orders.into_iter().map(|record| record.into()).collect())
}

pub async fn get_order(id: &str) -> Result<types::Order, Error> {
    DB
        .select((ORDERS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Order not found".into()))
}

pub async fn delete_order(id: &str) -> Result<(), Error> {
    let deleted: Option<Order> = DB.delete((ORDERS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Order with id {} not found", id)));
    }
    Ok(())
}
