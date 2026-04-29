use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::Application::{apputils, gitutils};
use crate::{EmuSettings, Emulator, Game};
use dioxus::prelude::*;
use rfd::MessageDialog;

#[component]
pub fn Games_Component(settings: Signal<EmuSettings>) -> Element {
    let mut value = use_signal(|| 5);
    let is_editable = use_signal(|| false);
    let game_buf = use_signal(|| GameBuf::default());
    let emulators = settings.read().emulators.clone();
    let mut sortingmethod = use_signal(|| SortMethod::ByName);

    rsx! {
        div {class:" bg-red-500 min-h-full flex flex-col",
            div {class:"flex-1 bg-blue-400",
                h1 { "Games" }
                input { r#type:"range", min:"3", max:"12", value:value(), oninput: move |event| {
                    value.set(event.value().parse::<i32>().unwrap());
                }}
                select {class:"", value: "name", onchange: move |e| {
                    match e.value().as_str() {
                        "name" => sortingmethod.set(SortMethod::ByName),
                        "extension" => sortingmethod.set(SortMethod::ByExtension),
                        "folder" => sortingmethod.set(SortMethod::ByFolder),
                        _ => sortingmethod.set(SortMethod::ByName),
                    }
                },
                    option {value: "name", "Name"  }
                    option {value: "extension", "Extension"  }
                    option {value: "folder", "Folder"  }
                }
            }
            div{ class:"flex-10 flex flex-wrap justify-start content-start m-3",
                {show_games(settings, value(), is_editable, game_buf, sortingmethod)}
            }
        }
        if is_editable() {
            Edit_Component {settings, emulators, is_editable, game_buf}
        }
    }
}

#[derive(Default, Clone, PartialEq)]
struct GameBuf {
    game: Game,
    key: u32,
}

#[component]
fn Edit_Component(
    settings: Signal<EmuSettings>,
    emulators: HashMap<String, Emulator>,
    mut is_editable: Signal<bool>,
    game_buf: Signal<GameBuf>,
) -> Element {
    let mut emulator_option = use_signal(|| game_buf.read().game.emulator.clone());
    let game_key = game_buf.read().key;
    let mut game = use_signal(|| game_buf.read().game.clone());
    rsx! {
        div { class:"absolute opacity-90 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-gray-300 size-full",
            div{ class: "absolute opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-orange-300 h-6/10 w-6/10 flex flex-col mt-1",
                "{game.peek().name}"
                select { class:"",
                    value: emulator_option,
                    onchange: move |e| {
                        emulator_option.set(e.value());
                        game.write().emulator = e.value();
                    },
                    option { disabled: true, value: "", "Choose the emulator" }
                    for (key, _val) in emulators.into_iter() {
                        option { value: "{key}", "{key}" }
                    }
                }
                button { class:"", onclick: move |_| {
                    settings.write().games.insert(game_key, game());
                    apputils::add_toml(&settings.peek());
                    println!("{:?}", game.clone());
                    is_editable.set(false);}, "Save and close"
                }
                button { class:"", onclick: move |_| {is_editable.set(false);}, "Close without saving"  }
            }
        }
    }
}

enum SortMethod {
    ByName,
    ByExtension,
    ByFolder,
}

fn show_games(
    settings: Signal<EmuSettings>,
    value: i32,
    is_editable: Signal<bool>,
    game_buf: Signal<GameBuf>,
    sorting_method: Signal<SortMethod>,
) -> Element {
    let raw_keys: Vec<u32> = settings.read().games.keys().copied().collect();

    let vec = match *sorting_method.read() {
        SortMethod::ByName => sort_by_name(settings, raw_keys),
        SortMethod::ByExtension => sort_by_extension(settings, raw_keys),
        SortMethod::ByFolder => sort_by_folder(settings, raw_keys),
    };

    //Should it shows games not present in folder but present in toml ?
    rsx! {
        for key in vec {
            { game_button(settings, value, is_editable, game_buf, key) }
        }
    }
}

fn sort_by_name(settings: Signal<EmuSettings>, mut vec: Vec<u32>) -> Vec<u32> {
    let settings_read = settings.read();

    vec.sort_by(|a, b| {
        let name_a = settings_read.games.get(a).map(|g| g.name.to_lowercase()).unwrap_or_default();
        let name_b = settings_read.games.get(b).map(|g| g.name.to_lowercase()).unwrap_or_default();

        name_a.cmp(&name_b)
    });

    vec
}

fn sort_by_extension(settings: Signal<EmuSettings>, mut vec: Vec<u32>) -> Vec<u32> {
    let settings_read = settings.read();

    vec.sort_by(|a, b| {
        let name_a = settings_read.games.get(a).map(|g| g.extension.to_lowercase()).unwrap_or_default();
        let name_b = settings_read.games.get(b).map(|g| g.extension.to_lowercase()).unwrap_or_default();

        name_a.cmp(&name_b)
    });

    vec
}

//todo
fn sort_by_folder(settings: Signal<EmuSettings>, mut vec: Vec<u32>) -> Vec<u32> {
    let settings_read = settings.read();

    vec.sort_by(|a, b| {
        let name_a = settings_read.games.get(a).map(|g| g.extension.to_lowercase()).unwrap_or_default();
        let name_b = settings_read.games.get(b).map(|g| g.extension.to_lowercase()).unwrap_or_default();

        name_a.cmp(&name_b)
    });

    vec
}

fn game_button(
    settings: Signal<EmuSettings>,
    value: i32,
    mut is_editable: Signal<bool>,
    mut game_buf: Signal<GameBuf>,
    key: u32,
) -> Element {
    let s = settings.read();
    let game = &s.games[&key];
    let name = game.name.clone();
    let emulator = game.emulator.clone();
    let extension = game.extension.clone();

    rsx! {
        button {
            key: "{key}",
            style: "height: {value * 2}rem; width: {value * 2}rem;",
            class: "bg-blue-500 group rounded-md m-1 relative",

            onclick: move |_| {
                let s = settings.read();
                if let Some(game) = s.games.get(&key) {
                    play(settings, game.clone());
                }
            },
            div {class:"flex flex-col justify-evenly h-full",
                div {class:"text-base", "{name}"}
                div {class:"text-sm", "{emulator}"} }


            button {
                class: "bg-purple-300 absolute top-0 right-0 opacity-0 group-hover:opacity-100 p-1",
                onclick: move |e| {
                    e.stop_propagation();
                    let s = settings.read();
                    if let Some(game) = s.games.get(&key) {
                        game_buf.set(GameBuf { game: game.clone(), key });
                        is_editable.set(true);
                    }
                },
                "Edit"
            }
            div {class:"bg-orange-300 absolute inset-s-0 top-0", "{extension}"  }
        }
    }
}

fn play(settings: Signal<EmuSettings>, val: Game) {
    let game_path = &val.path;
    match gitutils::add_repo_to_emu(&*settings.read(), &val.emulator) {
        Ok(()) => {
            launch_game(settings, &val.emulator, game_path);
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
}

fn launch_game(settings: Signal<EmuSettings>, emulator_name: &String, game_path: &PathBuf) {
    if let Some(emulator) = settings.read().emulators.get(emulator_name) {
        match emulator {
            Emulator::RetroArch { path, core, .. } => {
                let status = Command::new(path)
                    .arg("-L")
                    .arg(core) // Load the specific core
                    .arg(game_path) // Load the game
                    .arg("-f") // Optional: Start in Fullscreen
                    .spawn(); // .spawn() lets your launcher stay open

                if let Err(e) = status {
                    apputils::show_error(&format!("Error while launching game : {}", e));
                }
            }
            Emulator::Other { path, .. } => {
                let status = Command::new(path)
                    .arg(game_path) // Load the game
                    .arg("-f") // doesn't seem to do anything !!!!!!!!!!!!!!!!!!
                    .spawn(); // .spawn() lets your launcher stay open

                if let Err(e) = status {
                    apputils::show_error(&format!("Error while launching game : {}", e));
                }
            }
            Emulator::New(_) => {
                apputils::show_error(&format!("Error while launching game : No Emulator found"));
            }
        }
    }
}
