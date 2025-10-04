use crate::types::{MusicState, Track};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct PutTrackRequest {
    pub sound_path: Option<String>,
    pub notes: Option<Vec<Option<i32>>>,
}

pub async fn handle_put_tracks(
    State(music_state): State<Arc<Mutex<MusicState>>>,
    Path(track_id): Path<String>,
    Json(request): Json<PutTrackRequest>,
) -> impl IntoResponse {
    let mut music_state = music_state.lock().await;

    if let Some(track) = music_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(notes) = request.notes {
            track.notes = notes;
        }

        if let Some(sound_path) = request.sound_path {
            if sound_path.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Sound path cannot be empty."
                    })),
                );
            }

            match Track::new(&sound_path, track.notes.clone()) {
                Ok(new_track) => {
                    track.sound = new_track.sound;
                }
                Err(err) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "message": format!("Failed to load sound file: {}", err)
                        })),
                    );
                }
            }
        }

        (StatusCode::OK, Json(json!({"state": *music_state})))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"message": "Track not found"})),
        )
    }
}
