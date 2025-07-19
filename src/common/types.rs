use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Cashier,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category_id: String,
    pub price: f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub role: Role,
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
    pub name: String,       
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<OrderStatus>,
    pub output_status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStationRequest {
    pub name: String,
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<OrderStatus>,
    pub output_status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationUpdate {
    pub category_ids: Option<Vec<String>>,
    pub input_statuses: Option<Vec<OrderStatus>>,
    pub output_status: Option<OrderStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUpdate {
    pub name: Option<String>,
    pub category_id: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}

