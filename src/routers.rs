use axum::{
    Router,
    response::Html,
    routing::{delete, get, post},
};
use tracing::info;

use crate::app_state::SharedState;

mod chat;
mod login;
mod rooms;

/// Router for the html content
pub async fn chat_html_handler() -> Html<&'static str> {
    info!("GET /");
    Html(include_str!("templates/chat.html"))
}

/// Add all routes
pub fn init_router() -> Router<SharedState> {
    Router::new()
        // --- HTML content ---
        .route("/", get(chat_html_handler))
        // --- Users login ---
        .route("/login", post(login::login))
        // --- Chat socket ---
        .route("/chat/{room_id}", get(chat::chat_handler))
        .route("/chat/{room_id}/history", get(chat::list_history))
        // --- CRUD Rooms ---
        .route("/rooms", get(rooms::list_rooms))
        .route("/rooms", post(rooms::create_room))
        .route("/rooms/{id}", delete(rooms::delete_room))
}
