use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,           // SurrealDB format: "item:uuid"
    pub name: String,
    pub category_id: String,  // Foreign key to Category
    pub price: f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub category_id: String,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}

impl CreateItemRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.category_id.trim().is_empty() {
            return Err("Category ID cannot be empty".to_string());
        }
        if self.price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
}

impl CreateCategoryRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Draft,      // Being built, not yet ordered
    Ordered,    // Paid and submitted  
    Ready,      // All items ready for pickup
    Completed,  // Handed out to customer
    Cancelled,  // Cancelled before completion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub sequential_id: u32,    // Customer-facing order number (#001, #002, etc.)
    pub total_price: f64,      // Calculated from OrderItems
    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,      // Reference to Order
    pub item_id: String,       // Reference to Item
    pub quantity: u32,
    pub price: f64,           // Unit price when ordered (historical snapshot)
    pub status: OrderStatus,   // Individual item status
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub order_id: String,
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderItemRequest {
    pub item_id: Option<String>,  // Allow changing the item (for corrections)
    pub quantity: Option<u32>,
    pub status: Option<OrderStatus>, // Allow updating status
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOrderItemUpdate {
    pub order_item_ids: Vec<String>,
    pub new_status: OrderStatus,
}

impl CreateOrderItemRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.order_id.trim().is_empty() {
            return Err("Order ID cannot be empty".to_string());
        }
        if self.item_id.trim().is_empty() {
            return Err("Item ID cannot be empty".to_string());
        }
        if self.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }
        Ok(())
    }
}

impl BulkOrderItemUpdate {
    pub fn validate(&self) -> Result<(), String> {
        if self.order_item_ids.is_empty() {
            return Err("At least one order item ID must be provided".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,                    // "Bar", "Kitchen", "Pickup"
    pub category_ids: Vec<String>,       // Filter: show only these categories
    pub input_statuses: Vec<OrderStatus>, // Show orders with items in these statuses
    pub output_status: OrderStatus,      // What status to update items to
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStationRequest {
    pub name: String,
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<OrderStatus>,
    pub output_status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStationRequest {
    pub name: Option<String>,
    pub category_ids: Option<Vec<String>>,
    pub input_statuses: Option<Vec<OrderStatus>>,
    pub output_status: Option<OrderStatus>,
}

impl CreateStationRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Station name cannot be empty".to_string());
        }
        if self.category_ids.is_empty() {
            return Err("At least one category must be selected".to_string());
        }
        if self.input_statuses.is_empty() {
            return Err("At least one input status must be selected".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,    // Full access - admin panel, cashier, and views
    Cashier,  // Cashier access and views
    Staff,    // View-only access to stations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub role: UserRole,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub session_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSecurityInfo {
    pub email: String,
    pub active: bool,
    pub failed_login_attempts: u32,
    pub locked_until: Option<String>,
    pub active_sessions_count: usize,
    pub recent_failed_attempts_count: u32,
    pub last_login: Option<String>,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if self.password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }
        Ok(())
    }
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        if self.password.trim().is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        Ok(())
    }
}