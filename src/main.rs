//#![windows_subsystem = "windows"]

use dioxus::prelude::*;
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAIN_CSS: &str = include_str!("../assets/styling/main.css");
const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");

use Panels::{Emulators_Component, Github_Component, Play_Component, Settings_Component};
mod Panels;

mod Application;
use Application::apputils;

use crate::Panels::EmuGit;

fn main() {
    dioxus::launch(App);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Panel {
    Play,
    Settings,
    Github,
    Emulators,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
struct EmuSettings {
    username: String,
    volume: u8,
    emulators: HashMap<String, (String, String)>,
    git: EmuGit,
}

#[component]
fn App() -> Element {
    let panel = use_signal(|| Panel::Play);

    let settings = use_signal(|| apputils::init_settings());

    let log = use_signal(|| String::new());

    use_hook(|| {
        println!("This runs only ONCE at startup");
        apputils::git_pull(settings);
    });

    rsx! {
        style { "{MAIN_CSS}" }
        style { "{TAILWIND_CSS}" }

        div { class: "flex h-screen",
            div { class: "flex-3",
                match panel() {
                    Panel::Play => rsx! {
                        Play_Component { settings, log }
                    },
                    Panel::Github => rsx! {
                        Github_Component { settings }
                    },
                    Panel::Emulators => rsx! {
                        Emulators_Component { settings }
                    },
                    Panel::Settings => rsx! {
                        Settings_Component { settings }
                    },
                }
            }

            div {class:"log",
            "HELLO ! {log}"
            }

            div { class: "flex-1",
                Options { panel, settings }
            }
        }
    }
}

#[component]
fn Options(panel: Signal<Panel>, settings: Signal<EmuSettings>) -> Element {
    rsx! {

        div { class: "flex flex-col gap-2 p-4 b",

            button { onclick: move |_| panel.set(Panel::Play),"Play"}

            button { onclick: move |_| panel.set(Panel::Settings), "Settings" }

            button { onclick: move |_| panel.set(Panel::Github), "Github" }

            button { onclick: move |_| panel.set(Panel::Emulators), "Emulators" }

            button { onclick: move |_| quit(settings), "Quit" }
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
        apputils::add_emu_to_repo(settings);
        apputils::git_push(settings);
    }
}
