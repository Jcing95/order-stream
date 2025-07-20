use leptos::prelude::*;
use server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Error {
    ServerFnError(ServerFnErrorErr),
    InternalError(String),
    NotAuthorized(String),
}

impl FromServerFnError for Error {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Error::ServerFnError(value)
    }
}
