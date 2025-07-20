use super::Database;
use crate::backend::error::Error;
use crate::common::{requests, types};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

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

pub async fn create_product(
    db: &Database,
    req: requests::product::Create,
) -> Result<types::Product, Error> {
    let item: Option<Product> = db
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

pub async fn get_products(db: &Database) -> Result<Vec<types::Product>, Error> {
    let items: Vec<Product> = db
        .select(PRODUCTS)
        .await
        .map_err(|e| Error::InternalError("Failed to get items".into()))?;

    Ok(items.into_iter().map(|record| record.into()).collect())
}

pub async fn get_product(db: &Database, id: &str) -> Result<types::Product, Error> {
    db.select((PRODUCTS, id))
        .await?
        .ok_or_else(|| Error::NotFound("Product not found".into()))
}

pub async fn update_product(
    db: &Database,
    id: &str,
    update: requests::product::Update,
) -> Result<types::Product, Error> {
    db.update((PRODUCTS, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update product".into()))
}

pub async fn delete_product(db: &Database, id: &str) -> Result<(), Error> {
    let deleted: Option<Product> = db
        .delete((PRODUCTS, id))
        .await
        .map_err(|e| Error::InternalError(format!("Failed to delete product with id: {}", id)))?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!("Product with id {} not found", id)));
    }
    Ok(())
}
