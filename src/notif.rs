use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub fn play_bell_sound() {
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error initializing audio stream: {}", e);
            return;
        }
    };

    let file = match File::open("bellsound.mp3") {
        Ok(file) => file,
        Err(_) => {
            println!("Notification sound file not found");
            return;
        }
    };

    let source = match Decoder::new(BufReader::new(file)) {
        Ok(source) => source,
        Err(e) => {
            println!("Error decoding audio: {}", e);
            return;
        }
    };

    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => (),
        Err(e) => println!("Error playing audio: {}", e),
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
}
