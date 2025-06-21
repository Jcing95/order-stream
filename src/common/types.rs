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