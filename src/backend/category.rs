use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
use crate::common::errors::Error;

#[server(CreateCategory, "/api/category")]
pub async fn create_category(
    req: requests::category::Create,
) -> Result<types::Category, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbCategory, NewCategory};
    use crate::backend::websocket::{broadcast_add};
    use crate::backend::schema::categories::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let category: DbCategory = diesel::insert_into(categories)
        .values(NewCategory { name: &req.name })
        .returning(DbCategory::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to create category: {}", e)))?;

    let result: types::Category = category.into();
    broadcast_add(result.clone());
    Ok(result)
}

#[server(GetCategories, "/api/category")]
pub async fn get_categories() -> Result<Vec<types::Category>, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbCategory;
    use crate::backend::schema::categories::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let result: Vec<DbCategory> = categories
        .select(DbCategory::as_select())
        .load(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(result.into_iter().map(Into::into).collect())
}

#[server(GetCategory, "/api/category")]
pub async fn get_category(category_id: String) -> Result<types::Category, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::DbCategory;
    use crate::backend::schema::categories::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let cid: i32 = category_id.parse()
        .map_err(|_| Error::InternalError("Invalid category ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let category: Option<DbCategory> = categories
        .filter(id.eq(cid))
        .select(DbCategory::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(|e| Error::InternalError(e.to_string()))?;

    category.map(Into::into)
        .ok_or_else(|| Error::InternalError("Category not found".to_string()).into())
}

#[server(UpdateCategory, "/api/category")]
pub async fn update_category(
    category_id: String,
    update: requests::category::Update,
) -> Result<types::Category, ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::models::{DbCategory, UpdateCategory};
    use crate::backend::websocket::broadcast_update;
    use crate::backend::schema::categories::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let cid: i32 = category_id.parse()
        .map_err(|_| Error::InternalError("Invalid category ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let category: DbCategory = diesel::update(categories.filter(id.eq(cid)))
        .set(UpdateCategory { name: update.name })
        .returning(DbCategory::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to update category: {}", e)))?;

    let result: types::Category = category.into();
    broadcast_update(result.clone());
    Ok(result)
}

#[server(DeleteCategory, "/api/category")]
pub async fn delete_category(category_id: String) -> Result<(), ServerFnError> {
    use crate::backend::db::get_pool;
    use crate::backend::websocket::broadcast_delete;
    use crate::backend::schema::categories::dsl::*;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let cid: i32 = category_id.parse()
        .map_err(|_| Error::InternalError("Invalid category ID".to_string()))?;

    let pool = get_pool();
    let mut conn = pool.get().await.map_err(|e| Error::InternalError(e.to_string()))?;

    let rows = diesel::delete(categories.filter(id.eq(cid)))
        .execute(&mut conn)
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    if rows == 0 {
        return Err(Error::InternalError(format!("Category {} not found", category_id)).into());
    }
    broadcast_delete::<types::Category>(category_id);
    Ok(())
}
