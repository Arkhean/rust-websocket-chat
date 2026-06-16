use axum::extract::ws::{Message, WebSocket};
use axum::{
    Json,
    extract::{Path, Query, State, ws::WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tokio::sync::broadcast;
use tracing::info;

use crate::app_state::{ChatMessage, SharedState};

#[derive(Deserialize)]
pub struct ConnectionParams {
    token: String,
}

/// Router for the websocket creation
pub async fn chat_handler(
    Path(room_id): Path<String>,
    Query(params): Query<ConnectionParams>,
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let state_lock = state.read().unwrap();

    // extract pseudo and room broadcaster from state
    let user_pseudo = state_lock.tokens.get(&params.token).cloned();
    let room_tx = state_lock.rooms.get(&room_id).map(|r| r.tx.clone());
    let state_for_socket = state.clone();
    let room_id_for_socket = room_id.clone();

    match (user_pseudo, room_tx) {
        (Some(pseudo), Some(tx)) => {
            drop(state_lock);
            ws.on_upgrade(move |socket| {
                handle_chat_socket(
                    socket,
                    tx,
                    pseudo,
                    state_for_socket,
                    room_id_for_socket,
                )
            })
        }
        (None, _) => (StatusCode::UNAUTHORIZED, "Invalid token.").into_response(),
        (_, None) => (StatusCode::NOT_FOUND, "Room not found.").into_response(),
    }
}

/// Socket handler
pub async fn handle_chat_socket(
    socket: WebSocket,
    tx: broadcast::Sender<String>,
    pseudo: String,
    state: SharedState,
    room_id: String,
) {
    info!("{} joined the chat", pseudo);

    let colors = [
        "#e6194b", "#3cb44b", "#ffe119", "#4363d8", "#f58231", "#911eb4", "#42d4f4",
    ];
    // Choose random color
    let color = colors[pseudo.len() % colors.len()].to_string();

    // split socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // First Task: wait for messages from the broadcaster and send them to the client.
    let mut rx = tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let login_for_recv_task = pseudo.clone();

    // Second Task: wait for client messages and forward them to the broadcaster
    let tx = tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            info!("message from {}: {}", login_for_recv_task, text);
            let msg = ChatMessage {
                user_login: login_for_recv_task.clone(),
                user_color: color.clone(),
                message: text.to_string(),
                datetime: Utc::now(),
            };

            {
                if let Ok(mut state_lock) = state.write()
                    && let Some(room) = state_lock.rooms.get_mut(&room_id)
                {
                    room.history.push(msg.clone());
                }
            }

            let text = serde_json::to_string(&msg).unwrap();
            let _ = tx.send(text);
        }
    });

    // start both tasks, if one stops, stop the other one
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    info!("{} left the chat", pseudo);
}

pub async fn list_history(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let state_lock = state.read().unwrap();
    state_lock
        .rooms
        .get(&id)
        .map(|room| Json(room.history.clone()).into_response())
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
