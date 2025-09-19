mod rest_api;
mod types;

use crate::rest_api::init_router;
use crate::types::MusicState;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

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
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum_server::bind(addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}

async fn play_music(music_state: Arc<Mutex<MusicState>>) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Create multiple sinks for overlapping playback
    let mut sinks = Vec::new();
    for _ in 0..8 {
        sinks.push(Sink::try_new(&stream_handle)?);
    }
    let mut sink_index = 0;

    // Load audio data once
    let file = File::open("resources/lost-in-the-world-bass-kick_F#_minor.wav")?;
    let mut audio_data = Vec::new();
    std::io::copy(&mut BufReader::new(file), &mut audio_data)?;

    loop {
        let cursor = Cursor::new(audio_data.clone());
        let source = Decoder::new(cursor)?;

        // Use next sink in rotation
        sinks[sink_index].append(source);
        sink_index = (sink_index + 1) % sinks.len();

        let interval_ms = 60_000 / music_state.lock().await.bpm;
        time::sleep(Duration::from_millis(interval_ms as u64)).await;
    }
}
