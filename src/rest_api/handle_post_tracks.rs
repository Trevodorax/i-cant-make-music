use crate::types::{MusicState, Track};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct PostTrackRequest {
    pub sound_path: String,
    pub notes: Vec<Option<i32>>,
}

pub async fn handle_post_tracks(
    State(music_state): State<Arc<Mutex<MusicState>>>,
    Json(request): Json<PostTrackRequest>,
) -> impl IntoResponse {
    if request.sound_path.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Sound path cannot be empty."
            })),
        );
    }

    let track = match Track::new(&request.sound_path, request.notes) {
        Ok(track) => track,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": format!("Failed to create track: {}", err)
                })),
            );
        }
    };

    let mut music_state = music_state.lock().await;
    music_state.tracks.push(track.clone());

    (
        StatusCode::CREATED,
        Json(json!({"created_track": track, "state": *music_state})),
    )
}
