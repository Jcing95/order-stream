use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait for types that can provide their resource name for WebSocket messaging
pub trait ResourceName {
    const RESOURCE_NAME: &'static str;
}

// Simple macro to implement ResourceName trait
#[macro_export]
macro_rules! impl_resource_name {
    ($type:ty, $name:expr) => {
        impl ResourceName for $type {
            const RESOURCE_NAME: &'static str = $name;
        }
    };
}

// Re-export for convenience
pub use impl_resource_name;

// Import all types that need ResourceData implementations
use crate::common::types::*;

/// Combined trait for resource types that can be used in WebSocket messaging
pub trait ResourceData: ResourceName + Serialize + Debug {}

// Implement ResourceData for all types
impl ResourceData for Category {}
impl ResourceData for User {}
impl ResourceData for Product {}
impl ResourceData for Item {}
impl ResourceData for Order {}
impl ResourceData for Station {}
impl ResourceData for Event {}

/// Generic message for any resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message<T> where
    T: ResourceData, {
    Add(T),
    Update(T),
    Delete(String), // resource id
}

/// WebSocket message envelope with resource type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage<T>
where
    T: ResourceData,
{
    pub resource_type: String,
    pub message: Message<T>,
}

impl<T: ResourceData> WebSocketMessage<T> {
    pub fn new(message: Message<T>) -> Self {
        Self {
            resource_type: T::RESOURCE_NAME.to_string(),
            message,
        }
    }
}

/// Type-erased WebSocket message for frontend consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericWebSocketMessage {
    pub resource_type: String,
    pub message: serde_json::Value,
}