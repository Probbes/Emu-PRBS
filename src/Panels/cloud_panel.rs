use std::path::PathBuf;

use dioxus::prelude::*;

use crate::{Application::apputils, Application::gitutils, EmuSettings};

#[component]
pub fn Cloud_Component(settings: Signal<EmuSettings>) -> Element {
    let s = settings.read();

    rsx! {
        h1 { "Github" }
         div { "Repository: "
            input {
                r#type: "text",
                value: s.git.get_repo_name(),
                oninput: move |e| {
                    settings.with_mut(|s| s.git.set_repo_name(e.value()));
                },
            }
        }

        div { "Directory: "
            input {
                r#type: "text",
                size:"40",
                value: s.git.get_directory().to_string_lossy().into_owned(),  //WTH is a Cow ?
                oninput: move |e| {
                    settings.with_mut(|s| s.git.set_directory(PathBuf::from(e.value())));
                },
            }
            button { onclick: move |_| {settings.with_mut(|s| {s.git.set_directory( apputils::pick_folder());});}, "..." }
        }
        button { onclick: move |_| apply_settings(&*settings.read()), "Apply Settings" }
        button { onclick: move |_| {
            apply_settings(&*settings.read());
            gitutils::git_pull(&*settings.read());
        }, "Git Pull" }
        button { onclick: move |_| {
            apply_settings(&*settings.read());
            gitutils::git_push(&*settings.read());}, "Git Push"
        }
        div {
            div {"Output: "}
        }
    }
}

fn apply_settings(settings: &EmuSettings) {
    apputils::add_toml(settings);
}
