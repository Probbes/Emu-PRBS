use dioxus::prelude::*;

use crate::{
    Application::apputils::{self},
    EmuSettings, Emulator,
};

#[component]
pub fn Emulators_Component(settings: Signal<EmuSettings>) -> Element {
    let emulators = settings.read().emulators.clone();

    rsx! {
        h1 { "Emulators" }

    }
}
