use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::db::DB;
    pub use crate::common::types;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::{Datetime, Thing};
    use surrealdb::RecordId;
    pub use validator::Validate;

    pub const ORDERS: &str = "orders";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Order {
        pub id: Option<RecordId>,
        #[validate(length(min = 1))]
        pub event: String,
        pub created_at: Datetime,
    }

    impl From<Order> for types::Order {
        fn from(record: Order) -> Self {
            Self {
                id: record.id.unwrap().key().to_string(),
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateOrder, "/api/order")]
pub async fn create_order(req: requests::order::Create) -> Result<types::Order, ServerFnError> {
    use crate::backend::item::ssr::create_items;
    use crate::backend::websocket::broadcast_add;
    
    // First create the order
    let order: Option<Order> = DB.create(ORDERS)
        .content(Order {
            id: None,
            event: req.event,
            created_at: Datetime::default(),
        })
        .await?;
    
    if order.is_none() {
        return Err(ServerError("Failed to create order".to_string().into()));
    }
    let order = order.unwrap();
    let order_id = order.id.as_ref().unwrap().key().to_string();
    
    // Convert to types::Order for broadcasting
    let order_type: types::Order = order.clone().into();
    
    // Broadcast the new order
    broadcast_add(order_type.clone());
    
    // Then create all the items
    if !req.items.is_empty() {
        create_items(order_id, req.items).await?;
    }
    
    Ok(order_type)
}

#[server(GetOrders, "/api/order")]
pub async fn get_orders() -> Result<Vec<types::Order>, ServerFnError> {
    let orders: Vec<Order> = DB.select(ORDERS).await?;
    Ok(orders.into_iter().map(Into::into).collect())
}

#[server(GetOrder, "/api/order")]
pub async fn get_order(id: String) -> Result<types::Order, ServerFnError> {
    let order: Option<Order> = DB.select((ORDERS, &id)).await?;
    order
        .map(Into::into)
        .ok_or_else(|| ServerError("Order not found".into()))
}

#[server(DeleteOrder, "/api/order")]
pub async fn delete_order(id: String) -> Result<(), ServerFnError> {
    use crate::backend::websocket::broadcast_delete;
    
    let deleted: Option<Order> = DB.delete((ORDERS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Order with id {} not found", id)));
    }
    
    // Broadcast the order deletion
    broadcast_delete::<types::Order>(id);
    
    Ok(())
}
