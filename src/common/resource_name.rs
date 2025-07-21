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