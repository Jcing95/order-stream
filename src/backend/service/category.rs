use leptos::prelude::*;
use crate::common::types::{CreateCategoryRequest, Category, UpdateCategoryRequest};

#[cfg(feature = "ssr")]
use crate::backend::error::Error;
#[cfg(feature = "ssr")]
use crate::backend::db;

#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::category::get_categories(&db)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
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
        // Validation happens in service layer
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::category::create_category(&db, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
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
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        match db::category::get_category(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?
        {
            Some(category) => Ok(category),
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
        // Validation happens in service layer
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(ServerFnError::new("Name cannot be empty".to_string()));
            }
        }

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::category::update_category(&db, &id, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
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
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::category::delete_category(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}