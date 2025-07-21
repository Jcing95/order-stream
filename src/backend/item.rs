use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::db::DB;
    pub use crate::common::types;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::Thing;
    use surrealdb::RecordId;
    pub use validator::Validate;
    pub const ITEMS: &str = "items";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Item {
        pub id: Option<RecordId>,
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
                id: record.id.unwrap().to_string(),
                order_id: record.order_id,
                product_id: record.product_id,
                quantity: record.quantity,
                price: record.price,
                status: record.status,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateItem, "/api/item")]
pub async fn create_item(req: requests::item::Create) -> Result<types::Item, ServerFnError> {
    use crate::backend::product::get_product;

    let product = get_product(req.product_id.clone()).await?;
    let i: Option<Item> = DB.create(ITEMS)
        .content(Item {
            id: None,
            order_id: req.order_id,
            product_id: req.product_id,
            quantity: req.quantity,
            price: product.price,
            status: types::OrderStatus::Draft,
        })
        .await?;
    i.map(Into::into).ok_or_else(|| ServerError("Failed to create item".into()))
}

#[server(GetItemsByOrder, "/api/item")]
pub async fn get_items_by_order(order_id: String) -> Result<Vec<types::Item>, ServerFnError> {
    let query = "SELECT * FROM items WHERE order_id = $order_id";
    let mut response = DB.query(query).bind(("order_id", order_id)).await?;

    let items: Vec<Item> = response.take(0)?;
    Ok(items.into_iter().map(Into::into).collect())
}

#[server(GetItems, "/api/item")]
pub async fn get_items() -> Result<Vec<types::Item>, ServerFnError> {
    let items: Vec<Item> = DB.select(ITEMS).await?;
    Ok(items.into_iter().map(Into::into).collect())
}

#[server(GetItem, "/api/item")]
pub async fn get_item(id: String) -> Result<types::Item, ServerFnError> {
    DB.select((ITEMS, &id))
        .await?
        .ok_or_else(|| ServerError("Item not found".into()))
}

#[server(DeleteItem, "/api/item")]
pub async fn delete_item(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Item> = DB.delete((ITEMS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Item with id {} not found", id)));
    }
    Ok(())
}
