use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Application::{apputils, gitutils};
use crate::{EmuSettings, Emulator, Game};
use crc32fast::Hasher;
use dioxus::fullstack::http::header::ValuesMut;
use dioxus::html::u::height;
use dioxus::prelude::*;
use rfd::MessageDialog;

#[component]
pub fn Games_Component(settings: Signal<EmuSettings>) -> Element {
    let mut value = use_signal(|| 5);
    let is_editable = use_signal(|| false);
    let game_buf = use_signal(|| Game::default());
    let emulators = settings.read().emulators.clone(); //Cloning isn't too bad because emulators hashmap shouldn't be very big

    rsx! {
        div {class:" bg-red-500 min-h-full flex flex-col",
            div {class:"flex-1 bg-blue-400",
                h1 { "Games" }
                input { r#type:"range", min:"3", max:"12", value:value(), oninput: move |event| {
                    value.set(event.value().parse::<i32>().unwrap());
                }}
            }
            div{ class:"flex-10 flex flex-wrap justify-start content-start m-3",
                {show_games(&settings.read(), value(), is_editable, game_buf)}
            }
        }
        if is_editable() {
            Edit_Component {emulators, is_editable, game_buf}
        }
    }
}

// Come back here after emulator !!!!!!!!!!!!!!!!!!!!!!
#[component]
fn Edit_Component(emulators: HashMap<String, Emulator>, mut is_editable: Signal<bool>, game_buf: Signal<Game>) -> Element {
    let mut emulator_option = String::new();
    rsx! {
        div { class:"absolute opacity-90 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-gray-300 size-full",
            div{ class: "absolute opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-orange-300 h-6/10 w-6/10 flex flex-col mt-1",
                "{game_buf.read().name}"
                select { class:"",
                    onchange: move |e| {
                        emulator_option = e.value();
                    },
                    option { disabled: true, selected: true, "Choose the emulator" }
                    for (key, val) in emulators.into_iter() {
                        option { "{key}" }
                    }
                }
                button { class:"", onclick: move |_| {is_editable.set(false);}, "Save and close"  }
                button { class:"", onclick: move |_| {is_editable.set(false);}, "Close without saving"  }
            }
        }
    }
}

fn show_games(settings: &EmuSettings, value: i32, mut is_editable: Signal<bool>, mut game_buf: Signal<Game>) -> Element {
    let s = settings.games.clone(); //Bad, I copy the entire games settings here
    rsx! {
        for (key, val) in s.into_iter() {
            button {
                key: "{key}",
                style: "height: {value * 2}rem; width: {value * 2}rem;",
                class:"bg-blue-500 group rounded-md m-1 relative",
                onclick: {
                    let name = val.name.clone();
                    move |_| println!("{}", name)
                },
                "{val.name} \n {val.emulator.get_name()}"

                button {
                    class:"bg-purple-300 absolute top-0 right-0 opacity-0 group-hover:opacity-100 p-1",
                    onclick: {
                        move |e| {
                            e.stop_propagation();
                            game_buf.set(val.clone());
                            is_editable.set(true);
                        }
                    },
                    "Edit"
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
