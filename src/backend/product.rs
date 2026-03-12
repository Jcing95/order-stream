use crate::common::{requests, types};

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateProduct, "/api/product")]
pub async fn create_product(
    req: requests::product::Create,
) -> Result<types::Product, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbProduct, NewProduct};
    use crate::backend::websocket::broadcast_add;
    use crate::backend::schema::products::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let cid: i32 = req.category_id.parse()
        .map_err(|_| Error::InternalError("Invalid category ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let product: DbProduct = diesel::insert_into(products)
        .values(NewProduct { name: &req.name, category_id: cid, price: req.price, active: true })
        .returning(DbProduct::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create product: {}", e)))?;

    let result: types::Product = product.into();
    broadcast_add(result.clone());
    Ok(result)
}

#[server(GetProducts, "/api/product")]
pub async fn get_products() -> Result<Vec<types::Product>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbProduct;
    use crate::backend::schema::products::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbProduct> = products
        .select(DbProduct::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetProduct, "/api/product")]
pub async fn get_product(product_id: String) -> Result<types::Product, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbProduct;
    use crate::backend::schema::products::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pid: i32 = product_id.parse()
        .map_err(|_| Error::InternalError("Invalid product ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let product: Option<DbProduct> = products
        .filter(id.eq(pid))
        .select(DbProduct::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    product.map(Into::into)
        .ok_or_else(|| Error::InternalError("Product not found".to_string()).into())
}

#[server(UpdateProduct, "/api/product")]
pub async fn update_product(
    product_id: String,
    update: requests::product::Update,
) -> Result<types::Product, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbProduct, UpdateProduct};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::products::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pid: i32 = product_id.parse()
        .map_err(|_| Error::InternalError("Invalid product ID".to_string()))?;

    let cid: Option<i32> = update.category_id
        .map(|s| s.parse().map_err(|_| Error::InternalError("Invalid category ID".to_string())))
        .transpose()?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let changes = UpdateProduct {
        name: update.name,
        category_id: cid,
        price: update.price,
        active: update.active,
    };

    let product: DbProduct = diesel::update(products.filter(id.eq(pid)))
        .set(&changes)
        .returning(DbProduct::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update product: {}", e)))?;

    let result: types::Product = product.into();
    broadcast_update(result.clone());
    Ok(result)
}

#[server(DeleteProduct, "/api/product")]
pub async fn delete_product(product_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::products::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pid: i32 = product_id.parse()
        .map_err(|_| Error::InternalError("Invalid product ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(products.filter(id.eq(pid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Product {} not found", product_id)).into());
    }
    broadcast_delete::<types::Product>(product_id);
    Ok(())
}
