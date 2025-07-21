use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
};

use tokio::sync::broadcast;

use futures_util::{SinkExt, StreamExt};

use serde::{Serialize, Deserialize};

use crate::common::types::Category;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CategoryMessage {
    CategoryAdded(Category),
    CategoryUpdated(Category),
    CategoryDeleted(String), // category id
}

pub type WebSocketSender = broadcast::Sender<CategoryMessage>;

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
        while let Ok(msg) = receiver.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_sender.send(axum::extract::ws::Message::Text(json.into())).await.is_err() {
                    break;
                }
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

pub fn broadcast_category_added(sender: &WebSocketSender, category: Category) {
    let _ = sender.send(CategoryMessage::CategoryAdded(category));
}

pub fn broadcast_category_updated(sender: &WebSocketSender, category: Category) {
    let _ = sender.send(CategoryMessage::CategoryUpdated(category));
}

pub fn broadcast_category_deleted(sender: &WebSocketSender, category_id: String) {
    let _ = sender.send(CategoryMessage::CategoryDeleted(category_id));
}