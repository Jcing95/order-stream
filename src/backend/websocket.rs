use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::fmt::Debug;
use leptos::logging::log;

use tokio::sync::broadcast;

use futures_util::{SinkExt, StreamExt};

use serde::{Deserialize, Serialize};

use crate::common::resource_name::ResourceName;

/// Generic message for any resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message<T> {
    Add(T),
    Update(T),
    Delete(String), // resource id
}

/// WebSocket message envelope with resource type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage<T>
where
    T: Serialize,
{
    pub resource_type: String,
    pub message: Message<T>,
}

impl<T: ResourceName + Serialize> WebSocketMessage<T> {
    pub fn new(message: Message<T>) -> Self {
        Self {
            resource_type: T::RESOURCE_NAME.to_string(),
            message,
        }
    }
}

/// Type-erased WebSocket message for broadcast channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub resource_type: String,
    pub data: String, // JSON-serialized Message<T>
}

pub type WebSocketSender = broadcast::Sender<BroadcastMessage>;

use std::sync::OnceLock;

// Global WebSocket sender for server functions
static WS_SENDER: OnceLock<WebSocketSender> = OnceLock::new();

/// Initialize the global WebSocket sender
pub fn init_websocket_sender(sender: WebSocketSender) {
    WS_SENDER
        .set(sender)
        .expect("WebSocket sender already initialized");
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    sender: axum::extract::State<WebSocketSender>,
) -> Response {
    ws.on_upgrade(move |socket| websocket_connection(socket, sender.0))
}

async fn websocket_connection(socket: WebSocket, sender: WebSocketSender) {
    let mut receiver = sender.subscribe();
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Task to forward broadcast messages to WebSocket client
    let send_task = tokio::spawn(async move {
        while let Ok(broadcast_msg) = receiver.recv().await {
            // Send the JSON data directly to client
            if ws_sender
                .send(axum::extract::ws::Message::Text(broadcast_msg.data.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Task to handle incoming WebSocket messages (if needed for bidirectional communication)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                axum::extract::ws::Message::Close(_) => break,
                _ => {} // Handle other message types if needed
            }
        }
    });

    // Wait for either task to complete (connection closed or error)
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

/// Generic broadcast function for adding resources
pub fn broadcast_add<T>(item: T)
where
    T: ResourceName + Serialize + Debug,
{
    let ws_message = WebSocketMessage::new(Message::Add(item));
    if let Ok(json_data) = serde_json::to_string(&ws_message) {
        let broadcast_msg = BroadcastMessage {
            resource_type: T::RESOURCE_NAME.to_string(),
            data: json_data,
        };
        if let Some(sender) = WS_SENDER.get() {
            let _ = sender.send(broadcast_msg);
            log!("Sent add: {:?}", ws_message);

        }
    }
}

/// Generic broadcast function for updating resources
pub fn broadcast_update<T>(item: T)
where
    T: ResourceName + Serialize + Debug,
{
    let ws_message = WebSocketMessage::new(Message::Update(item));
    if let Ok(json_data) = serde_json::to_string(&ws_message) {
        let broadcast_msg = BroadcastMessage {
            resource_type: T::RESOURCE_NAME.to_string(),
            data: json_data,
        };
        if let Some(sender) = WS_SENDER.get() {
            let _ = sender.send(broadcast_msg);
            log!("Sent update: {:?}", ws_message);

        }
    }
}

/// Generic broadcast function for deleting resources
pub fn broadcast_delete<T>(item_id: String)
where
    T: ResourceName + Serialize + Debug,
{
    let ws_message: WebSocketMessage<T> = WebSocketMessage {
        resource_type: T::RESOURCE_NAME.to_string(),
        message: Message::Delete(item_id),
    };
    if let Ok(json_data) = serde_json::to_string(&ws_message) {
        let broadcast_msg = BroadcastMessage {
            resource_type: T::RESOURCE_NAME.to_string(),
            data: json_data,
        };
        if let Some(sender) = WS_SENDER.get() {
            let _ = sender.send(broadcast_msg);
            log!("Sent delete: {:?}", ws_message);
        }
    }
}
