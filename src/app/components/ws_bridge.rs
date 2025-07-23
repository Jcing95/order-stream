use crate::app::states::websocket;
use crate::common::resource_type::GenericWebSocketMessage;
use codee::string::JsonSerdeCodec;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_use::{use_websocket, UseWebSocketReturn};

#[cfg(feature = "hydrate")]
fn get_websocket_url() -> String {
    use web_sys::window;
    let window = window().expect("should have a window in this context");
    let location = window.location();
    let host = location.host().unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let protocol = if location.protocol().unwrap_or_else(|_| "http:".to_string()) == "https:" {
        "wss:"
    } else {
        "ws:"
    };
    format!("{}//{}/ws", protocol, host)
}

#[cfg(not(feature = "hydrate"))]
fn get_websocket_url() -> String {
    "ws://127.0.0.1:3000/ws".to_string()
}

#[component]
pub fn WsBridge() -> impl IntoView {
    let ws_url = get_websocket_url();
    log!("Connecting to WebSocket at: {}", ws_url);

    let UseWebSocketReturn {
        message,
        ready_state,..
    } = use_websocket::<GenericWebSocketMessage, GenericWebSocketMessage, JsonSerdeCodec>(
        &ws_url,
    );
    let ws_state = websocket::get();

    Effect::new(move |_| {
        if let Some(msg) = message.get() {
            log!("Received websocket message: {:?}", msg);
            // Convert back to JSON string for the handler
            if let Ok(json_str) = serde_json::to_string(&msg) {
                ws_state.handle_message(&msg.resource_type, &json_str);
            } else {
                log!("WsBridge: Failed to serialize message back to JSON");
            }
        }
    });

    let ws_state = websocket::get();
    Effect::new(move |_| {
        ws_state.set_state(ready_state.get())
    });

    view! {
        <></>
    }
}