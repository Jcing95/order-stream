use leptos::prelude::*;
use server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Error {
    ServerFnError(ServerFnErrorErr),
    InternalError(String),
    NotAuthorized(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ServerFnError(e) => write!(f, "Server function error: {}", e),
            Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Error::NotAuthorized(msg) => write!(f, "Not authorized: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl FromServerFnError for Error {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Error::ServerFnError(value)
    }
}
