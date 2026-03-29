use dioxus::prelude::*;

use crate::{
    Application::apputils::{self},
    EmuSettings,
};

#[component]
pub fn Emulators_Component(settings: Signal<EmuSettings>) -> Element {
    let mut edit_emulators = use_signal(|| false);

    let mut key_change = use_signal(|| String::new());
    let mut val_change = use_signal(|| (String::new(), String::new()));

    let emulators = settings.read().emulators.clone();

    rsx! {
        h1 { "Emulators" }
        div { class:"flex-col",
            div { class:"flex-3",
                for (key, val) in emulators {
                    h2 { "{key}" }
                    div {
                        button {
                            onclick: move |_| {
                                *key_change.write() = key.clone();
                                *val_change.write() = val.clone();
                                edit_emulators.set(true);
                            },
                            "Edit"
                        }
                    }
                }
                div {
                    button {
                        onclick: move |_| {
                            settings.write().emulators.insert(String::from("Name"), (String::from("Binary"),String::from("Save")));
                            apputils::add_toml(&settings.read());
                        },
                        "Add"
                    }
                }
            }
            div { class:"flex-1",if *edit_emulators.read() { Emulators_Edit_Component { settings, key_change, val_change, edit_emulators } } }
        }
    }
}

//Can edit each emulator with the use of another tab.
#[component]
pub fn Emulators_Edit_Component(
    settings: Signal<EmuSettings>,
    key_change: Signal<String>,
    val_change: Signal<(String, String)>,
    edit_emulators: Signal<bool>,
) -> Element {
    let mut k = use_signal(|| key_change.read().clone());
    let mut v1 = use_signal(|| val_change.read().0.clone());
    let mut v2 = use_signal(|| val_change.read().1.clone());
    rsx! {
        div { "EDIT" }

        div { class:"flex-col",
            div { class: "flex-1", "Name: " input {  r#type: "text", value: k, oninput: move |e| { k.set(e.value()); }, } }

            div { class: "flex-1", "Binary: "
                input {  r#type: "text", value: v1, oninput: move |e| { v1.set(e.value()); }, }
                button { onclick: move |_| {v1.set(apputils::pick_file());}, "..." }
            }

             div { class: "flex-1", "Save Folder: "
                input {  r#type: "text", value: v2, oninput: move |e| { v2.set(e.value()); }, }
                button { onclick: move |_| {v2.set(apputils::pick_folder());}, "..." }
            }
        }

        div {
            button {
                onclick: move |_| {
                    settings.write().emulators.remove(&key_change.read().clone());
                    settings.write().emulators.insert(k.read().clone(), (v1.read().clone(),v2.read().clone()));
                    apputils::add_toml(&settings.read());
                    edit_emulators.set(!edit_emulators());
                },
                "Apply"
            }
        }
        div {
            button {
                onclick: move |_| {
                    settings.write().emulators.remove(&key_change.read().clone());
                    apputils::add_toml(&settings.read());
                    edit_emulators.set(!edit_emulators());
                },
                "Delete"
            }
        }
        div {
            button {
                onclick: move |_| {
                    edit_emulators.set(!edit_emulators());
                },
                "Close"
            }
        }
    }
}
