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
    let args: Vec<String> = std::env::args().collect();
    let devicepath = String::from("/dev/input/") + &args[1];
    let sound_path = args[2].clone() + &String::from("/") + &String::from("config.json");

    let d = Device::new_from_path(devicepath).unwrap();
    println!("device is {}", sound_path);
    let conf = read_user_from_file(sound_path).unwrap();
    let default_file = conf
        .get("defines")
        .unwrap()
        .get("30")
        .unwrap()
        .to_string()
        .trim()
        .trim_matches('"')
        .to_string();
    println!("default file is {}", default_file);
    loop {
        let ev = d
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
            .map(|val| val.1);
        match ev {
            Ok(ev) => match ev.event_type().unwrap() {
                evdev_rs::enums::EventType::EV_KEY => {
                    if ev.value == 1 {
                        let key_code = ev.event_code;
                        let key_int = event_code_to_int(&key_code);

                        let file = conf
                            .get("defines")
                            .and_then(|defines| defines.get(key_int.1.to_string()))
                            .map_or_else(|| Value::String(default_file.clone()), |v| v.clone())
                            .to_string();
                        let key_file = file.trim().trim_matches('"').to_string();

                        println!("key file is {}", file);

                        let dir = args[2].clone() + &String::from("/") + &key_file;
                        sound::play_sound(dir, 100);
                    }
                }
                _ => {}
            },
            Err(_e) => {
                thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}
