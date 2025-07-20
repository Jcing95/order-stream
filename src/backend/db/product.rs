use crate::common::{requests, types, errors::Error};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

use super::DB;
const PRODUCTS: &str = "products";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    pub id: Option<Thing>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub category_id: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub active: bool,
}

impl From<Product> for types::Product {
    fn from(record: Product) -> Self {
        Self {
            id: record.id.unwrap().id.to_string(),
            name: record.name,
            category_id: record.category_id,
            price: record.price,
            active: record.active,
        }
    }
}

pub async fn create_product(req: requests::product::Create) -> Result<types::Product, Error> {
    let item: Option<Product> = DB
        .create(PRODUCTS)
        .content(Product {
            id: None,
            name: req.name,
            category_id: req.category_id,
            price: req.price,
            active: req.active,
        })
        .await?;
    item.map(Into::into)
        .ok_or_else(|| Error::InternalError("Failed to create product".into()))
}

pub async fn get_products() -> Result<Vec<types::Product>, Error> {
    let items: Vec<Product> = DB.select(PRODUCTS).await?;
    Ok(items.into_iter().map(|record| record.into()).collect())
}

pub async fn get_product(id: &str) -> Result<types::Product, Error> {
    DB
        .select((PRODUCTS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Product not found".into()))
}

pub async fn update_product(
    id: &str,
    update: requests::product::Update,
) -> Result<types::Product, Error> {
    DB
        .update((PRODUCTS, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update product".into()))
}

pub async fn delete_product(id: &str) -> Result<(), Error> {
    let deleted: Option<Product> = DB
        .delete((PRODUCTS, id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to delete product with id: {}", id)))?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Product with id {} not found", id)));
    }
    Ok(())
}
