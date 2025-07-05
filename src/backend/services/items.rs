use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};

#[cfg(feature = "ssr")]
use crate::backend::errors::AppError;
#[cfg(feature = "ssr")]
use crate::backend::database;
#[cfg(feature = "ssr")]
use crate::backend::services::auth::{get_authenticated_user_from_request, require_auth, require_role};
#[cfg(feature = "ssr")]
use crate::common::types::UserRole;
#[cfg(feature = "ssr")]
use leptos_axum::extract;
#[cfg(feature = "ssr")]
use tower_cookies::Cookies;

#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // Require staff level access or higher
        let cookies = extract::<Cookies>().await
            .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;
        
        let user = get_authenticated_user_from_request(&cookies).await
            .map_err(|e| ServerFnError::new(format!("Authentication failed: {}", e)))?;
        
        let user = require_auth(user)?;
        require_role(&user, UserRole::Staff)?;

        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::items::get_items(&db)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        // Require admin access for creating items
        let cookies = extract::<Cookies>().await
            .map_err(|e| ServerFnError::new(format!("Failed to get cookies: {}", e)))?;
        
        let user = get_authenticated_user_from_request(&cookies).await
            .map_err(|e| ServerFnError::new(format!("Authentication failed: {}", e)))?;
        
        let user = require_auth(user)?;
        require_role(&user, UserRole::Admin)?;

        // Validation happens in service layer
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::items::create_item(&db, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        let item = database::items::get_item(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
            
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::items::update_item(&db, &id, request)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
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
        let db = database::get_db_connection()
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))?;
        
        database::items::delete_item(&db, &id)
            .await
            .map_err(|e: AppError| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}