//#![windows_subsystem = "windows"]

use dioxus::prelude::*;
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MAIN_CSS: &str = include_str!("../assets/styling/main.css");
const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
static ICON: Asset = asset!("/assets/chrysocolle.png");

use Panels::{Cloud_Component, Emulators_Component, Games_Component, Settings_Component};
mod Panels;

mod Application;
use crate::{Application::apputils, Application::gitutils};

use crate::Panels::EmuGit;

fn main() {
    dioxus::launch(App);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Panel {
    Games,
    Emulators,
    Cloud,
    Settings,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
struct EmuSettings {
    username: String,
    project_folder: PathBuf,
    games: HashMap<String, Game>,
    emulators: HashMap<String, Emulator>,
    git: EmuGit,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
struct Game {
    name: String,
    path: PathBuf,
    fullscreen: bool,
    emulator: String,
}
#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
enum Emulator {
    RetroArch {
        name: String,
        path: PathBuf,
        default_fullscreen: bool,
        core: PathBuf,
    },
    Other {
        name: String,
        path: PathBuf,
        default_fullscreen: bool,
    },
    #[default]
    None,
}

#[component]
fn App() -> Element {
    let panel = use_signal(|| Panel::Games);

    let mut settings = use_signal(|| apputils::init_settings());

    let mut show_folder_warning = use_signal(|| false);

    use_hook(|| {
        if !settings.read().project_folder.is_dir() {
            show_folder_warning.set(true);
        } else {
            //apputils::git_pull(settings); !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        }
    });

    rsx! {
        style { "{MAIN_CSS}" }
        style { "{TAILWIND_CSS}" }
        document::Title{"Chrysocolle"}
        document::Link{rel: "icon", href: asset!("/assets/favicon.ico")}
        div { class: "flex flex-row-reverse bg-green-500 min-h-screen",
            div { class: "flex-3",
                match panel() {
                    Panel::Games => rsx! {
                        Games_Component { settings }
                    },
                    Panel::Cloud => rsx! {
                        Cloud_Component { settings }
                    },
                    Panel::Emulators => rsx! {
                        Emulators_Component { settings }
                    },
                    Panel::Settings => rsx! {
                        Settings_Component { settings }
                    },
                }
            }

            div { class: "flex-1",
                div { class:"flex flex-col",
                    div { class: "flex justify-center-safe items-center my-6",
                        img {class:"h-12 mx-5 object-cover", src: ICON }
                        "Chrysocolle"
                    }
                    div { class: " bg-cyan-500 my-2",
                        Options { panel, settings }}
                }

            }
        }

        if *show_folder_warning.read() {
            div { class:"absolute opacity-90 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-gray-300 size-full",
                div{ class: "absolute opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-orange-300 h-6/10 w-6/10",
                    "Folder of the app containing the settings file was not found. Please select the folder where settings.toml should be :"
                    button { class:"", onclick: move |_| {
                        let picked_folder = apputils::pick_folder();
                        settings.write().project_folder = picked_folder.clone();
                        apputils::add_toml(&settings.read());
                        apputils::create_app_space(Path::new(&picked_folder));
                        show_folder_warning.set(false)
                    },"..."}
                }
            }
        }
    }
}

#[component]
fn Options(panel: Signal<Panel>, settings: Signal<EmuSettings>) -> Element {
    rsx! {

        div { class: "flex flex-col gap-1 min-w-full",

            button { class:"optionbutton", onclick: move |_| panel.set(Panel::Games),"Games"}

            button { class:"optionbutton", onclick: move |_| panel.set(Panel::Emulators), "Emulators" }

            button { class:"optionbutton", onclick: move |_| panel.set(Panel::Cloud), "Cloud" }

            button { class:"optionbutton", onclick: move |_| panel.set(Panel::Settings), "Settings" }

            button { class:"optionbutton", onclick: move |_| quit(settings), "Quit" }
        }
    }
}

fn quit(settings: Signal<EmuSettings>) {
    let window = dioxus_desktop::window();
    println!("Quitting Client");
    let confirm = MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title("Quit Application")
        .set_description("Are you sure you want to quit?")
        .set_buttons(MessageButtons::YesNo)
        .show();

    if confirm == MessageDialogResult::Yes {
        window.close();
        match gitutils::add_emu_to_repo(&*settings.read()) {
            Ok(()) => println!("successful"),
            Err(err) => {
                apputils::show_error(&format!("Error adding to repository : {}", err));
            }
        }
        gitutils::git_push(&*settings.read());
    }
}
