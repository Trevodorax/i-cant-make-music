use crate::types::MusicState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_delete_tracks(
    State(music_state): State<Arc<Mutex<MusicState>>>,
    Path(track_id): Path<String>,
) -> impl IntoResponse {
    let mut music_state = music_state.lock().await;

    let initial_len = music_state.tracks.len();
    music_state.tracks.retain(|track| track.id != track_id);

    if music_state.tracks.len() < initial_len {
        (StatusCode::NO_CONTENT, Json(json!({"state": *music_state})))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"message": "Track not found"})),
        )
    }
}
