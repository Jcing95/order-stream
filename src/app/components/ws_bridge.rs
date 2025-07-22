use crate::app::states::websocket;
use crate::common::resource_type::GenericWebSocketMessage;
use codee::string::JsonSerdeCodec;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_use::{use_websocket, UseWebSocketReturn};

#[component]
pub fn WsBridge() -> impl IntoView {
    let UseWebSocketReturn {
        message,
        ready_state,..
    } = use_websocket::<GenericWebSocketMessage, GenericWebSocketMessage, JsonSerdeCodec>(
        format!("ws://{}/ws", "127.0.0.1:3000").as_str(),
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