mod rest_api;
mod types;

use crate::rest_api::init_router;
use crate::types::MusicState;
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_state = Arc::new(Mutex::new(MusicState::new()));

    // === Play music according to music state === //
    let music_state_clone = music_state.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Err(e) = rt.block_on(play_music(music_state_clone)) {
            eprintln!("Music playback error: {}", e);
        }
    });

    // === Setup router for music state changes === //
    let router = init_router(music_state.clone()).await;
    // uncomment this to allow any CORS (for development)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let router = router.layer(cors);

    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum_server::bind(addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}

async fn play_music(music_state: Arc<Mutex<MusicState>>) -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let mut current_note_position: usize = 1;

    loop {
        let music_state = music_state.lock().await.clone();

        let max_notes_per_beat: usize = music_state
            .tracks
            .iter()
            .map(|track| track.notes.len())
            .max()
            .unwrap_or(0);

        for track in &music_state.tracks {
            if let Some(note) = track.notes[(current_note_position - 1) % track.notes.len()] {
                let cursor = Cursor::new(track.sound.clone());
                let source = Decoder::new(cursor)?;
                let shifted_source = source.speed(2_f32.powf(note as f32 / 12.0));

                let sink = Sink::connect_new(&stream_handle.mixer());
                sink.append(shifted_source);
                sink.detach();
            }
        }

        let interval_ms = 60_000 / music_state.bpm / music_state.notes_per_beat as u16;
        time::sleep(Duration::from_millis(interval_ms as u64)).await;
        current_note_position = if current_note_position < max_notes_per_beat {
            current_note_position + 1
        } else {
            1
        }
    }
}