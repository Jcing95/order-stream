use crate::common::{requests, types};

use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::db::DB;
    pub use crate::backend::websocket::{broadcast_add, broadcast_delete, broadcast_update};
    pub use crate::common::types;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::Thing;
    use surrealdb::RecordId;
    pub use validator::Validate;
    pub const PRODUCTS: &str = "products";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Product {
        pub id: Option<RecordId>,
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
                id: record.id.unwrap().key().to_string(),
                name: record.name,
                category_id: record.category_id,
                price: record.price,
                active: record.active,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateProduct, "/api/product")]
pub async fn create_product(
    req: requests::product::Create,
) -> Result<types::Product, ServerFnError> {
    let p: Option<Product> = DB
        .create(PRODUCTS)
        .content(Product {
            id: None,
            name: req.name,
            category_id: req.category_id,
            price: req.price,
            active: true,
        })
        .await?;
    if let Some(product) = p {
        let result: types::Product = product.clone().into();
        broadcast_add(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to create product".into()))
    }
}

#[server(GetProducts, "/api/product")]
pub async fn get_products() -> Result<Vec<types::Product>, ServerFnError> {
    let products: Vec<Product> = DB.select(PRODUCTS).await?;
    Ok(products.into_iter().map(Into::into).collect())
}

#[server(GetProduct, "/api/product")]
pub async fn get_product(id: String) -> Result<types::Product, ServerFnError> {
    let product: Option<Product> = DB.select((PRODUCTS, &id)).await?;
    product
        .map(Into::into)
        .ok_or_else(|| ServerError("Product not found".into()))
}

#[server(UpdateProduct, "/api/product")]
pub async fn update_product(
    id: String,
    update: requests::product::Update,
) -> Result<types::Product, ServerFnError> {
    // Get the existing product
    let existing_product: Option<Product> = DB.select((PRODUCTS, &id)).await?;
    if existing_product.is_none() {
        return Err(ServerError("Product not found".into()));
    }
    let product = existing_product.unwrap();
    let updated = Product {
        id: product.id,
        name: update.name.or_else(|| Some(product.name)).unwrap(),
        category_id: update.category_id.or_else(|| Some(product.category_id)).unwrap(),
        price: update.price.or_else(|| Some(product.price)).unwrap(),
        active: update.active.or_else(|| Some(product.active)).unwrap(),
    };
    // Update the product in the database
    let updated_product: Option<Product> = DB
        .update((PRODUCTS, &id))
        .content(updated)
        .await?;

    if let Some(product) = updated_product {
        let result: types::Product = product.into();
        broadcast_update(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to update product".into()))
    }
}

#[server(DeleteProduct, "/api/product")]
pub async fn delete_product(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Product> = DB.delete((PRODUCTS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Product with id {} not found", id)));
    }
    broadcast_delete::<types::Product>(id);
    Ok(())
}
