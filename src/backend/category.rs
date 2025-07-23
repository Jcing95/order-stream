use leptos::prelude::*;

use crate::common::{requests, types};

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

    pub const CATEGORIES: &str = "categories";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Category {
        pub id: Option<RecordId>,
        #[validate(length(min = 1, max = 64))]
        pub name: String,
    }

    impl From<Category> for types::Category {
        fn from(record: Category) -> Self {
            Self {
                id: record.id.unwrap().key().to_string(),
                name: record.name,
            }
        }
    }
}
#[cfg(feature = "ssr")]
use ssr::*;

#[server(CreateCategory, "/api/category")]
pub async fn create_category(
    req: requests::category::Create,
) -> Result<types::Category, ServerFnError> {
    let c: Option<Category> = DB
        .create(CATEGORIES)
        .content(Category {
            id: None,
            name: req.name,
        })
        .await?;

    if let Some(category) = c {
        let result: types::Category = category.clone().into();
        broadcast_add(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to create category".into()))
    }
}

#[server(GetCategories, "/api/category")]
pub async fn get_categories() -> Result<Vec<types::Category>, ServerFnError> {
    let categories: Vec<Category> = DB.select(CATEGORIES).await?;
    Ok(categories.into_iter().map(Into::into).collect())
}

#[server(GetCategory, "/api/category")]
pub async fn get_category(id: String) -> Result<types::Category, ServerFnError> {
    let category: Option<Category> = DB.select((CATEGORIES, &id)).await?;
    category
        .map(Into::into)
        .ok_or_else(|| ServerError("Category not found".into()))
}

#[server(UpdateCategory, "/api/category")]
pub async fn update_category(
    id: String,
    update: requests::category::Update,
) -> Result<types::Category, ServerFnError> {
    // Get the existing category
    let existing_category: Option<Category> = DB.select((CATEGORIES, &id)).await?;
    if existing_category.is_none() {
        return Err(ServerError("Category not found".into()));
    }
    let category = existing_category.unwrap();
    let updated = Category {
        id: category.id,
        name: update.name.or_else(|| Some(category.name)).unwrap(),
    };
    // Update the category in the database
    let updated_category: Option<Category> = DB
        .update((CATEGORIES, &id))
        .content(updated)
        .await?;

    if let Some(category) = updated_category {
        let result: types::Category = category.into();
        broadcast_update(result.clone());
        Ok(result)
    } else {
        Err(ServerError("Failed to update category".into()))
    }
}

#[server(DeleteCategory, "/api/category")]
pub async fn delete_category(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Category> = DB.delete((CATEGORIES, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Category with id {} not found", id)));
    }
    broadcast_delete::<types::Category>(id);
    Ok(())
}
