use crate::types::MusicState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_get_state(
    State(music_state): State<Arc<Mutex<MusicState>>>,
) -> impl IntoResponse {
    let music_state = music_state.lock().await;

    (StatusCode::OK, Json(json!({"state": *music_state})))
}
