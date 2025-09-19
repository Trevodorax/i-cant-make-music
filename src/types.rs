use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct MusicState {
    pub bpm: u16,
}

impl MusicState {
    pub fn new() -> MusicState {
        MusicState { bpm: 60 }
    }
}
