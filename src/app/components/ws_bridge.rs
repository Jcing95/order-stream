use crate::app::states::websocket;
use codee::string::JsonSerdeCodec;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_use::{use_websocket, UseWebSocketReturn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebSocketMessage {
    resource_type: String,
    message: serde_json::Value,
}

#[component]
pub fn WsBridge() -> impl IntoView {
    use leptos_use::core::ConnectionReadyState;
    use leptos_use::{use_websocket, UseWebSocketReturn};
    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        ..
    } = use_websocket::<WebSocketMessage, WebSocketMessage, JsonSerdeCodec>(
        format!("ws://{}/ws", "127.0.0.1:3000").as_str(),
    );
    let ws_state = websocket::get();

    let (messages, set_messages) = signal(Vec::<String>::new());

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

    view! {
        <></>
        // <div>
        //     <p>
        //         "Status: "
        //         {move || match ready_state.get() {
        //             ConnectionReadyState::Connecting => "Connecting".to_string(),
        //             ConnectionReadyState::Open => "Open".to_string(),
        //             ConnectionReadyState::Closing => "Closing".to_string(),
        //             ConnectionReadyState::Closed => "Closed".to_string(),
        //         }}
        //     </p>
        //     <button on:click=on_click>"Send Message"</button>
        //     <ul>
        //         {move || messages.get().into_iter().map(|msg| {
        //             view! {<li>{msg}</li>}
        //         }).collect::<Vec<_>>()}
        //     </ul>
        // </div>
    }
}

// #[component]
// pub fn WsBridge() -> impl IntoView {
//     let UseWebSocketReturn {
//         message,
//         ready_state,
//         ..
//     } = use_websocket::<String, String, JsonSerdeCodec>(
//         format!(
//             "ws://{}/ws",
//             "127.0.0.1:3000"
//         )
//         .as_str(),
//     );
//     let ws_state = websocket::get();

//     Effect::new(move || {
//         if let Some(json_str) = message.get() {
//             log!("received message {:?}", &json_str);
//             // Try to determine the resource type from the JSON
//             if let Ok(raw_json) = serde_json::from_str::<serde_json::Value>(&json_str) {
//                 if let Some(resource_type) = raw_json.get("resource_type").and_then(|v| v.as_str())
//                 {
//                     ws_state.handle_message(resource_type, json_str.as_str());
//                 } else {
//                     log!("WsBridge: Failed to parse JSON");
//                 }
//             }
//         }
//     });

//     // Track ready state
//     Effect::new(move || {
//         log!("WsBridge: Ready state changed: {:?}", ready_state.get());
//     });

//     view! {<></>}
// }
