use std::{ffi::OsStr, path::PathBuf};

use dioxus::prelude::*;

use crate::{
    Application::apputils::{self},
    EmuSettings, Emulator,
};

#[derive(Default, Clone, PartialEq)]
struct EmuBuf {
    emulator: Emulator,
    key: String,
}

#[component]
pub fn Emulators_Component(settings: Signal<EmuSettings>) -> Element {
    let mut value = use_signal(|| 5);
    let is_editable = use_signal(|| false);
    let emu_buf = use_signal(|| EmuBuf::default());

    rsx! {
        h1 { "Emulators" }
        input { r#type:"range", min:"3", max:"12", value:value(), oninput: move |event| {
            value.set(event.value().parse::<i32>().unwrap());
        }}
        div{ class:"flex-10 flex flex-wrap justify-start content-start m-3",
                button {class:"", onclick: move |_| {
                    add_emulator(is_editable, emu_buf);
                }, "Add"  }
                 {show_emulators(&*settings.read(), is_editable, emu_buf)}
        }

        if is_editable() {
            Edit_Component {settings, is_editable, emu_buf}
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum EmuType {
    RetroArch,
    Other,
    New,
}

#[component]
fn Edit_Component(settings: Signal<EmuSettings>, is_editable: Signal<bool>, emu_buf: Signal<EmuBuf>) -> Element {
    let mut emulator = use_signal(|| emu_buf.peek().emulator.clone());
    let key = use_signal(|| emu_buf.read().key.clone());

    //input var
    let mut selected_type = use_signal(|| match &*emulator.read() {
        Emulator::RetroArch { .. } => EmuType::RetroArch,
        Emulator::Other { .. } => EmuType::Other,
        Emulator::New(_) => EmuType::New,
    });
    let mut checked = use_signal(|| emulator.peek().get_fullscreen().to_owned());
    let mut text = use_signal(|| emulator.peek().get_name().to_string());

    rsx! {
        div { class:"absolute opacity-90 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-gray-300 size-full",
            div{ class: "absolute opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-orange-300 h-6/10 w-6/10 flex flex-col",

                //Emulator Type
                div {
                    input {r#type: "radio", name: "emu", checked: selected_type() == EmuType::RetroArch,
                        oninput: move |_| {
                            selected_type.set(EmuType::RetroArch);
                            emulator.set(Emulator::RetroArch { name: key(), path: PathBuf::new(), default_fullscreen: false, core: PathBuf::new() });
                        }
                    }
                    label { "RetroArch" }
                }
                div {
                    input {r#type: "radio", name: "emu", checked: selected_type() == EmuType::Other,
                        oninput: move |_| {
                            selected_type.set(EmuType::Other);
                            emulator.set(Emulator::Other { name: key(), path: PathBuf::new(), default_fullscreen: false });
                        }
                    }
                    label { "Other Emulator" }
                }
                div {
                    input { r#type: "radio", name: "emu", oninput: move |_| {
                        selected_type.set(EmuType::New);
                        emulator.set(Emulator::New(PathBuf::new()));
                    }
                }
                    label { "No Emulator" }
                }

                //Core for retroarch emu
                if EmuType::RetroArch == selected_type.read().clone() {
                    div {class:"flex",
                        "Core file (.dll) : {emulator.peek().get_core().to_string_lossy()} : "
                        button {class: "", onclick: move |_| {emulator.write().set_core(apputils::pick_file());}, "..."  }
                    }
                }

                //Name
                input {value: "{text}",
                    oninput: move |e| {
                        emulator.write().set_name(e.value());
                        text.set(e.value())
                    }
                }

                //Path
                div {class:"flex",
                    "Path of the emulator : {emulator.peek().get_path().to_string_lossy()} : "
                    button {class: "", onclick: move |_| {emulator.write().set_path(apputils::pick_file());}, "..."  }
                }

                //Fullscreen
                div {class: "flex",
                    input { r#type: "checkbox", checked,
                        oninput: move |_| {
                            emulator.write().set_fullscreen(!checked());
                            checked.set(!checked())
                        }
                    }
                    label { "Start at fullscreen" }
                }

                //Quit
                button { class:"", onclick: move |_| {
                    if emu_buf.peek().key == String::from("New") {
                        settings.write().emulators.remove("New");
                    }
                    settings.write().emulators.insert(key(), emulator());
                    apputils::add_toml(&settings.peek());
                    is_editable.set(false);
                }, "Save and close"  }
                button { class: "", onclick: move |_| {is_editable.set(false);}, "Close without saving"  }
                button { class: "", onclick: move |_| {
                    settings.write().emulators.remove(&key());
                    apputils::add_toml(&settings.peek());
                    is_editable.set(false);
                }, "Delete emulator"  }
            }
        }
    }
}

//To redo, too many clone() -- The second move inside the button works because I clone inside the first
fn show_emulators(settings: &EmuSettings, mut is_editable: Signal<bool>, mut emu_buf: Signal<EmuBuf>) -> Element {
    let s = settings.emulators.clone(); //Bad, I copy the entire emulators settings here

    rsx! {
        for (key, val) in s.into_iter() {
            button {
                key: "{key}",
                class:"bg-blue-500 group size-60 rounded-md m-1 relative",
                onclick: {
                    let val = val.clone();
                    move |_| println!("{}", val.get_name())
                },"{val.get_name()}"



                button {
                    class:"bg-purple-300 absolute top-0 right-0 opacity-0 group-hover:opacity-100 p-1",
                    onclick: {
                        move |e| {
                            e.stop_propagation();
                            emu_buf.set(EmuBuf{emulator: val.clone(), key: key.clone()});
                            is_editable.set(true);
                        }
                    },
                    "Edit"
                }
            }
        }
    }
}

fn add_emulator(mut is_editable: Signal<bool>, mut emu_buf: Signal<EmuBuf>) {
    emu_buf.set(EmuBuf {
        emulator: Emulator::New(PathBuf::new()),
        key: String::from("New"),
    });
    is_editable.set(true);
    /*
    let emulators = &mut settings.emulators;
    let path = apputils::pick_file();
    let name = match path.file_prefix() {
        Some(n) => n,
        None => &OsStr::new(""),
    };
    let name = name.to_string_lossy().into_owned();
    emulators.insert(name, Emulator::New(path));
    */
}
