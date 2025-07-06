use crate::backend::errors::{AppError, AppResult};
use super::Database;

/// Validates that a string field is not empty after trimming whitespace
pub fn non_empty_string(value: &str, field_name: &str) -> AppResult<()> {
    if value.trim().is_empty() {
        return Err(AppError::ValidationError(format!("{} cannot be empty", field_name)));
    }
    Ok(())
}

/// Validates that a numeric value is not negative
pub fn non_negative_f64(value: f64, field_name: &str) -> AppResult<()> {
    if value < 0.0 {
        return Err(AppError::ValidationError(format!("{} cannot be negative", field_name)));
    }
    Ok(())
}

/// Validates that a quantity is greater than zero
pub fn positive_quantity(quantity: u32, field_name: &str) -> AppResult<()> {
    if quantity == 0 {
        return Err(AppError::ValidationError(format!("{} must be greater than 0", field_name)));
    }
    Ok(())
}

/// Validates email format (basic check)
pub fn email_format(email: &str) -> AppResult<()> {
    if !email.contains('@') || email.len() > 255 {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }
    Ok(())
}

/// Validates that a string field meets length requirements
pub fn string_length(value: &str, field_name: &str, min_len: usize, max_len: usize) -> AppResult<()> {
    let len = value.len();
    if len < min_len {
        return Err(AppError::ValidationError(format!("{} must be at least {} characters long", field_name, min_len)));
    }
    if len > max_len {
        return Err(AppError::ValidationError(format!("{} must be no more than {} characters long", field_name, max_len)));
    }
    Ok(())
}

/// Validates referential integrity before deletion
pub async fn no_references(
    db: &Database,
    referencing_table: &str,
    foreign_key_field: &str,
    id: &str,
    entity_name: &str,
) -> AppResult<()> {
    let query = format!("SELECT COUNT() FROM {} WHERE {} = $value GROUP ALL", referencing_table, foreign_key_field);
    let mut response = db
        .query(&query)
        .bind(("value", id.to_string()))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to check {} references: {}", entity_name, e)))?;

    let count: Option<usize> = response
        .take(0)
        .map_err(|e| AppError::DatabaseError(format!("Failed to parse reference count: {}", e)))?;

    if count.unwrap_or(0) > 0 {
        return Err(AppError::ValidationError(format!(
            "Cannot delete {}: {} records in {} still reference this {}",
            entity_name, count.unwrap_or(0), referencing_table, entity_name
        )));
    }
    
    Ok(())
}