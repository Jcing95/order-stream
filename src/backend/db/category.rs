use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

use crate::common::{errors::Error, requests, types};

use super::DB;
const CATEGORIES: &str = "categories";

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

pub async fn create_category(req: requests::category::Create) -> Result<types::Category, Error> {
    DB.create(CATEGORIES)
        .content(Category {
            id: None,
            name: req.name,
        })
        .await?
        .ok_or_else(|| Error::InternalError("Failed to create category".into()))
}

pub async fn get_categories() -> Result<Vec<types::Category>, Error> {
    let categories: Vec<Category> = DB.select(CATEGORIES).await?;
    Ok(categories.into_iter().map(Into::into).collect())
}

pub async fn get_category(id: &str) -> Result<types::Category, Error> {
    DB.select((CATEGORIES, id))
        .await?
        .ok_or_else(|| Error::NotFound("Category not found".into()))
}

pub async fn update_category(
    id: &str,
    update: requests::category::Update,
) -> Result<types::Category, Error> {
    DB.update((CATEGORIES, id))
        .merge(update)
        .await?
        .ok_or_else(|| Error::InternalError("Failed to update category".into()))
}

pub async fn delete_category(id: &str) -> Result<(), Error> {
    let deleted: Option<Category> = DB.delete((CATEGORIES, id)).await?;
    if deleted.is_none() {
        return Err(Error::NotFound(format!(
            "Category with id {} not found",
            id
        )));
    }
    Ok(())
}
