use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Product, UpdateItemRequest};

#[cfg(feature = "ssr")]
use crate::backend::error::Error;
#[cfg(feature = "ssr")]
use crate::backend::db;

#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Product>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::product::get_products(&db)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(CreateItem, "/api")]
pub async fn create_item(request: CreateItemRequest) -> Result<Product, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // Validation happens in service layer
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::product::create_product(&db, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(GetItem, "/api")]
pub async fn get_item(id: String) -> Result<Product, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        let item = db::product::get_product(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
            
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
pub async fn update_item(id: String, request: UpdateItemRequest) -> Result<Product, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::product::update_product(&db, &id, request)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
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
        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::product::delete_product(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}