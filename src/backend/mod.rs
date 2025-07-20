#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod db;

#[cfg(not(feature = "ssr"))]
pub mod ssr {}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use super::db::DB;
    pub use crate::common::types;
    pub use serde::{Deserialize, Serialize};
    pub use surrealdb::sql::{Datetime, Thing};
    pub use validator::Validate;
    pub use leptos::server_fn::error::ServerFnError::ServerError;
}

pub mod category;
pub mod event;
pub mod item;
pub mod order;
pub mod product;
pub mod station;
pub mod user;
