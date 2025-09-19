use crate::types::MusicState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]

pub struct PutBpmRequest {
    pub bpm: u16,
}

pub async fn handle_put_bpm(
    State(music_state): State<Arc<Mutex<MusicState>>>,
    Json(request): Json<PutBpmRequest>,
) -> impl IntoResponse {
    if request.bpm == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Invalid bpm."
            })),
        );
    }

    let mut music_state = music_state.lock().await;

    music_state.bpm = request.bpm;

    (StatusCode::OK, Json(json!({"state": *music_state})))
}
