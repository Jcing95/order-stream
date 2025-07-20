use crate::backend::db::product::get_product;
use crate::backend::error::Error;
use crate::common::{requests, types};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

use super::DB;
const ITEMS: &str = "items";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Item {
    pub id: Option<Thing>,
    pub order_id: String,
    pub product_id: String,
    #[validate(range(min = 1))]
    pub quantity: u32,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub status: types::OrderStatus,
}

impl From<Item> for types::Item {
    fn from(record: Item) -> Self {
        Self {
            id: record.id.unwrap().id.to_string(),
            order_id: record.order_id,
            product_id: record.product_id,
            quantity: record.quantity,
            price: record.price,
            status: record.status,
        }
    }
}

pub async fn create_item(req: requests::item::Create) -> Result<Item, Error> {
    let product = get_product(&req.product_id).await;
    match product {
        Ok(p) => DB
            .create(ITEMS)
            .content(Item {
                id: None,
                order_id: req.order_id,
                product_id: req.product_id,
                quantity: req.quantity,
                price: p.price,
                status: types::OrderStatus::Draft,
            })
            .await?
            .ok_or_else(|| Error::InternalError("Failed to create item".into())),
        Err(e) => Err(e),
    }
}

pub async fn get_items_by_order(order_id: &str) -> Result<Vec<types::Item>, Error> {
    let query = "SELECT * FROM items WHERE order_id = $order_id";
    let mut response = DB
        .query(query)
        .bind(("order_id", order_id.to_string()))
        .await?;

    let items: Vec<Item> = response.take(0)?;
    Ok(items.into_iter().map(Into::into).collect())
}

pub async fn get_items() -> Result<Vec<types::Item>, Error> {
    let items: Vec<Item> = DB.select(ITEMS).await?;
    Ok(items.into_iter().map(Into::into).collect())
}

pub async fn get_item(id: &str) -> Result<types::Item, Error> {
    DB.select((ITEMS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Item not found".into()))
}

pub async fn delete_item(id: &str) -> Result<(), Error> {
    let deleted: Option<Item> = DB.delete((ITEMS, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Item with id {} not found", id)));
    }
    Ok(())
}
