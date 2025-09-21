use serde::Serialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Serialize)]
pub struct MusicState {
    pub bpm: u16,
    pub tracks: Vec<Track>,
    pub notes_per_beat: usize,
}

impl MusicState {
    pub fn new() -> MusicState {
        MusicState {
            bpm: 80,
            tracks: vec![
                Track::new(
                    "resources/lost-in-the-world-bass-kick_F#_minor.wav",
                    vec![Some(0), None, Some(6), None],
                ),
                Track::new(
                    "resources/long-piano-note-C.wav",
                    vec![
                        Some(0),
                        Some(5),
                        None,
                        Some(15),
                        None,
                        Some(5),
                        Some(10),
                        Some(15),
                        Some(0),
                        Some(5),
                        None,
                        Some(15),
                        None,
                        Some(5),
                        Some(10),
                        Some(17),
                    ],
                ),
            ],
            notes_per_beat: 4,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Track {
    pub sound: Vec<u8>,
    pub notes: Vec<Option<i32>>,
}

impl Track {
    pub fn new(sound_path: &str, notes: Vec<Option<i32>>) -> Track {
        // TODO: handle the potential errors with a Result instead
        let file = File::open(sound_path).expect("Unable to open sound file");
        let mut audio_data = Vec::new();
        std::io::copy(&mut BufReader::new(file), &mut audio_data)
            .expect("Unable to read audio data");

        Track {
            sound: audio_data,
            notes,
        }
    }
}
