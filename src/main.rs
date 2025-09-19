use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let bpm = 30;

    loop {
        let file = File::open("resources/lost-in-the-world-bass-kick_F#_minor.wav")?;
        let buf_reader = BufReader::new(file);
        let source = Decoder::new(buf_reader)?;
        
        sink.append(source);
        
        time::sleep(Duration::from_secs(60 / bpm)).await;
    }
}
