use leptos::prelude::*;
use crate::common::types::{CreateCategoryRequest, Category, UpdateCategoryRequest};

#[cfg(feature = "ssr")]
use crate::backend::{database, services::utils::*};

/// Get all categories - requires staff level access
#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        database::categories::get_categories(&db)
            .await
            .map_err(map_db_error)
    }).await
}

/// Create a new category - requires admin access
#[server(CreateCategory, "/api")]
pub async fn create_category(request: CreateCategoryRequest) -> Result<Category, ServerFnError> {
    // Validate request
    request.validate().map_err(|e| ServerFnError::new(e))?;
    
    with_admin_auth(|db, _user| async move {
        database::categories::create_category(&db, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Get a specific category by ID - requires staff level access
#[server(GetCategory, "/api")]
pub async fn get_category(id: String) -> Result<Category, ServerFnError> {
    with_staff_auth(|db, _user| async move {
        let category = database::categories::get_category(&db, &id)
            .await
            .map_err(map_db_error)?;
        
        ok_or_not_found(category)
    }).await
}

/// Update an existing category - requires admin access
#[server(UpdateCategory, "/api")]
pub async fn update_category(id: String, request: UpdateCategoryRequest) -> Result<Category, ServerFnError> {
    // Validate request
    if let Some(ref name) = request.name {
        if name.trim().is_empty() {
            return Err(ServerFnError::new(INVALID_REQUEST));
        }
    }
    
    with_admin_auth(|db, _user| async move {
        database::categories::update_category(&db, &id, request)
            .await
            .map_err(map_db_error)
    }).await
}

/// Delete a category - requires admin access
#[server(DeleteCategory, "/api")]
pub async fn delete_category(id: String) -> Result<(), ServerFnError> {
    with_admin_auth(|db, _user| async move {
        database::categories::delete_category(&db, &id)
            .await
            .map_err(map_db_error)
    }).await
}