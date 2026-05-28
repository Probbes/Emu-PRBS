//#![windows_subsystem = "windows"]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder, tao::window::Icon};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use std::{fs::File, io::BufReader, path::Path};

const MAIN_CSS: &str = include_str!("../assets/styling/main.css");
const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
static ICON: Asset = asset!("/assets/chrysocolle.png");

use Panels::{Cloud_Component, Emulators_Component, Games_Component, Settings_Component};
mod Panels;

mod Application;
use crate::{Application::apputils, Application::gitutils};
use Application::settings::*;

fn main() {
    let window_icon = apputils::create_icon();

    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Chrysocolle")
                    .with_window_icon(Some(window_icon)),
            ),
        )
        .launch(App);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Panel {
    Games,
    Emulators,
    Cloud,
    Settings,
}

#[component]
fn App() -> Element {
    let panel = use_signal(|| Panel::Games);

    let mut settings = use_signal(|| apputils::init_settings());

    let mut show_folder_warning = use_signal(|| false);

    use_effect(move || {
        if !settings.peek().project_folder.is_dir() {
            show_folder_warning.set(true);
        } else {
            //gitutils::git_pull(&*settings.peek());
            apputils::get_games(&mut *settings.write());
            apputils::get_save_path(&mut *settings.write());
        }
    });

    rsx! {
        style { "{MAIN_CSS}" }
        style { "{TAILWIND_CSS}" }
        document::Link { rel: "icon", href: asset!("/assets/icon.ico") }
        document::Title{"Chrysocolle"}

        div { class: "flex flex-row-reverse min-h-screen",
            div { class: "flex-4",
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

            div { class: " bg-linear-to-t from-neutral to-primary-950",
                div { class:"flex flex-col",
                    div { class: "flex items-center m-6 text-xl font-headline font-normal tracking-wide",
                        img {class:"[image-rendering:pixelated] w-16 h-16", src: ICON }
                        div {class:"ml-2",  "Chrysocolle"}
                    }
                    Options { panel, settings }
                }

            }
        }

        if *show_folder_warning.read() {
            div { class:"popup1",
                div{ class: "popup2",
                    "Folder of the app containing the settings file was not found. Please select the folder where settings.toml should be :"
                    button { class:"button", onclick: move |_| {
                        let picked_folder = apputils::pick_folder();
                        settings.write().project_folder = picked_folder.join("Chrysocolle");
                        apputils::add_toml(&settings.read());
                        apputils::create_app_space(Path::new(&picked_folder));
                        settings.write().git.set_directory(picked_folder.join("Chrysocolle").join("Saves"));
                        apputils::get_games(&mut *settings.write());
                        show_folder_warning.set(false)
                    },"..."}
                }
            }
        }
    }
}

#[component]
fn Options(mut panel: Signal<Panel>, settings: Signal<EmuSettings>) -> Element {
    let menu_items = [
        (Panel::Games, "Games"),
        (Panel::Emulators, "Emulators"),
        (Panel::Cloud, "Cloud"),
        (Panel::Settings, "Settings"),
    ];

    rsx! {
        div { class: "flex flex-col min-w-full gap-2",
            for (target_panel, label) in menu_items {
                button {
                    class: "optionbutton flex items-center justify-between ",
                    onclick: move |_| panel.set(target_panel),
                    "{label}"
                    if panel() == target_panel {
                        div { class: "w-1 h-6 bg-white rounded-full" }
                    }
                }
            }
            button { class: "optionbutton", onclick: move |_| quit(settings), "Quit" }
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
        match gitutils::add_emu_to_repo(&*settings.read()) {
            Ok(()) => println!("successful"),
            Err(err) => {
                apputils::show_error(&format!("Error adding to repository : {}", err));
            }
        }
        gitutils::git_push(&*settings.read());
        window.close();
    }
}
