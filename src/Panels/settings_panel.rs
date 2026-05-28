use dioxus::prelude::*;

use crate::{Application::apputils, EmuSettings};

#[component]
pub fn Settings_Component(settings: Signal<EmuSettings>) -> Element {
    let mut checked = use_signal(|| settings.read().pure_name);
    rsx! {
        div {class:"panel px-2",
            h1 {class:"", "Settings" }
            div {class:"flex flex-col",
                "Username:"
                input {
                    class:"buttonpick",
                    r#type: "text",
                    value: settings.peek().username.as_str(),
                    oninput: move |e| {
                        //settings.with_mut(|s| s.username = e.value());
                        settings.write().username = e.value();
                    },
                }
            }
            div {class:"flex flex-col",
                "Folder Path: "
                button {class: "buttonpick", onclick: move |_| {settings.write().project_folder = apputils::pick_folder();}, "{settings.read().project_folder.to_string_lossy()}"  }
            }

            div {class: "flex my-2",
                label { "Remove () and [] from names" }
                input { r#type: "checkbox", checked,
                    oninput: move |_| {
                        settings.write().pure_name = !checked();
                        checked.set(!checked())
                    }
                }
            }

            div {class:"m-5",
                button {class:"button", onclick: move |_| apply_settings(settings), "Apply Settings" }
            }
        }
    }
}

fn apply_settings(settings: Signal<EmuSettings>) {
    let s = settings.read();
    apputils::add_toml(&s);
}
