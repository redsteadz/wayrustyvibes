mod play_sound;

use crate::play_sound::sound;
use evdev_rs::util::event_code_to_int;
use evdev_rs::Device;
use evdev_rs::ReadFlag;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;

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
    let d = Device::new_from_path("/dev/input/event4").unwrap();
    let conf = read_user_from_file("config.json").unwrap();
    let mut key_pressed: i32 = 0;
    let mut key_file: String = "".to_owned();
    loop {
        let ev = d
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
            .map(|val| val.1);
        match ev {
            Ok(ev) => {
                let key_code = ev.event_code;
                let key_int = event_code_to_int(&key_code);

                let file = conf
                    .get("defines")
                    .and_then(|defines| defines.get(key_int.1.to_string()))
                    .map_or_else(|| Value::String("a.wav".to_owned()), |v| v.clone())
                    .to_string();
                let key_file = file.trim().trim_matches('"').to_string();

                match ev.event_type().unwrap() {
                    // evdev_rs::enums::EventType::EV_MSC => {
                    //     key_pressed = ev.value;
                    // }
                    evdev_rs::enums::EventType::EV_KEY => {
                        if ev.value == 1 {
                            println!("key {} pressed {} {} ", "bruh", key_int.1, key_code);
                            let dir = String::from("nk-cream/") + &key_file;
                            sound::play_sound(dir, 100);
                        }
                    }
                    _ => {}
                }
            }
            Err(_e) => {
                thread::sleep(std::time::Duration::from_millis(50));
                // println!("error {}", _e);
            }
        }
    }
}
