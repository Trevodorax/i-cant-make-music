use serde::Serialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Serialize)]
pub struct MusicState {
    pub bpm: u16,
    pub track: Track,
}

impl MusicState {
    pub fn new() -> MusicState {
        MusicState {
            bpm: 100,
            track: Track::new("resources/lost-in-the-world-bass-kick_F#_minor.wav"),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Track {
    pub sound: Vec<u8>,
    pub notes: [Option<i32>; 4],
}

impl Track {
    pub fn new(sound_path: &str) -> Track {
        // TODO: handle the potential errors with a Result instead
        let file = File::open(sound_path).expect("Unable to open sound file");
        let mut audio_data = Vec::new();
        std::io::copy(&mut BufReader::new(file), &mut audio_data)
            .expect("Unable to read audio data");

        Track {
            sound: audio_data,
            notes: [Some(0), Some(6), Some(24), None],
        }
    }
}
