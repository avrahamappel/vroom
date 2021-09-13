use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::process::{exit, Command};

use rodio::source::{Repeat, Source, Stoppable};
use rodio::Decoder;
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

/**
 * Start the sound and return the ability to stop it.
 */
fn play_sound() -> Stoppable<Repeat<Decoder<BufReader<File>>>> {
    let audio_file: &str = "Engine Rev Inside Car-SoundBible.com-1161104884.mp3";

    // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(audio_file).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples())
    source.repeat_infinite().stoppable()
}

fn execute_command(cmd: String) -> Option<i32> {
    println!("{}", cmd);

    let mut cmd_parts = cmd.split(" ");

    let cmd = cmd_parts.next()?;

    let mut command = Command::new(cmd);

    for arg in cmd_parts {
        command.arg(arg);
    }

    let status = command.status();

    if let Ok(exit) = status {
        exit.code()
    } else {
        None
    }
}

fn main() {
    let cmd = args().skip(1).collect::<Vec<String>>().join(" ");

    print_temperature();

    let mut sound = play_sound();

    let code = execute_command(cmd).unwrap();

    sound.stop();

    exit(code);
}
