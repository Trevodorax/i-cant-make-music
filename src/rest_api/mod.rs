mod handle_put_bpm;

use crate::rest_api::handle_put_bpm::handle_put_bpm;
use crate::types::MusicState;
use axum::Router;
use axum::routing::put;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_router(music_state: Arc<Mutex<MusicState>>) -> Router {
    Router::new()
        .route("/bpm", put(handle_put_bpm))
        .with_state(music_state.clone())
}
