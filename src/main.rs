#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod application;
use application::app_emu;
use application::app_toml;

fn main() -> eframe::Result<()> {
    //Main with app creation
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]) // width, height in pixels
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Emu PRBS",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); //To see images (see cargo.toml for supported img format)
            Ok(Box::new(app_emu::MyApp {
                ..Default::default()
            }))
        }),
    )
}
