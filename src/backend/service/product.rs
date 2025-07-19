use leptos::prelude::*;
use crate::common::types::{Product, ProductUpdate};

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
pub async fn create_item(name: String, price: f64, category_id: String) -> Result<Product, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if name.trim().is_empty() {
            return Err(ServerFnError::new("Name cannot be empty".to_string()));
        }

        let db = db::get_db_connection()
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))?;
        
        db::product::create_product(&db, name, price, category_id)
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
        
        db::product::get_product(&db, &id)
            .await
            .map_err(|e: Error| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}

#[server(UpdateItem, "/api")]
pub async fn update_item(id: String, request: ProductUpdate) -> Result<Product, ServerFnError> {
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