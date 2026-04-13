use std::process::Command;

use crate::Application::apputils;
use crate::EmuSettings;
use dioxus::prelude::*;
use rfd::MessageDialog;

#[component]
pub fn Play_Component(settings: Signal<EmuSettings>) -> Element {
    let s = settings.read();
    let emulators = s.emulators.clone();

    rsx! {
        div {class:" bg-red-500 min-h-full flex flex-col",
            div {class:"flex-1 bg-blue-400",
                h1 { "Games" }
                input { r#type:"range", min:"1", max:"10",}
            }

            div { class: "flex-18 p-4 flex flex-wrap content-start",
                for (key, val) in emulators {
                        button {
                        class: "bg-purple-300 w-40 h-40 m-5 pb-10",
                        onclick: move |_| {
                            play(settings, &key, &val);
                        },
                        "{key}",
                    }
                }
            }
        }
    }
}

fn play(settings: Signal<EmuSettings>, key: &String, val: &(String, String)) {
    let path = &val.0;

    match apputils::add_repo_to_emu(settings, key.clone(), val.clone()) {
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
