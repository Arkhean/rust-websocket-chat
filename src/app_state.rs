use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

#[derive(Serialize, Clone)]
pub struct ChatMessage {
    pub user_login: String,
    pub user_color: String,
    pub message: String,
    pub datetime: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub tx: broadcast::Sender<String>,
    pub history: Vec<ChatMessage>,
}

pub struct AppState {
    pub rooms: HashMap<String, Room>,
    pub tokens: HashMap<String, String>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            rooms: HashMap::new(),
            tokens: HashMap::new(),
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

pub fn init_state() -> SharedState {
    Arc::new(RwLock::new(AppState::new()))
}
