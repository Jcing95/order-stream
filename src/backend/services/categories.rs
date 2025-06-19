use leptos::prelude::*;
use crate::common::types::{CreateCategoryRequest, Category, UpdateCategoryRequest};

#[cfg(feature = "ssr")]
use crate::backend::database::model::category::CategoryRecord;
#[cfg(feature = "ssr")]
use crate::backend::database::Database;
#[cfg(feature = "ssr")]
use surrealdb::sql::Datetime;

#[cfg(feature = "ssr")]
async fn get_db_connection() -> Result<Database, ServerFnError> {
    use crate::backend::database::connect_database;
    use crate::backend::config::AppConfig;
    
    let config = AppConfig::from_env().map_err(|e| ServerFnError::new(e.to_string()))?;
    let db = connect_database(&config.database).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(db)
}

#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let categories: Vec<CategoryRecord> = db
            .select("categories")
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get categories: {}", e)))?;

        Ok(categories.into_iter().map(|record| record.into()).collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateCategory, "/api")]
pub async fn create_category(request: CreateCategoryRequest) -> Result<Category, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        #[derive(serde::Serialize)]
        struct CreateCategoryData {
            name: String,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let category: Option<CategoryRecord> = db
            .create("categories")
            .content(CreateCategoryData {
                name: request.name,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create category: {}", e)))?;

        category.map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to create category: no record returned from database".to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetCategory, "/api")]
pub async fn get_category(id: String) -> Result<Category, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let category: Option<CategoryRecord> = db
            .select(("categories", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get category: {}", e)))?;

        match category {
            Some(record) => Ok(record.into()),
            None => Err(ServerFnError::new(format!("Category with id {} not found", id))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateCategory, "/api")]
pub async fn update_category(id: String, request: UpdateCategoryRequest) -> Result<Category, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        // First check if category exists
        let existing: Option<CategoryRecord> = db
            .select(("categories", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get category: {}", e)))?;

        let mut existing = existing
            .ok_or_else(|| ServerFnError::new(format!("Category with id {} not found", id)))?;

        // Update fields if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(ServerFnError::new("Name cannot be empty".to_string()));
            }
            existing.name = name;
        }

        existing.updated_at = Datetime::default();

        let updated: Option<CategoryRecord> = db
            .update(("categories", id.as_str()))
            .content(existing)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to update category: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to update category: no record returned from database".to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(DeleteCategory, "/api")]
pub async fn delete_category(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let deleted: Option<CategoryRecord> = db
            .delete(("categories", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to delete category: {}", e)))?;

        if deleted.is_none() {
            return Err(ServerFnError::new(format!("Category with id {} not found", id)));
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}