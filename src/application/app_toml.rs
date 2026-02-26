use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Settings {
    username: String,
    volume: u8,
    emulators: HashMap<String, (String, String)>,
}

pub fn init_hashmap() -> HashMap<String, (String, String)> {
    let mut h = HashMap::new();
    let file: String = fs::read_to_string("settings.toml").unwrap();
    let settings: Settings = toml::from_str(&file).unwrap();

    for (key, val) in settings.emulators.iter() {
        println!("name = {}, binary = {} & img = {}", key, val.0, val.1);
        h.insert(key.to_owned(), val.to_owned());
    }

    return h;
}

pub fn add_toml() {
    let mut emulators = HashMap::new();
    emulators.insert(
        String::from("Azahar"),
        (String::from("binary"), String::from("img")),
    );
    let settings = Settings {
        username: String::from("Probbes"),
        volume: 8,
        emulators,
    };

    let toml = toml::to_string(&settings).unwrap();

    let mut file = File::create("settings.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

pub fn remove_toml() {}

pub fn add_hash() {}

pub fn remove_hash() {}

/* pub fn init_hashmap() -> HashMap<String, app_emu::Emulator> {
    let mut h = HashMap::new();
    let buffer = std::fs::read_to_string("settings.toml").expect("File not found");
    let doc = buffer.parse::<DocumentMut>().expect("invalid doc");

    if let Some(emulators) = doc["emulators"].as_table() {
        for (name, item) in emulators.iter() {
            let table = item.as_inline_table().expect("expected inline table");

            let binary = table
                .get("binary")
                .and_then(|v| v.as_str())
                .expect("binary not found")
                .to_string();

            let img = table
                .get("img")
                .and_then(|v| v.as_str())
                .expect("img not found")
                .to_string();

            h.insert(name.to_string(), app_emu::Emulator { binary, img });
        }
    }
    h
}

pub fn pick_file() -> String {
    let files = FileDialog::new()
        .add_filter("*", &["*"])
        .set_directory("/")
        .pick_file();

    if let Some(s) = files {
        s.display().to_string()
    } else {
        String::from("")
    }
}

pub fn write_to_toml_emulators(emulator: &str, category: &str, new_value: &str) {
    let buffer = std::fs::read_to_string("settings.toml").expect("File not found");
    let mut doc = buffer.parse::<DocumentMut>().expect("invalid doc");
    doc["emulators"][emulator][category] = value(new_value);
    std::fs::write("settings.toml", doc.to_string()).expect("No settings.toml");
} */
