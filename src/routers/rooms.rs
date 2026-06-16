use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::app_state::{Room, SharedState};

#[derive(Deserialize)]
pub struct CreateRoomInput {
    name: String,
}

#[derive(Serialize)]
pub struct RoomResponse {
    id: String,
    name: String,
}

pub async fn create_room(
    State(state): State<SharedState>,
    Json(payload): Json<CreateRoomInput>,
) -> Json<RoomResponse> {
    let mut state = state.write().unwrap();
    let id = Uuid::new_v4().to_string();

    let (tx, _rx) = broadcast::channel(100);

    let new_room = Room {
        id: id.clone(),
        name: payload.name.clone(),
        tx,
        history: Vec::new(),
    };

    state.rooms.insert(id.clone(), new_room);

    Json(RoomResponse {
        id,
        name: payload.name,
    })
}

pub async fn list_rooms(State(state): State<SharedState>) -> Json<Vec<RoomResponse>> {
    let state = state.read().unwrap();
    let rooms = state
        .rooms
        .values()
        .map(|r| RoomResponse {
            id: r.id.clone(),
            name: r.name.clone(),
        })
        .collect();

    Json(rooms)
}

pub async fn delete_room(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    let mut state = state.write().unwrap();
    if state.rooms.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
