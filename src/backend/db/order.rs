use super::Database;
use crate::backend::error::Error;
use crate::common::{requests, types};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use validator::Validate;

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

pub async fn create_order(db: &Database, req: requests::order::Create) -> Result<types::Order, Error> {
    db.create(ORDERS).content(Order {
        id: None,
        event: req.event,
        created_at: Datetime::default(),
    }).await?.ok_or_else(|| Error::InternalError("Failed to create order".into()))
}

pub async fn get_orders(db: &Database) -> Result<Vec<types::Order>, Error> {
    let orders: Vec<Order> = db.select(ORDERS).await?;

    Ok(orders.into_iter().map(|record| record.into()).collect())
}

pub async fn get_order(db: &Database, id: &str) -> Result<Option<types::Order>, Error> {
    let order: Option<Order> = db.select((ORDERS, id)).await?;

    Ok(order.map(|record| record.into()))
}


pub async fn delete_order(db: &Database, id: &str) -> Result<(), Error> {
    let deleted: Option<Order> = db.delete((ORDERS, id)).await?;

    if deleted.is_none() {
        return Err(Error::NotFound(format!("Order with id {} not found", id)));
    }

    Ok(())
}
