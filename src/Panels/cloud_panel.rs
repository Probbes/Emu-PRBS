use dioxus::prelude::*;

use crate::{Application::apputils, Application::gitutils, EmuSettings};

#[component]
pub fn Cloud_Component(settings: Signal<EmuSettings>) -> Element {
    let s = settings.read();

    rsx! {
        div { class: "panel px-2",
            h1 {class:"", "Github" }
            div {class:"flex flex-col my-2",
                "Repository: "
                input {class:"buttonpick",
                    r#type: "text",
                    value: s.git.get_repo_name(),
                    oninput: move |e| {
                        settings.with_mut(|s| s.git.set_repo_name(e.value()));
                    },
                }
            }
            div {class:"flex flex-col my-2", "Directory: "
                button {class: "buttonpick", onclick: move |_| {settings.with_mut(|s| {s.git.set_directory( apputils::pick_folder());});}, "{s.git.get_directory().to_string_lossy().into_owned()}" }
            }
            div {class:"m-5",
                button {class:"button m-2", onclick: move |_| apply_settings(&*settings.read()), "Apply Settings" }
                button {class:"button m-2", onclick: move |_| {
                    apply_settings(&*settings.read());
                    gitutils::git_pull(&*settings.read());
                }, "Git Pull" }
                button {class:"button m-2", onclick: move |_| {
                    apply_settings(&*settings.read());
                    gitutils::git_push(&*settings.read());}, "Git Push"
                }
            }

        }
    }
}

fn apply_settings(settings: &EmuSettings) {
    apputils::add_toml(settings);
}
