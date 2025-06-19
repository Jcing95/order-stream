use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json;

use crate::common::errors::{AppError};
use crate::common::types::{CreateItemRequest, Item, UpdateItemRequest};
use crate::backend::services::items::ItemBusinessService;
use crate::backend::database::service::connection::Database;

pub fn items_router() -> Router<Database> {
    Router::new()
        .route("/api/items", get(get_items))
        .route("/api/items", post(create_item))
        .route("/api/items/{id}", get(get_item))
        .route("/api/items/{id}", put(update_item))
        .route("/api/items/{id}", delete(delete_item))
}

async fn get_items(State(db): State<Database>) -> Result<Json<Vec<Item>>, AppError> {
    let items = ItemBusinessService::get_items(&db).await?;
    Ok(Json(items))
}

async fn create_item(
    State(db): State<Database>,
    Json(request): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let item = ItemBusinessService::create_item(&db, request).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn get_item(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    let item = ItemBusinessService::get_item(&db, &id).await?;
    match item {
        Some(item) => Ok(Json(item)),
        None => Err(AppError::NotFound(format!("Item with id {} not found", id))),
    }
}

async fn update_item(
    State(db): State<Database>,
    Path(id): Path<String>,
    Json(request): Json<UpdateItemRequest>,
) -> Result<Json<Item>, AppError> {
    let item = ItemBusinessService::update_item(&db, &id, request).await?;
    Ok(Json(item))
}

async fn delete_item(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    ItemBusinessService::delete_item(&db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Error handling for Axum
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            AppError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
            ),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
