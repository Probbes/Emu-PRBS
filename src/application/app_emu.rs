use std::collections::HashMap;

use eframe::egui;
use egui::{FontId, RichText, Sense, TextStyle, UiBuilder};

use crate::application::app_toml;

pub struct MyApp {
    pub centralpanel: Panel,
    pub emulators: HashMap<String, (String, String)>,
}

pub enum Panel {
    Play,
    Settings,
    Github,
    Locations,
}

fn set_styles(ctx: &egui::Context) {
    //app styles
    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(40.0, egui::FontFamily::Monospace),
    );

    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(30.0, egui::FontFamily::Monospace),
    );

    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(30.0, egui::FontFamily::Monospace),
    );

    ctx.set_style(style);
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            centralpanel: Panel::Play,
            emulators: app_toml::init_hashmap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_styles(ctx);

        match self.centralpanel {
            Panel::Play => {
                self.make_play_panel(ctx);
            }
            Panel::Settings => {
                self.make_settings_panel(ctx);
            }

            Panel::Github => {
                self.make_github_panel(ctx);
            }
            Panel::Locations => {
                self.make_locations_panel(ctx);
            }
        }

        egui::SidePanel::right("right_panel")
            .min_width(32.0)
            .default_width(500.0)
            .show(ctx, |ui| {
                if ui.button("PLAY").clicked() {
                    self.centralpanel = Panel::Play;
                };
                if ui.button("SETTINGS").clicked() {
                    self.centralpanel = Panel::Settings;
                };
                if ui.button("GITHUB").clicked() {
                    self.centralpanel = Panel::Github;
                };
                if ui.button("INSTALLED GAMES").clicked() {
                    self.centralpanel = Panel::Locations;
                };
            });
    }
}

impl MyApp {
    //----------Play Panel-------------------
    fn make_play_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Play");
            for (key, emulator) in self.emulators.iter() {
                let response = ui
                    .scope_builder(
                        UiBuilder::new()
                            .id_salt("interactive button")
                            .sense(Sense::click()),
                        |ui| {
                            let response = ui.response();
                            let visuals = ui.style().interact(&response);

                            egui::Frame::canvas(ui.style())
                                .fill(visuals.bg_fill.gamma_multiply(0.3))
                                .stroke(visuals.bg_stroke)
                                .inner_margin(ui.spacing().menu_margin)
                                .show(ui, |ui| {
                                    ui.set_width(200.0);
                                    ui.set_height(200.0);

                                    ui.vertical_centered(|ui| {
                                        ui.add(
                                            egui::Label::new(
                                                RichText::new(key)
                                                    .color(egui::Color32::WHITE)
                                                    .size(32.0),
                                            )
                                            .selectable(false),
                                        )
                                    });
                                    if emulator.1 != "" {
                                        ui.add(
                                            egui::Image::from_uri(format!("file://{}", emulator.1))
                                                .fit_to_exact_size(egui::vec2(96.0, 96.0))
                                                .maintain_aspect_ratio(true),
                                        );
                                    }
                                });
                        },
                    )
                    .response;
                if response.clicked() {
                    let path = &emulator.0;

                    match open::that(path) {
                        Ok(()) => println!("Opened '{}' successfully.", path),
                        Err(err) => {
                            panic!("An error occurred when opening '{}': {}", path, err)
                        }
                    }
                }
            }
        })
    }

    //----------Settings Panel-------------------
    fn make_settings_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let style = ui.style_mut();
            let text_size = 25.0;
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(text_size, egui::FontFamily::Proportional),
            );

            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(text_size, egui::FontFamily::Proportional),
            );

            /*let mut to_remove: Option<String> = None; //Remove emulator
            let mut change_hash: Option<String> = None; //Change emulators hashmap value */

            ui.heading("Settings");
            ui.label("List of emulators : ");
            for (key, emulator) in self.emulators.iter_mut() {
                egui::Frame::default()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .corner_radius(5.0)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(key).size(text_size + 5.0));
                        ui.label("Binary Path : ");
                        if ui.button(&emulator.0).clicked() {
                            //Pick file -> add file to toml & HashMap BINARY
                        }
                        ui.label("Image Path : ");
                        if ui.button(&emulator.1).clicked() {
                            //Pick file -> add file to toml & HashMap IMG
                        }
                        //Remove Emulator button
                        if ui.button("Remove").clicked() {
                            //Remove emulator from toml & HashMap
                        }
                    });

                ui.add_space(20.0);
            }

            if ui.button("Add").clicked() {
                //Add empty emulator with name
            }

            //Remove the emulator after button click (after the iterator)
            /*             if let Some(key) = to_remove {
                self.emulators.remove(&key);

                let buffer = std::fs::read_to_string("settings.toml").expect("File not found");
                let mut doc = buffer
                    .parse::<toml_edit::DocumentMut>()
                    .expect("invalid doc");
                if let Some(table) = doc["emulators"].as_table_mut() {
                    table.remove(&key);
                }
                std::fs::write("settings.toml", doc.to_string()).expect("No settings.toml");
            }

            if let Some(key) = change_hash {
                if let Some(x) = self.emulators.get(&key) {
                    let y = &x.binary;
                }
            } */
        })
    }

    //----------Github Panel-------------------
    fn make_github_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Github");
        })
    }

    //----------Locations Panel-------------------
    fn make_locations_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Locations");
        })
    }
}
