use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateOrder, "/api/order")]
pub async fn create_order(req: requests::order::Create) -> Result<types::Order, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbOrder, NewOrder};
    use crate::backend::websocket::broadcast_add;
    use crate::backend::schema::orders::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let eid: i32 = req.event.parse()
        .map_err(|_| Error::InternalError("Invalid event ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let order: DbOrder = diesel::insert_into(orders)
        .values(NewOrder { event_id: eid })
        .returning(DbOrder::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create order: {}", e)))?;

    let order_type: types::Order = order.into();
    broadcast_add(order_type.clone());

    if !req.items.is_empty() {
        use crate::backend::item::create_items;
        create_items(order_type.id.clone(), req.items).await?;
    }

    Ok(order_type)
}

#[server(GetOrders, "/api/order")]
pub async fn get_orders() -> Result<Vec<types::Order>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbOrder;
    use crate::backend::schema::orders::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbOrder> = orders
        .select(DbOrder::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetOrder, "/api/order")]
pub async fn get_order(order_id: String) -> Result<types::Order, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbOrder;
    use crate::backend::schema::orders::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let oid: i32 = order_id.parse()
        .map_err(|_| Error::InternalError("Invalid order ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let order: Option<DbOrder> = orders
        .filter(id.eq(oid))
        .select(DbOrder::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    order.map(Into::into)
        .ok_or_else(|| Error::InternalError("Order not found".to_string()).into())
}

#[server(DeleteOrder, "/api/order")]
pub async fn delete_order(order_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::orders::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let oid: i32 = order_id.parse()
        .map_err(|_| Error::InternalError("Invalid order ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(orders.filter(id.eq(oid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Order {} not found", order_id)).into());
    }
    broadcast_delete::<types::Order>(order_id);
    Ok(())
}
