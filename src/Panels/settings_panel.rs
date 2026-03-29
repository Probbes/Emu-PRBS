use dioxus::prelude::*;

use crate::{Application::apputils::add_toml, EmuSettings};

#[component]
pub fn Settings_Component(settings: Signal<EmuSettings>) -> Element {
    let s = settings.read();
    rsx! {
        h1 {"Settings" }
        div {
            input {
                r#type: "text",
                value: s.username.as_str(),
                oninput: move |e| {
                    settings.with_mut(|s| s.username = e.value());
                },
            }
        }
        button { onclick: move |_| apply_settings(settings), "Apply Settings" }
    }
}

fn apply_settings(settings: Signal<EmuSettings>) {
    let s = settings.read();
    add_toml(&s);
}
