use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Application::apputils, EmuSettings};

#[component]
pub fn Github_Component(settings: Signal<EmuSettings>) -> Element {
    let s = settings.read();
    let log = use_signal(|| String::new());

    rsx! {
        h1 { "Github" }
         div { "Repository: "
            input {
                r#type: "text",
                value: s.git.repo.as_str(),
                oninput: move |e| {
                    settings.with_mut(|s| s.git.repo = e.value());
                },
            }
        }
        div { "Token: "
            input {
                r#type: "text",
                size:"60",
                value: s.git.token.as_str(),
                oninput: move |e| {
                    settings.with_mut(|s| s.git.token = e.value());
                },
            }
        }
        div { "Username: "
            input {
                r#type: "text",
                value: s.git.username.as_str(),
                oninput: move |e| {
                    settings.with_mut(|s| s.git.username = e.value());
                },
            }
        }
        div { "Directory: "
            input {
                r#type: "text",
                size:"40",
                value: s.git.directory.as_str(),
                oninput: move |e| {
                    settings.with_mut(|s| s.git.directory = e.value());
                },
            }
            button { onclick: move |_| {settings.with_mut(|s| {s.git.directory = apputils::pick_folder();});}, "..." }
        }
        button { onclick: move |_| apply_settings(settings), "Apply Settings" }
        button { onclick: move |_| {
            apply_settings(settings);
            apputils::git_pull(settings);
        }, "Git Pull" }
        button { onclick: move |_| {
            apply_settings(settings);
            apputils::git_push(settings);}, "Git Push"
        }
        div {
            div {"Output: "}
            div {style: "white-space: pre-wrap;", "{log}"}
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct EmuGit {
    repo: String,
    token: String,
    username: String,
    directory: String,
}

impl EmuGit {
    pub fn get_directory(&self) -> &str {
        &self.directory
    }
    pub fn get_repo(&self) -> &str {
        &self.repo
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
}

fn apply_settings(settings: Signal<EmuSettings>) {
    apputils::add_toml(&settings.read());
}
