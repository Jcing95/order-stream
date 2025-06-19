// Services contain Leptos server functions (available to both client and server)
pub mod services;

// Backend-only modules (SSR only)
#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod errors;
#[cfg(feature = "ssr")]
pub mod database;