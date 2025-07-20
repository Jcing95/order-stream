use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::ssr::*;

    pub const ORDERS: &str = "orders";

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
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateOrder, "/api/order")]
pub async fn create_order(req: requests::order::Create) -> Result<types::Order, ServerFnError> {
    DB.create(ORDERS)
        .content(Order {
            id: None,
            event: req.event,
            created_at: Datetime::default(),
        })
        .await?
        .ok_or_else(|| ServerError("Failed to create order".into()))
}

#[server(GetOrders, "/api/order")]
pub async fn get_orders() -> Result<Vec<types::Order>, ServerFnError> {
    let orders: Vec<Order> = DB.select(ORDERS).await?;
    Ok(orders.into_iter().map(Into::into).collect())
}

#[server(GetOrder, "/api/order")]
pub async fn get_order(id: String) -> Result<types::Order, ServerFnError> {
    DB.select((ORDERS, &id))
        .await?
        .ok_or_else(|| ServerError("Order not found".into()))
}

#[server(DeleteOrder, "/api/order")]
pub async fn delete_order(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Order> = DB.delete((ORDERS, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Order with id {} not found", id)));
    }
    Ok(())
}
