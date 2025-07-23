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
                id: record.id.unwrap().key().to_string(),
                order_id: Some(record.order_id),
                product_id: record.product_id,
                quantity: record.quantity,
                price: record.price,
                status: record.status,
            }
        }
    }

    pub async fn create_items(order_id: String, items: Vec<types::Item>) -> Result<Vec<types::Item>, leptos::prelude::ServerFnError> {
        use crate::backend::product::get_product;
        let mut created_items = Vec::new();
        
        for item in items {
            // Get the product to fetch the current price
            let product = get_product(item.product_id.clone()).await?;
            
            let db_item: Option<Item> = DB.create(ITEMS)
                .content(Item {
                    id: None,
                    order_id: order_id.clone(),
                    product_id: item.product_id,
                    quantity: item.quantity,
                    price: product.price, // Use current product price
                    status: types::OrderStatus::Draft,
                })
                .await?;
                
            if let Some(created) = db_item {
                created_items.push(created.into());
            } else {
                return Err(ServerError("Failed to create item".into()));
            }
        }
        
        Ok(created_items)
    }
}
#[cfg(feature = "ssr")]
use ssr::*;


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

#[server(UpdateItem, "/api/item")]
pub async fn update_item(
    id: String,
    update: requests::item::Update,
) -> Result<types::Item, ServerFnError> {
    // Get the existing item
    let existing_item: Option<Item> = DB.select((ITEMS, &id)).await?;
    if existing_item.is_none() {
        return Err(ServerError("Item not found".into()));
    }
    let item = existing_item.unwrap();
    
    // If product_id is being changed, get the new price
    let new_price = if let Some(ref new_product_id) = update.product_id {
        if new_product_id != &item.product_id {
            use crate::backend::product::get_product;
            let product = get_product(new_product_id.clone()).await?;
            product.price
        } else {
            item.price
        }
    } else {
        item.price
    };
    
    let updated = Item {
        id: item.id,
        order_id: item.order_id,
        product_id: update.product_id.or_else(|| Some(item.product_id)).unwrap(),
        quantity: update.quantity.or_else(|| Some(item.quantity)).unwrap(),
        price: new_price,
        status: update.status.or_else(|| Some(item.status)).unwrap(),
    };
    // Update the item in the database
    let updated_item: Option<Item> = DB
        .update((ITEMS, &id))
        .content(updated)
        .await?;
        
    updated_item
        .map(Into::into)
        .ok_or_else(|| ServerError("Failed to update item".into()))
}

#[server(DeleteItem, "/api/item")]
pub async fn delete_item(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Item> = DB.delete((ITEMS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Item with id {} not found", id)));
    }
    Ok(())
}
