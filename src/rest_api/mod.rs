mod handle_delete_tracks;
mod handle_get_sounds;
mod handle_get_state;
mod handle_post_tracks;
mod handle_put_bpm;
mod handle_put_tracks;

use crate::rest_api::handle_delete_tracks::handle_delete_tracks;
use crate::rest_api::handle_get_sounds::handle_get_sounds;
use crate::rest_api::handle_get_state::handle_get_state;
use crate::rest_api::handle_post_tracks::handle_post_tracks;
use crate::rest_api::handle_put_bpm::handle_put_bpm;
use crate::rest_api::handle_put_tracks::handle_put_tracks;
use crate::types::MusicState;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_router(music_state: Arc<Mutex<MusicState>>) -> Router {
    Router::new()
        .route("/bpm", put(handle_put_bpm))
        .route("/tracks", post(handle_post_tracks))
        .route("/tracks/{id}", put(handle_put_tracks))
        .route("/tracks/{id}", delete(handle_delete_tracks))
        .route("/state", get(handle_get_state))
        .route("/sounds", get(handle_get_sounds))
        .with_state(music_state.clone())
}
