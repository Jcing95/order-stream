#[cfg(feature = "hydrate")]
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_use::{use_websocket, UseWebSocketReturn};
#[cfg(feature = "hydrate")]
use codee::string::JsonSerdeCodec;
#[cfg(feature = "hydrate")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "hydrate")]
use crate::common::types::*;

#[cfg(feature = "hydrate")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message<T> {
    Add(T),
    Update(T),
    Delete(String),
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage<T> {
    pub resource_type: String,
    pub message: Message<T>,
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Clone)]
pub struct WebSocketState {
    // Typed signals for each resource
    pub categories: RwSignal<Option<Message<Category>>>,
    pub users: RwSignal<Option<Message<User>>>,
    pub products: RwSignal<Option<Message<Product>>>,
    pub items: RwSignal<Option<Message<Item>>>,
    pub orders: RwSignal<Option<Message<Order>>>,
    pub stations: RwSignal<Option<Message<Station>>>,
    pub events: RwSignal<Option<Message<Event>>>,
}

#[cfg(feature = "hydrate")]
impl WebSocketState {
    pub fn new() -> Self {
        let categories = RwSignal::new(None);
        let users = RwSignal::new(None);
        let products = RwSignal::new(None);
        let items = RwSignal::new(None);
        let orders = RwSignal::new(None);
        let stations = RwSignal::new(None);
        let events = RwSignal::new(None);

        // Set up WebSocket connection
        Effect::new({
            let categories = categories;
            let users = users;
            let products = products;
            let items = items;
            let orders = orders;
            let stations = stations;
            let events = events;
            
            move |_| {
                let UseWebSocketReturn { message, .. } = use_websocket::<String, String, JsonSerdeCodec>(
                    format!("ws://{}/ws", window().location().host().expect("Failed to get host")).as_str()
                );
                
                // Handle incoming WebSocket messages and route to appropriate signals
                Effect::new(move |_| {
                    if let Some(json_str) = message.get() {
                        // Try to determine the resource type from the JSON
                        if let Ok(raw_json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                            if let Some(resource_type) = raw_json.get("resource_type").and_then(|v| v.as_str()) {
                                match resource_type {
                                    "category" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Category>>(&json_str) {
                                            categories.set(Some(ws_msg.message));
                                        }
                                    },
                                    "user" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<User>>(&json_str) {
                                            users.set(Some(ws_msg.message));
                                        }
                                    },
                                    "product" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Product>>(&json_str) {
                                            products.set(Some(ws_msg.message));
                                        }
                                    },
                                    "item" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Item>>(&json_str) {
                                            items.set(Some(ws_msg.message));
                                        }
                                    },
                                    "order" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Order>>(&json_str) {
                                            orders.set(Some(ws_msg.message));
                                        }
                                    },
                                    "station" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Station>>(&json_str) {
                                            stations.set(Some(ws_msg.message));
                                        }
                                    },
                                    "event" => {
                                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Event>>(&json_str) {
                                            events.set(Some(ws_msg.message));
                                        }
                                    },
                                    _ => {} // Unknown resource type
                                }
                            }
                        }
                    }
                });
            }
        });

        Self {
            categories,
            users,
            products,
            items,
            orders,
            stations,
            events,
        }
    }

    pub fn categories(&self) -> ReadSignal<Option<Message<Category>>> {
        self.categories.read_only()
    }

    pub fn users(&self) -> ReadSignal<Option<Message<User>>> {
        self.users.read_only()
    }

    pub fn products(&self) -> ReadSignal<Option<Message<Product>>> {
        self.products.read_only()
    }

    pub fn items(&self) -> ReadSignal<Option<Message<Item>>> {
        self.items.read_only()
    }

    pub fn orders(&self) -> ReadSignal<Option<Message<Order>>> {
        self.orders.read_only()
    }

    pub fn stations(&self) -> ReadSignal<Option<Message<Station>>> {
        self.stations.read_only()
    }

    pub fn events(&self) -> ReadSignal<Option<Message<Event>>> {
        self.events.read_only()
    }
}

#[cfg(feature = "hydrate")]
pub fn provide() -> WebSocketState {
    let websocket_state = WebSocketState::new();
    provide_context(websocket_state.clone());
    websocket_state
}

#[cfg(feature = "hydrate")]
pub fn get() -> WebSocketState {
    expect_context::<WebSocketState>()
}

// Stub implementations for SSR
#[cfg(not(feature = "hydrate"))]
pub fn provide() {}

#[cfg(not(feature = "hydrate"))]
pub fn get() {}