use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Application::gitutils;
use crate::{EmuSettings, Game};
use crc32fast::Hasher;
use dioxus::{prelude::*};
use rfd::MessageDialog;

#[component]
pub fn Games_Component(settings: Signal<EmuSettings>) -> Element {

    get_games(&*settings.read());

    rsx! {
        div {class:" bg-red-500 min-h-full flex flex-col",
            div {class:"flex-1 bg-blue-400",
                h1 { "Games" }
                input { r#type:"range", min:"1", max:"10",}
            }

            button {class:"flex-1 bg-red-400",
                onclick: move |_| show_games(&*settings.read()),"Test"
            }
        }
    }
}

fn show_games(settings: &EmuSettings) {
    for (key, val) in settings.games.iter() {
        println!("name : {} - id : {}", key, val.name);
    }
}

fn get_games(settings: &EmuSettings) {
    let path = PathBuf::from(&settings.project_folder)
            .join("Chrysocolle")
            .join("Games");
    let mut vec: Vec<(String, u32)> = Vec::new();
    get_id(path, &mut vec);

    for entry in vec.iter() {
        println!("name : {} - id : {}", entry.0, entry.1);
    }
    
    /* 
    let file = File::open(path)
            .join("GBA")
            .join("Fire Emblem.gba"),).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();
    let mut buffer = [0; 8192];

    while let Ok(count) = reader.read(&mut buffer) {
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
    }
    let finali = hasher.finalize();
    println!("{:08x}", finali);
    */
}

fn get_id (path: PathBuf, vec: &mut Vec<(String, u32)>) {
    let folder_entries=  std::fs::read_dir(path).unwrap();
    for folder_entry in folder_entries {
        if let Ok(e) = folder_entry {
            if let Ok(i) = e.file_type() {
                if i.is_dir() {
                    get_id(PathBuf::from(e.path()), vec);
                }
                else if i.is_file() {
                    let file = File::open(e.path()).unwrap();
                    let mut reader = BufReader::new(file).take(8 * 1024 * 1024);
                    let mut hasher = Hasher::new();
                    let mut buffer = [0; 8192];

                    while let Ok(count) = reader.read(&mut buffer) {
                        if count == 0 { break; }
                        hasher.update(&buffer[..count]);
                    }
                    let finali = hasher.finalize();
                    let name = e.file_name().to_string_lossy().into_owned();
                    vec.push((name, finali));
                }
            }
        }
    }
}

fn play(settings: Signal<EmuSettings>, key: &String, val: &Game) {
    let path = &val.path;

    /*

    match gitutils::add_repo_to_emu(settings, key.clone(), val.clone()) {
        Ok(()) => {
            let status = Command::new(path).spawn();

            match status {
                Ok(_) => println!("Game launched successfully!"),
                Err(e) => eprintln!("Failed to launch RetroArch: {}", e),
            }
        }
        Err(err) => {
            MessageDialog::new()
                .set_title("Error")
                .set_description(err.to_string())
                .set_buttons(rfd::MessageButtons::Ok)
                .set_level(rfd::MessageLevel::Error)
                .show();
        }
    }
     */
}

/* fn launch_retroarch(rom_path: &str, core_name: &str) {
    // 1. Define your paths (In a real app, these might come from a config file)
    let retroarch_path = r"C:\RetroArch-Win64\retroarch.exe";
    let core_path = format!(r"C:\RetroArch-Win64\cores\{}.dll", core_name);

    // 2. Build the command
    let status = Command::new(retroarch_path)
        .arg("-L")
        .arg(&core_path) // Load the specific core
        .arg(rom_path) // Load the game
        .arg("-f") // Optional: Start in Fullscreen
        .spawn(); // .spawn() lets your launcher stay open

    match status {
        Ok(_) => println!("Game launched successfully!"),
        Err(e) => eprintln!("Failed to launch RetroArch: {}", e),
    }
} */
