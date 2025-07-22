use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::resource_type::ResourceName;
use crate::impl_resource_name;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Visitor,
    Staff,
    Cashier,
    Admin,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Draft,     // Being built, not yet ordered
    Ordered,   // Paid and submitted
    Ready,     // All items ready for pickup
    Completed, // Handed out to customer
    Cancelled, // Cancelled before completion
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Item {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub order_id: String,   // Reference to Order
    #[validate(length(min = 1))]
    pub product_id: String, // Reference to Item
    #[validate(range(min = 1))]
    pub quantity: u32,
    #[validate(range(min = 0.0))]
    pub price: f64,          // Unit price when ordered (historical snapshot)
    pub status: OrderStatus, // Individual item status
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1))]
    pub category_id: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(email)]
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Category {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Order {
    #[validate(length(min = 1))]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Station {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(min = 1))]
    pub category_ids: Vec<String>,
    #[validate(length(min = 1))]
    pub input_statuses: Vec<OrderStatus>,
    pub output_status: OrderStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Event {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
}

// Implement ResourceName for all types
impl_resource_name!(Category, "category");
impl_resource_name!(User, "user");
impl_resource_name!(Product, "product");
impl_resource_name!(Item, "item");
impl_resource_name!(Order, "order");
impl_resource_name!(Station, "station");
impl_resource_name!(Event, "event");

