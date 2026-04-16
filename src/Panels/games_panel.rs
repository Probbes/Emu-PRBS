use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Application::gitutils;
use crate::{EmuSettings, Game};
use dioxus::prelude::*;
use rfd::MessageDialog;

#[component]
pub fn Games_Component(settings: Signal<EmuSettings>) -> Element {
    let games = settings.read().games.clone();

    rsx! {
        div {class:" bg-red-500 min-h-full flex flex-col",
            div {class:"flex-1 bg-blue-400",
                h1 { "Games" }
                input { r#type:"range", min:"1", max:"10",}
            }

            button {class:"flex-1 bg-red-400",
                onclick: move |_| test(&*settings.read()),"Test"
            }
        }
    }
}

fn test(settings: &EmuSettings) {
    let metadata = fs::metadata(
        PathBuf::from(&settings.project_folder)
            .join("Chrysocolle")
            .join("Games")
            .join("GBA")
            .join("Advance Wars 2 - Black Hole Rising (USA, Australia).gba"),
    )
    .unwrap();

    println!("{:?}", metadata);
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
