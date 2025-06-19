pub mod api;

#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod services;
#[cfg(feature = "ssr")]
pub mod errors;
#[cfg(feature = "ssr")]
pub mod database;