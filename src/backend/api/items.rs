use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};


#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::items;
        
        let db = super::get_db_connection().await?;
        let items = items::Service::get_items(&db).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(items)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateItem, "/api")]
pub async fn create_item(request: CreateItemRequest) -> Result<Item, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::items;
        
        let db = super::get_db_connection().await?;
        let item = items::Service::create_item(&db, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(item)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetItem, "/api")]
pub async fn get_item(id: String) -> Result<Item, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::items;
        
        let db = super::get_db_connection().await?;
        let item = items::Service::get_item(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        match item {
            Some(item) => Ok(item),
            None => Err(ServerFnError::new(format!("Item with id {} not found", id))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateItem, "/api")]
pub async fn update_item(id: String, request: UpdateItemRequest) -> Result<Item, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::items;
        
        let db = super::get_db_connection().await?;
        let item = items::Service::update_item(&db, &id, request).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(item)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(DeleteItem, "/api")]
pub async fn delete_item(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::common::errors::AppError;
        use crate::backend::services::items;
        
        let db = super::get_db_connection().await?;
        items::Service::delete_item(&db, &id).await.map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

