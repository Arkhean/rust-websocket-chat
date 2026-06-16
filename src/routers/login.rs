use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::app_state::SharedState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pseudo: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

/// Router for login
/// Users give a pseudo
/// and the server gives them a token to be used to identify in chat rooms
pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let token = Uuid::new_v4().to_string();
    state
        .write()
        .unwrap()
        .tokens
        .insert(token.clone(), payload.pseudo.clone());

    info!("{} signed in", payload.pseudo);

    Json(LoginResponse { token })
}
