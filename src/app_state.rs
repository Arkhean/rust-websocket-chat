use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Struct to store chat message that are exchanged with clients
#[derive(Serialize, Clone)]
pub struct ChatMessage {
    pub user_login: String,
    pub user_color: String,
    pub message: String,
    pub datetime: DateTime<Utc>,
}

/// Struct to store room with chat history
#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub tx: broadcast::Sender<String>,
    pub history: Vec<ChatMessage>,
}

/// The AppState (no database) stores the rooms and the user tokens
pub struct AppState {
    pub rooms: HashMap<String, Room>,
    pub tokens: HashMap<String, String>,
}

/// Simple constructor for AppState
impl AppState {
    fn new() -> AppState {
        AppState {
            rooms: HashMap::new(),
            tokens: HashMap::new(),
        }
    }
}

/// Create a custom type to ease readability
pub type SharedState = Arc<RwLock<AppState>>;

pub fn init_state() -> SharedState {
    Arc::new(RwLock::new(AppState::new()))
}
