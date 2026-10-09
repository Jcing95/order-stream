use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::backend::schema::*;
use crate::common::types;

// ---------------------------------------------------------------------------
// Users
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
pub struct DbUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub role: Option<String>,
}

impl From<DbUser> for types::User {
    fn from(u: DbUser) -> Self {
        Self {
            id: u.id.to_string(),
            email: u.email,
            role: u.role.parse_role(),
        }
    }
}

// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = categories)]
pub struct DbCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: Option<String>,
}

impl From<DbCategory> for types::Category {
    fn from(c: DbCategory) -> Self {
        Self {
            id: c.id.to_string(),
            name: c.name,
        }
    }
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = events)]
pub struct DbEvent {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = events)]
pub struct UpdateEvent {
    pub name: Option<String>,
}

impl From<DbEvent> for types::Event {
    fn from(e: DbEvent) -> Self {
        Self {
            id: e.id.to_string(),
            name: e.name,
        }
    }
}

// ---------------------------------------------------------------------------
// Products
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = products)]
pub struct DbProduct {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub price: f64,
    pub active: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub category_id: i32,
    pub price: f64,
    pub active: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub category_id: Option<i32>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}

impl From<DbProduct> for types::Product {
    fn from(p: DbProduct) -> Self {
        Self {
            id: p.id.to_string(),
            name: p.name,
            category_id: p.category_id.to_string(),
            price: p.price,
            active: p.active,
        }
    }
}

// ---------------------------------------------------------------------------
// Orders
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = orders)]
pub struct DbOrder {
    pub id: i32,
    pub event_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub event_id: i32,
}

impl From<DbOrder> for types::Order {
    fn from(o: DbOrder) -> Self {
        Self {
            id: o.id.to_string(),
            event_id: o.event_id.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Items
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = items)]
pub struct DbItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: f64,
    pub status: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: f64,
    pub status: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
    pub price: Option<f64>,
    pub status: Option<String>,
}

impl From<DbItem> for types::Item {
    fn from(i: DbItem) -> Self {
        Self {
            id: i.id.to_string(),
            order_id: i.order_id.to_string(),
            product_id: i.product_id.to_string(),
            quantity: i.quantity as u32,
            price: i.price,
            status: i.status.parse_order_status(),
        }
    }
}

// ---------------------------------------------------------------------------
// Stations
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = stations)]
pub struct DbStation {
    pub id: i32,
    pub name: String,
    pub output_status: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = stations)]
pub struct NewStation<'a> {
    pub name: &'a str,
    pub output_status: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = stations)]
pub struct UpdateStation {
    pub name: Option<String>,
    pub output_status: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = station_categories)]
pub struct StationCategory {
    pub station_id: i32,
    pub category_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = station_input_statuses)]
pub struct StationInputStatus {
    pub station_id: i32,
    pub status: String,
}

/// Assemble a full `types::Station` from a DB row + junction rows.
pub fn station_from_parts(
    s: DbStation,
    category_ids: Vec<i32>,
    input_statuses: Vec<String>,
) -> types::Station {
    types::Station {
        id: s.id.to_string(),
        name: s.name,
        category_ids: category_ids.into_iter().map(|id| id.to_string()).collect(),
        input_statuses: input_statuses
            .into_iter()
            .map(|st| st.parse_order_status())
            .collect(),
        output_status: s.output_status.parse_order_status(),
    }
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = settings)]
pub struct DbSettings {
    pub id: i32,
    pub active_event_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = settings)]
pub struct UpdateSettings {
    pub active_event_id: Option<Option<i32>>,
}

impl From<DbSettings> for types::Settings {
    fn from(s: DbSettings) -> Self {
        Self {
            id: s.id.to_string(),
            active_event_id: s.active_event_id.map(|id| id.to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Sessions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sessions)]
pub struct DbSession {
    pub id: String,
    pub user_id: Uuid,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: String,
    pub user_id: Uuid,
    pub expiry_date: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Enum parsing helpers (TEXT <-> Rust enums)
// ---------------------------------------------------------------------------

pub trait ParseRole {
    fn parse_role(&self) -> types::Role;
}

impl ParseRole for String {
    fn parse_role(&self) -> types::Role {
        match self.as_str() {
            "Admin" => types::Role::Admin,
            "Cashier" => types::Role::Cashier,
            "Staff" => types::Role::Staff,
            _ => types::Role::Visitor,
        }
    }
}

pub trait ParseOrderStatus {
    fn parse_order_status(&self) -> types::OrderStatus;
}

impl ParseOrderStatus for String {
    fn parse_order_status(&self) -> types::OrderStatus {
        match self.as_str() {
            "Draft" => types::OrderStatus::Draft,
            "Ready" => types::OrderStatus::Ready,
            "Completed" => types::OrderStatus::Completed,
            "Cancelled" => types::OrderStatus::Cancelled,
            _ => types::OrderStatus::Ordered,
        }
    }
}

pub fn order_status_str(s: types::OrderStatus) -> &'static str {
    match s {
        types::OrderStatus::Draft => "Draft",
        types::OrderStatus::Ordered => "Ordered",
        types::OrderStatus::Ready => "Ready",
        types::OrderStatus::Completed => "Completed",
        types::OrderStatus::Cancelled => "Cancelled",
    }
}

pub fn role_str(r: &types::Role) -> &'static str {
    match r {
        types::Role::Admin => "Admin",
        types::Role::Cashier => "Cashier",
        types::Role::Staff => "Staff",
        types::Role::Visitor => "Visitor",
    }
}
