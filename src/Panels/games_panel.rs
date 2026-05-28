use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::Application::{apputils, gitutils};
use crate::{EmuSettings, Emulator, Game};
use dioxus::prelude::*;
use regex::Regex;
use rfd::MessageDialog;

#[component]
pub fn Games_Component(settings: Signal<EmuSettings>) -> Element {
    let is_editable = use_signal(|| false);
    let game_buf = use_signal(|| GameBuf::default());
    let emulators = settings.read().emulators.clone();
    let mut sortingmethod = use_signal(|| SortMethod::ByName);

    rsx! {
        div {class:"panel flex flex-col overflow-hidden",
            div {class:"flex-none px-4",
                h1 { "Games" }
                div {class:"flex items-center flex-wrap",
                    div {class:"flex",
                        "Size of items:"
                        input {class:"mx-1", r#type:"range", min:"5", max:"12", value:settings.read().game_size, oninput: move |event| {
                            settings.write().game_size = event.value().parse::<u8>().unwrap_or(5);
                            apputils::add_toml(&*settings.peek());
                        }}
                    }
                    div {class:"",
                        "Sorting Method: "
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
                            //option {value: "folder", "Folder"  }
                        }
                    }

                }
            }
            div{ class:"flex-1 overflow-y-auto custom-scrollbar flex flex-wrap justify-start content-start m-3",
                {show_games(settings, is_editable, game_buf, sortingmethod)}
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
    let v = &game_buf.read().game.emulator;
    rsx! {
        div { class:"popup1",
            div{ class: "popup2",
                div {
                    div {class:"text-3xl font-headline font-extrabold tracking-wide", "{game_buf.peek().game.name}" }
                    div {class:"flex flex-col m-4",
                        "Emulator"
                        select { class:"",
                            value: v.to_owned(),
                            onchange: move |e| {
                                game_buf.write().game.emulator = e.value();
                            },
                            option { disabled: true, value: "", "Choose the emulator" }
                            for (key, _val) in emulators.into_iter() {
                                option { value: "{key}",selected: &key == &game_buf.read().game.emulator, "{key}" }
                            }
                        }
                    }
                }
                div {class:"flex flex-nowrap justify-center",
                    button { class:"button m-2", onclick: move |_| {
                    let buf = game_buf.read();
                    let mut s = settings.write();

                    if let Some(existing_game) = s.games.get_mut(&buf.key) {
                        existing_game.emulator = buf.game.emulator.clone();
                    }
                    apputils::add_toml(&s);
                    is_editable.set(false);}, "Save and close"
                    }
                    button { class:"button m-2", onclick: move |_| {is_editable.set(false);}, "Close without saving"  }
                }

            }
        }
    }
}

#[derive(PartialEq)]
enum SortMethod {
    ByName,
    ByExtension,
    ByFolder,
}

fn show_games(
    settings: Signal<EmuSettings>,
    is_editable: Signal<bool>,
    game_buf: Signal<GameBuf>,
    sorting_method: Signal<SortMethod>,
) -> Element {
    let mut vec: Vec<u32> = settings.read().games.keys().copied().collect();

    match *sorting_method.read() {
        SortMethod::ByName => sort_by_name(settings, &mut vec),
        SortMethod::ByExtension => sort_by_extension(settings, &mut vec),
        SortMethod::ByFolder => sort_by_name(settings, &mut vec),
    };

    //Should it shows games not present in folder but present in toml ?
    rsx! {
        for key in vec {
            { game_button(settings, is_editable, game_buf, key) }
        }
    }
}

fn sort_by_name(settings: Signal<EmuSettings>, vec: &mut Vec<u32>) {
    let settings_read = settings.read();

    vec.sort_by(|a, b| {
        let name_a = settings_read.games.get(a).map(|g| g.name.to_lowercase()).unwrap_or_default();
        let name_b = settings_read.games.get(b).map(|g| g.name.to_lowercase()).unwrap_or_default();

        name_a.cmp(&name_b)
    });
}

fn sort_by_extension(settings: Signal<EmuSettings>, vec: &mut Vec<u32>) {
    let settings_read = settings.read();

    vec.sort_by(|a, b| {
        let name_a = settings_read.games.get(a).map(|g| g.extension.to_lowercase()).unwrap_or_default();
        let name_b = settings_read.games.get(b).map(|g| g.extension.to_lowercase()).unwrap_or_default();

        name_a.cmp(&name_b)
    });
}

//todo
// fn sort_by_folder(settings: Signal<EmuSettings>, vec: &mut Vec<u32>) {
//     let settings_read = settings.read();
//     let v = apputils::get_all_folders(&settings.read().project_folder.join("Games"));
//     for folder in v.into_iter() {
//         for game in vec {}
//     }
// }

fn game_button(settings: Signal<EmuSettings>, mut is_editable: Signal<bool>, mut game_buf: Signal<GameBuf>, key: u32) -> Element {
    let value = settings.read().game_size;
    let name = match settings.read().pure_name {
        true => {
            let re = Regex::new(r"\([^)]*\)|\[[^\]]*\]").unwrap();
            &re.replace_all(&settings.read().games[&key].name, "").into_owned()
        }
        false => &settings.read().games[&key].name,
    };
    rsx! {
        button {
            key: "{key}",
            style: "height: {value * 2}rem; width: {value * 2}rem;",
            class: "buttoncard group relative",

            onclick: move |_| {
                let s = settings.read();
                if let Some(game) = s.games.get(&key) {
                    play(settings, &game);
                }
            },
            div {class:"flex flex-col justify-evenly h-full",
                div {style:"font-size: {value*3}px", class:"", "{name}"}
                div {class:"bg-primary-900 p-1 rounded-md absolute top-0 right-0 text-xs  ", "{settings.read().games[&key].emulator}"}
            }
                div {class:"bg-primary-900 p-1 -mx-2 rounded-md absolute inset-s-0 top-0 text-xs", ".{settings.read().games[&key].extension}"}

            button {
                class: "buttonedit",
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
        }
    }
}

fn play(settings: Signal<EmuSettings>, val: &Game) {
    let game_path = &val.path;
    match gitutils::add_repo_to_emu(&*settings.read(), &val.emulator) {
        Ok(()) => {
            launch_game(settings, &val.emulator, game_path);
        }
        Err(err) => {
            MessageDialog::new()
                .set_title("Error add_repo_to_emu")
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
