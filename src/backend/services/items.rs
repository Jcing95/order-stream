use leptos::prelude::*;
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};

#[cfg(feature = "ssr")]
use crate::common::errors::{AppError, AppResult};
#[cfg(feature = "ssr")]
use crate::backend::database::model::item::ItemRecord;
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

#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let db = get_db_connection().await?;
        
        let items: Vec<ItemRecord> = db
            .select("items")
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get items: {}", e)))?;

        Ok(items.into_iter().map(|record| record.into()).collect())
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
        let db = get_db_connection().await?;
        
        request
            .validate()
            .map_err(|e| ServerFnError::new(e))?;

        #[derive(serde::Serialize)]
        struct CreateItemData {
            name: String,
            category_id: String,
            price: f64,
            active: bool,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let item: Option<ItemRecord> = db
            .create("items")
            .content(CreateItemData {
                name: request.name,
                category_id: request.category_id,
                price: request.price,
                active: true,
                created_at: Datetime::default(),
                updated_at: Datetime::default(),
            })
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create item: {}", e)))?;

        item.map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to create item: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let item: Option<ItemRecord> = db
            .select(("items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get item: {}", e)))?;

        match item {
            Some(record) => Ok(record.into()),
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
        let db = get_db_connection().await?;
        
        // First check if item exists
        let existing: Option<ItemRecord> = db
            .select(("items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get item: {}", e)))?;

        let mut existing = existing
            .ok_or_else(|| ServerFnError::new(format!("Item with id {} not found", id)))?;

        // Update fields if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(ServerFnError::new("Name cannot be empty".to_string()));
            }
            existing.name = name;
        }
        if let Some(category_id) = request.category_id {
            if category_id.trim().is_empty() {
                return Err(ServerFnError::new("Category ID cannot be empty".to_string()));
            }
            existing.category_id = category_id;
        }
        if let Some(price) = request.price {
            if price < 0.0 {
                return Err(ServerFnError::new("Price cannot be negative".to_string()));
            }
            existing.price = price;
        }
        if let Some(active) = request.active {
            existing.active = active;
        }

        existing.updated_at = Datetime::default();

        let updated: Option<ItemRecord> = db
            .update(("items", id.as_str()))
            .content(existing)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to update item: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| ServerFnError::new("Failed to update item: no record returned from database".to_string()))
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
        let db = get_db_connection().await?;
        
        let deleted: Option<ItemRecord> = db
            .delete(("items", id.as_str()))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to delete item: {}", e)))?;

        if deleted.is_none() {
            return Err(ServerFnError::new(format!("Item with id {} not found", id)));
        }

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client side")
    }
}