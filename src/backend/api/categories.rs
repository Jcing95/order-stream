use leptos::prelude::*;
use crate::common::types::{CreateCategoryRequest, Category, UpdateCategoryRequest};


#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::categories;
        
        let db = super::get_db_connection().await?;
        let categories = categories::Service::get_categories(&db).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(categories)
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
        use crate::common::errors::AppError;
        use crate::backend::services::categories;
        
        let db = super::get_db_connection().await?;
        let category = categories::Service::create_category(&db, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(category)
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
        use crate::common::errors::AppError;
        use crate::backend::services::categories;
        
        let db = super::get_db_connection().await?;
        let category = categories::Service::get_category(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        match category {
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
        use crate::common::errors::AppError;
        use crate::backend::services::categories;
        
        let db = super::get_db_connection().await?;
        let category = categories::Service::update_category(&db, &id, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(category)
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
        use crate::common::errors::AppError;
        use crate::backend::services::categories;
        
        let db = super::get_db_connection().await?;
        categories::Service::delete_category(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}