use leptos::prelude::*;

#[component]
pub fn WsBridge() -> impl IntoView {
    use codee::string::JsonSerdeCodec;
    use leptos_use::{use_websocket, UseWebSocketReturn};
    use serde::{Serialize, Deserialize};
    use crate::common::types::*;
    use leptos::logging::log;

    let UseWebSocketReturn { message, .. } = use_websocket::<String, String, JsonSerdeCodec>(
        format!(
            "ws://{}/ws",
            window().location().host().expect("Failed to get host")
        )
        .as_str(),
    );

    Effect::new(move || {
        if let Some(json_str) = message.get() {
            log!("received message {:?}", &json_str);
            // Try to determine the resource type from the JSON
            if let Ok(raw_json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(resource_type) = raw_json.get("resource_type").and_then(|v| v.as_str())
                {
                    match resource_type {
                        "category" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Category>>(&json_str)
                            {
                                categories.set(Some(ws_msg.message));
                            }
                        }
                        "user" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<User>>(&json_str)
                            {
                                users.set(Some(ws_msg.message));
                            }
                        }
                        "product" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Product>>(&json_str)
                            {
                                products.set(Some(ws_msg.message));
                            }
                        }
                        "item" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Item>>(&json_str)
                            {
                                items.set(Some(ws_msg.message));
                            }
                        }
                        "order" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Order>>(&json_str)
                            {
                                orders.set(Some(ws_msg.message));
                            }
                        }
                        "station" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Station>>(&json_str)
                            {
                                stations.set(Some(ws_msg.message));
                            }
                        }
                        "event" => {
                            if let Ok(ws_msg) =
                                serde_json::from_str::<WebSocketMessage<Event>>(&json_str)
                            {
                                events.set(Some(ws_msg.message));
                            }
                        }
                        _ => {} // Unknown resource type
                    }
                }
            }
        }
    });

    view! {<></>}
}
