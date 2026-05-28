use std::{path::PathBuf, process::Command};

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
    let is_editable = use_signal(|| false);
    let emu_buf = use_signal(|| EmuBuf::default());

    rsx! {
        div {class:"panel flex flex-col px-4",
            h1 {class: "", "Emulators" }
            div {class:"flex justify-between items-center",
                div {
                    "Size of items: "
                    input {class:"", r#type:"range", min:"5", max:"12", value:settings.read().emu_size, oninput: move |event| {
                        settings.write().emu_size = event.value().parse::<u8>().unwrap_or(5);
                        apputils::add_toml(&*settings.peek());
                    }}
                }

                div {class:"",
                    button {class:"button ", onclick: move |_| {
                        add_emulator(is_editable, emu_buf);
                    }, "+ Add Emulator"  }
                }
            }


            div{ class:"flex flex-wrap justify-start content-start m-3",
                    {show_emulators(settings, is_editable, emu_buf)}
            }
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
    let mut key = use_signal(|| emu_buf.read().key.clone());

    //input var
    let mut selected_type = use_signal(|| match &*emulator.read() {
        Emulator::RetroArch { .. } => EmuType::RetroArch,
        Emulator::Other { .. } => EmuType::Other,
        Emulator::New(_) => EmuType::New,
    });
    let mut checked = use_signal(|| emulator.peek().get_fullscreen().to_owned());
    let mut text = use_signal(|| emulator.peek().get_name().to_string());

    rsx! {
        div { class:"popup1 overflow-hidden",
            div{ class: "popup2  overflow-y-auto custom-scrollbar",
                div {
                    //Name
                    input {class:"text-3xl font-headline font-extrabold tracking-wide",
                        value: "{text}",
                        oninput: move |e| {
                            emulator.write().set_name(e.value());
                            text.set(e.value())
                        }
                    }
                    //Emulator Type
                    div {class:"radio",
                        div {
                            input {r#type: "radio", name: "emu", checked: selected_type() == EmuType::RetroArch,
                            oninput: move |_| {
                                selected_type.set(EmuType::RetroArch);
                                emulator.set(Emulator::RetroArch { name: key(), path: PathBuf::new(), default_fullscreen: false, core: PathBuf::new(), save_path: PathBuf::new(), git_path: PathBuf::new()});
                            }
                            }
                            label { "RetroArch" }
                        }
                        div {
                            input {r#type: "radio", name: "emu", checked: selected_type() == EmuType::Other,
                            oninput: move |_| {
                                selected_type.set(EmuType::Other);
                                emulator.set(Emulator::Other { name: key(), path: PathBuf::new(), default_fullscreen: false , save_path: PathBuf::new(), git_path: PathBuf::new()});
                            }
                            }
                            label { "Other Emulator" }
                        }
                    }

                    //Core for retroarch emu
                    if EmuType::RetroArch == selected_type.read().clone() {
                        div {class:"flex flex-col my-2",
                            "Core file (.dll) :"
                            button {class: "buttonpick",
                            onclick: move |_| {emulator.write().set_core(apputils::pick_file());},
                            "{emulator.read().get_core().to_string_lossy()}"  }
                        }
                    }

                    //Path
                    div {class:"flex flex-col my-2",
                        "Path of the emulator : "
                        button {class: "buttonpick",
                        onclick: move |_| {emulator.write().set_path(apputils::pick_file());},
                        "{emulator.read().get_path().to_string_lossy()}"  }
                    }

                    //SavePath
                    div {class:"flex flex-col my-2",
                        "Save path of the emulator: "
                        button {class: "buttonpick",
                        onclick: move |_| {emulator.write().set_save_path(apputils::pick_folder());},
                        "{emulator.read().get_save_path().to_string_lossy()}"  }
                    }

                    //GitPath
                    div {class:"",
                        "Git path of the emulator:  "
                        select {
                            onchange: move |e| {
                            emulator.write().set_git_path(PathBuf::from(e.value()));
                            },
                            for (key, val) in settings.read().git.get_save_dir().iter() {
                                if val == &emulator.read().get_git_path() {
                                    option {value: val.to_str(), selected: true, "{key}"  }
                                }
                                else {
                                    option {value: val.to_str(), "{key}"  }
                                }

                            }
                        }
                    }

                    //Fullscreen
                    div {class: "flex my-2",
                        label { "Start at fullscreen" }
                        input { r#type: "checkbox", checked,
                            oninput: move |_| {
                                emulator.write().set_fullscreen(!checked());
                                checked.set(!checked())
                            }
                        }
                    }
                }


                div {class:"flex flex-nowrap justify-center",
                    //Quit
                    button { class:"button m-2", onclick: move |_| {
                        if key() != emulator.peek().get_name() {
                            settings.write().emulators.remove(&key());
                            println!("checking games");
                            for game in settings.write().games.iter_mut() {
                                if game.1.emulator == key() {
                                    game.1.emulator = emulator.peek().get_name().to_string();
                                }
                            }
                            key.set(emulator.peek().get_name().to_string());
                        }
                        settings.write().emulators.insert(key(), emulator());
                        apputils::add_toml(&settings.peek());
                        is_editable.set(false);
                    }, "Save and close"  }

                    button { class: "button m-2", onclick: move |_| {is_editable.set(false);}, "Close without saving"  }

                    button { class: "button m-2", onclick: move |_| {
                        for game in settings.write().games.iter_mut() {
                            if game.1.emulator == key() {
                                game.1.emulator = String::new();
                            }
                        }
                        settings.write().emulators.remove(&key());
                        apputils::add_toml(&settings.peek());
                        is_editable.set(false);
                    }, "Delete emulator"  }
                }

            }
        }
    }
}

fn show_emulators(settings: Signal<EmuSettings>, is_editable: Signal<bool>, emu_buf: Signal<EmuBuf>) -> Element {
    let emulator = &settings.read().emulators;
    rsx! {
        for (key, _val) in emulator.into_iter() {
            {emulator_button(settings, key.clone(), is_editable, emu_buf)}
        }
    }
}

fn emulator_button(settings: Signal<EmuSettings>, key: String, mut is_editable: Signal<bool>, mut emu_buf: Signal<EmuBuf>) -> Element {
    let key2 = key.clone(); // Not sure about this cloning
    let value = settings.read().emu_size;
    rsx! {
        button {
            key: "{&key}",
            style: "height: {value * 2}rem; width: {value * 2}rem;",
            class:"buttoncard group relative",
            onclick: {
                move |_| play_emulator(&settings.read().emulators[&key])
            },div {style:"font-size: {value*4}px","{settings.read().emulators[&key].get_name()}"  }

            button {
                class:"buttonedit",
                onclick: {
                    move |e| {
                        e.stop_propagation();
                        let s = settings.read();
                        if let Some(emulator) = s.emulators.get(&key2) {
                            emu_buf.set(EmuBuf { emulator: emulator.clone(),key: key2.clone()});
                            is_editable.set(true);
                        }
                    }
                },
                "Edit"
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
}

fn play_emulator(emulator: &Emulator) {
    match emulator {
        Emulator::RetroArch { path, core, .. } => {
            let status = Command::new(path)
                .arg("-L")
                .arg(core)
                //.arg(rom_path)
                //.arg("-f") // Optional: Start in Fullscreen
                .spawn();

            match status {
                Ok(_) => println!("RetroArch launched successfully!"),
                Err(e) => eprintln!("Failed to launch RetroArch: {}", e),
            }
        }
        Emulator::Other { path, .. } => {
            let status = Command::new(path).arg("-L").spawn();
            match status {
                Ok(_) => println!("Emulator launched successfully!"),
                Err(e) => eprintln!("Failed to launch Emulator: {}", e),
            }
        }
        _ => println!("tant pis"),
    }
}
