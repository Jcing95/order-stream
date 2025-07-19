use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Cashier,
    Staff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Draft,     // Being built, not yet ordered
    Ordered,   // Paid and submitted
    Ready,     // All items ready for pickup
    Completed, // Handed out to customer
    Cancelled, // Cancelled before completion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub order_id: String,   // Reference to Order
    pub product_id: String, // Reference to Item
    pub quantity: u32,
    pub price: f64,          // Unit price when ordered (historical snapshot)
    pub status: OrderStatus, // Individual item status
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
pub struct Order {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    pub category_ids: Vec<String>,
    pub input_statuses: Vec<OrderStatus>,
    pub output_status: OrderStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
}

