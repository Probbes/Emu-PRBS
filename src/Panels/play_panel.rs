use crate::Application::apputils;
use crate::EmuSettings;
use dioxus::prelude::*;
use rfd::MessageDialog;

#[component]
pub fn Play_Component(settings: Signal<EmuSettings>, log: Signal<String>) -> Element {
    let s = settings.read();
    let emulators = s.emulators.clone();

    rsx! {
        h1 { "Play" }
        for (key, val) in emulators {
            button {
                onclick: move |_| {
                    let path = &val.0;

                    match open::that(path) {
                        Ok(()) => println!("Opened '{}' successfully.", path),
                        Err(_err) => {
                           MessageDialog::new()
                            .set_title("Error")
                            .set_description("Error while opening the emulator. Please check your emulator settings inside the app.")
                            .set_buttons(rfd::MessageButtons::Ok)
                            .set_level(rfd::MessageLevel::Error)
                            .show();
                        }
                    }

                    apputils::add_repo_to_emu(settings, key.clone(), val.clone());
                },
                "{key}",
            }
        }
        button { onclick: move |_| {
                apputils::add_emu_to_repo(settings);
            } ,
            "Get Saves",
        }
    }
}
