use evdev::Key;
use evdev_rs::enums::EV_MSC;
use evdev_rs::Device;
use evdev_rs::ReadFlag;
use rodio_wav_fix::{source::Source, Decoder, OutputStream};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Value = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let mut d = Device::new_from_path("/dev/input/event4").unwrap();
    let conf = read_user_from_file("config.json").unwrap();
    let mut key_pressed: i32 = 0;
    let mut key_file: String = "".to_owned();
    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);
        match ev {
            Ok(ev) => {
                let key = Key::new(ev.value.try_into().unwrap());
                let file = conf
                    .get("defines")
                    .unwrap()
                    .get(ev.value.to_string())
                    .unwrap_or(&Value::String("".to_owned()))
                    .to_string();

                // if !file.is_empty() {
                //     println!("{:?} reading conf {} ", key, file);
                // }
                // println!(
                //     "Event: time {}.{}, ++++++++++++++++++++ {} +++++++++++++++ +++ value {}",
                //     ev.time.tv_sec,
                //     ev.time.tv_usec,
                //     ev.event_type()
                //         .map(|ev_type| format!("{}", ev_type))
                //         .unwrap_or("".to_owned()),
                //     ev.value,
                // );
                match ev.event_type().unwrap() {
                    evdev_rs::enums::EventType::EV_MSC => {
                        key_pressed = ev.value;
                        key_file = file.trim().trim_matches('"').to_string();
                        // println!(
                        //     "Event:+++ {} +++ value {}",
                        //     ev.event_type()
                        //         .map(|ev_type| format!("{}", ev_type))
                        //         .unwrap_or("".to_owned()),
                        //     ev.value,
                        // );
                    }
                    evdev_rs::enums::EventType::EV_KEY => {
                        if ev.value == 1 {
                            println!("key {} pressed {} ", key_pressed, key_file);
                            let dir = String::from("nk-cream/") + &key_file;
                            // let x = Command::new("aplay").arg(dir).output();
                            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                            // Load a sound from a file, using a path relative to Cargo.toml
                            let file = BufReader::new(File::open(dir).unwrap());
                            // Decode that sound file into a source
                            let source = Decoder::new(file).unwrap();
                            // Play the sound directly on the device
                            let _ = stream_handle.play_raw(source.convert_samples());
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => (),
        }
    }
}
