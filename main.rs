use std::fs::File;
use std::io::BufReader;

use rodio::{source::Source, Decoder, OutputStream};
use sysinfo::{System, SystemExt};

fn print_temperature() {
    let mut system = System::new();

    // First we update all information of our system struct.
    system.refresh_all();

    // Then let's print the temperature of the different components:
    for component in system.get_components_list() {
        println!("{:?}", component);
    }
}

// Play the engine sound
fn play_sound() {
    let audio_file: &str = "Engine Rev Inside Car-SoundBible.com-1161104884.mp3";

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(audio_file).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    print_temperature();
    play_sound();
}
