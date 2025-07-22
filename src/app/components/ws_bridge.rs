use leptos::prelude::*;

#[component]
pub fn WsBridge() -> impl IntoView {
    use codee::string::JsonSerdeCodec;
    #[cfg(feature = "hydrate")]
    use leptos_use::{use_websocket_with_options, UseWebSocketReturn, UseWebSocketOptions, DummyEncoder};
    use leptos::logging::log;
    
    use crate::app::states::websocket;

    
    #[cfg(feature = "hydrate")]
    {
        log!("WsBridge: Starting WebSocket connection");
        let ws_state = websocket::get();
        let ws_url = format!(
            "ws://{}/ws",
            window().location().host().expect("Failed to get host")
        );
        log!("WsBridge: Connecting to {}", ws_url);
        
        
        let UseWebSocketReturn { ready_state, .. } = use_websocket_with_options::<String, String, JsonSerdeCodec, (), DummyEncoder>(
            ws_url.as_str(),
            UseWebSocketOptions::default().on_message(move |message: &String| {
                log!("WsBridge: received message via callback: {:?}", message);
                // Try to determine the resource type from the JSON
                if let Ok(raw_json) = serde_json::from_str::<serde_json::Value>(message) {
                    if let Some(resource_type) = raw_json.get("resource_type").and_then(|v| v.as_str()) {
                        log!("WsBridge: Processing message for resource type: {}", resource_type);
                        ws_state.handle_message(resource_type, message);
                    } else {
                        log!("WsBridge: No resource_type found in message");
                    }
                } else {
                    log!("WsBridge: Failed to parse JSON");
                }
            })
        );

        // Track ready state
        Effect::new(move || {
            log!("WsBridge: Ready state changed: {:?}", ready_state.get());
        });
    }

    view! {<></>}
}
