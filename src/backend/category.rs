use leptos::prelude::*;

use crate::common::{requests, types};

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::backend::ssr::*;

    pub const CATEGORIES: &str = "categories";

    #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
    pub struct Category {
        pub id: Option<Thing>,
        #[validate(length(min = 1, max = 64))]
        pub name: String,
    }

    impl From<Category> for types::Category {
        fn from(record: Category) -> Self {
            Self {
                id: record.id.unwrap().id.to_string(),
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
    DB.create(CATEGORIES)
        .content(Category {
            id: None,
            name: req.name,
        })
        .await?
        .ok_or_else(|| ServerError("Failed to create category".into()))
}

#[server(GetCategories, "/api/category")]
pub async fn get_categories() -> Result<Vec<types::Category>, ServerFnError> {
    let categories: Vec<Category> = DB.select(CATEGORIES).await?;
    Ok(categories.into_iter().map(Into::into).collect())
}

#[server(GetCategory, "/api/category")]
pub async fn get_category(id: String) -> Result<types::Category, ServerFnError> {
    DB.select((CATEGORIES, &id))
        .await?
        .ok_or_else(|| ServerError("Category not found".into()))
}

#[server(UpdateCategory, "/api/category")]
pub async fn update_category(
    id: String,
    update: requests::category::Update,
) -> Result<types::Category, ServerFnError> {
    DB.update((CATEGORIES, &id))
        .merge(update)
        .await?
        .ok_or_else(|| ServerError("Failed to update category".into()))
}

#[server(DeleteCategory, "/api/category")]
pub async fn delete_category(id: String) -> Result<(), ServerFnError> {
    let deleted: Option<Category> = DB.delete((CATEGORIES, &id)).await?;
    if deleted.is_none() {
        return Err(ServerError(format!("Category with id {} not found", id,)));
    }
    Ok(())
}
