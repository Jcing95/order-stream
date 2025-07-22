use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use crate::common::types::*;
use crate::common::resource_type::*;

#[derive(Debug, Clone)]
pub struct WebSocketState {
    pub state: RwSignal<ConnectionReadyState>,
    pub categories: RwSignal<Option<Message<Category>>>,
    pub users: RwSignal<Option<Message<User>>>,
    pub products: RwSignal<Option<Message<Product>>>,
    pub items: RwSignal<Option<Message<Item>>>,
    pub orders: RwSignal<Option<Message<Order>>>,
    pub stations: RwSignal<Option<Message<Station>>>,
    pub events: RwSignal<Option<Message<Event>>>,
}

impl WebSocketState {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(ConnectionReadyState::Connecting),
            categories: RwSignal::new(None),
            users: RwSignal::new(None),
            products: RwSignal::new(None),
            items: RwSignal::new(None),
            orders: RwSignal::new(None),
            stations: RwSignal::new(None),
            events: RwSignal::new(None),
        }
    }

    pub fn set_state(&self, state: ConnectionReadyState) {
        self.state.set(state);
    }

    pub fn handle_message(&self, resource_type: &str, json_str: &str) {
        match resource_type {
            "category" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Category>>(json_str) {
                    self.categories.set(Some(ws_msg.message));
                }
            }
            "user" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<User>>(json_str) {
                    self.users.set(Some(ws_msg.message));
                }
            }
            "product" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Product>>(json_str) {
                    self.products.set(Some(ws_msg.message));
                }
            }
            "item" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Item>>(json_str) {
                    self.items.set(Some(ws_msg.message));
                }
            }
            "order" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Order>>(json_str) {
                    self.orders.set(Some(ws_msg.message));
                }
            }
            "station" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Station>>(json_str) {
                    self.stations.set(Some(ws_msg.message));
                }
            }
            "event" => {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage<Event>>(json_str) {
                    self.events.set(Some(ws_msg.message));
                }
            }
            _ => {} // Unknown resource type
        }
    }
}

pub fn provide() -> WebSocketState {
    let websocket_state = WebSocketState::new();
    provide_context(websocket_state.clone());
    websocket_state
}

pub fn get() -> WebSocketState {
    expect_context::<WebSocketState>()
}
