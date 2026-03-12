use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[cfg(feature = "ssr")]
pub async fn create_items(
    order_id_str: String,
    item_list: Vec<types::Item>,
) -> Result<Vec<types::Item>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbItem, NewItem};
    use crate::backend::schema::items::dsl::items as items_table;
    use crate::backend::websocket::broadcast_add;
    use crate::backend::models::order_status_str;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let oid: i32 = order_id_str.parse()
        .map_err(|_| Error::InternalError("Invalid order ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let mut created_items = Vec::new();
    for item in item_list {
        let pid: i32 = item.product_id.parse()
            .map_err(|_| Error::InternalError("Invalid product ID".to_string()))?;

        let product = crate::backend::product::get_product(item.product_id.clone()).await?;

        let new_item = NewItem {
            order_id: oid,
            product_id: pid,
            quantity: item.quantity as i32,
            price: product.price,
            status: order_status_str(types::OrderStatus::Ordered).to_string(),
        };

        let db_item: DbItem = diesel::insert_into(items_table)
            .values(&new_item)
            .returning(DbItem::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| Error::InternalError(format!("Failed to create item: {}", e)))?;

        let item_type: types::Item = db_item.into();
        broadcast_add(item_type.clone());
        created_items.push(item_type);
    }
    Ok(created_items)
}

#[server(GetItemsByOrder, "/api/item")]
pub async fn get_items_by_order(order_id: String) -> Result<Vec<types::Item>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbItem;
    use crate::backend::schema::items::dsl::{self, items};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let oid: i32 = order_id.parse()
        .map_err(|_| Error::InternalError("Invalid order ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbItem> = items
        .filter(dsl::order_id.eq(oid))
        .select(DbItem::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetItems, "/api/item")]
pub async fn get_items() -> Result<Vec<types::Item>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbItem;
    use crate::backend::schema::items::dsl::items;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbItem> = items
        .select(DbItem::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetItem, "/api/item")]
pub async fn get_item(item_id: String) -> Result<types::Item, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbItem;
    use crate::backend::schema::items::dsl::{self, items};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let iid: i32 = item_id.parse()
        .map_err(|_| Error::InternalError("Invalid item ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let item: Option<DbItem> = items
        .filter(dsl::id.eq(iid))
        .select(DbItem::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    item.map(Into::into)
        .ok_or_else(|| Error::InternalError("Item not found".to_string()).into())
}

#[server(UpdateItem, "/api/item")]
pub async fn update_item(
    item_id: String,
    update: requests::item::Update,
) -> Result<types::Item, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbItem, UpdateItem, order_status_str};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::items::dsl::{self, items};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let iid: i32 = item_id.parse()
        .map_err(|_| Error::InternalError("Invalid item ID".to_string()))?;

    let pid: Option<i32> = update.product_id
        .as_ref()
        .map(|s| s.parse().map_err(|_| Error::InternalError("Invalid product ID".to_string())))
        .transpose()?;

    // If product is changing, look up the new price
    let new_price = if let Some(ref new_pid) = update.product_id {
        let product = crate::backend::product::get_product(new_pid.clone()).await?;
        Some(product.price)
    } else {
        None
    };

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let changes = UpdateItem {
        product_id: pid,
        quantity: update.quantity.map(|q| q as i32),
        price: new_price,
        status: update.status.map(|s| order_status_str(s).to_string()),
    };

    let item: DbItem = diesel::update(items.filter(dsl::id.eq(iid)))
        .set(&changes)
        .returning(DbItem::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update item: {}", e)))?;

    let item_type: types::Item = item.into();
    broadcast_update(item_type.clone());
    Ok(item_type)
}

#[server(DeleteItem, "/api/item")]
pub async fn delete_item(item_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::items::dsl::{self, items};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let iid: i32 = item_id.parse()
        .map_err(|_| Error::InternalError("Invalid item ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(items.filter(dsl::id.eq(iid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Item {} not found", item_id)).into());
    }
    broadcast_delete::<types::Item>(item_id);
    Ok(())
}

#[server(GetItemsByStation, "/api/item")]
pub async fn get_items_by_station(station_id: String) -> Result<Vec<types::Item>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbItem, order_status_str};
    use crate::backend::schema::items::dsl::{self, items};
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let sid: i32 = station_id.parse()
        .map_err(|_| Error::InternalError("Invalid station ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    // Load station's accepted input statuses
    let input_status_strs: Vec<String> = sis_dsl::station_input_statuses
        .filter(sis_dsl::station_id.eq(sid))
        .select(sis_dsl::status)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    // Load category IDs handled by this station
    let cat_ids: Vec<i32> = sc_dsl::station_categories
        .filter(sc_dsl::station_id.eq(sid))
        .select(sc_dsl::category_id)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    // Load matching items (correct status), then filter by category
    let candidate_items: Vec<DbItem> = items
        .filter(dsl::status.eq_any(&input_status_strs))
        .select(DbItem::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    // Filter by product category using a join query
    use crate::backend::schema::products::dsl as p_dsl;
    let mut filtered = Vec::new();
    for item in candidate_items {
        let product_cat: Option<i32> = p_dsl::products
            .filter(p_dsl::id.eq(item.product_id))
            .select(p_dsl::category_id)
            .first(&mut conn)
            .await
            .optional()
            .map_err(|e| Error::InternalError(e.to_string()))?;

        if let Some(cat_id) = product_cat {
            if cat_ids.contains(&cat_id) {
                filtered.push(item);
            }
        }
    }

    Ok(filtered.into_iter().map(Into::into).collect())
}

#[server(UpdateItemsByOrder, "/api/item")]
pub async fn update_items_by_order(
    order_id: String,
    station_id: String,
    new_status: types::OrderStatus,
) -> Result<Vec<types::Item>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbItem, order_status_str};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::items::dsl::{self, items};
    use crate::backend::schema::station_categories::dsl as sc_dsl;
    use crate::backend::schema::station_input_statuses::dsl as sis_dsl;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let oid: i32 = order_id.parse()
        .map_err(|_| Error::InternalError("Invalid order ID".to_string()))?;
    let sid: i32 = station_id.parse()
        .map_err(|_| Error::InternalError("Invalid station ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let input_status_strs: Vec<String> = sis_dsl::station_input_statuses
        .filter(sis_dsl::station_id.eq(sid))
        .select(sis_dsl::status)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let cat_ids: Vec<i32> = sc_dsl::station_categories
        .filter(sc_dsl::station_id.eq(sid))
        .select(sc_dsl::category_id)
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    let order_items: Vec<DbItem> = items
        .filter(dsl::order_id.eq(oid))
        .filter(dsl::status.eq_any(&input_status_strs))
        .select(DbItem::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    use crate::backend::schema::products::dsl as p_dsl;
    let new_status_str = order_status_str(new_status).to_string();
    let mut updated_items = Vec::new();

    for item in order_items {
        let product_cat: Option<i32> = p_dsl::products
            .filter(p_dsl::id.eq(item.product_id))
            .select(p_dsl::category_id)
            .first(&mut conn)
            .await
            .optional()
            .map_err(|e| Error::InternalError(e.to_string()))?;

        if let Some(cat_id) = product_cat {
            if cat_ids.contains(&cat_id) {
                let updated: DbItem = diesel::update(items.filter(dsl::id.eq(item.id)))
                    .set(dsl::status.eq(&new_status_str))
                    .returning(DbItem::as_returning())
                    .get_result(&mut conn)
                    .await
                    .map_err(|e| Error::InternalError(e.to_string()))?;

                let item_type: types::Item = updated.into();
                broadcast_update(item_type.clone());
                updated_items.push(item_type);
            }
        }
    }

    Ok(updated_items)
}
