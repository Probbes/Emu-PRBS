use crc32fast::Hasher;
use dioxus::{CapturedError, prelude::*};
use fs_utils::copy::copy_directory;
use rfd::FileDialog;
use rfd::MessageDialog;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::remove_dir_all;
use std::fs::{self, DirBuilder, File};
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::EmuSettings;
use crate::Emulator;
use crate::Game;

//TODO!!! Sync emulators name from the cloud (git pull gives emulators folders, get those names in the emulators Hashmap)

pub fn show_error(msg: &str) {
    eprintln!("{}", msg);

    MessageDialog::new()
        .set_title("Error")
        .set_description(msg)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd::MessageLevel::Error)
        .show();
}

pub fn init_settings() -> EmuSettings {
    let file = match fs::read_to_string("settings.toml") {
        Ok(value) => value,
        Err(_err) => {
            eprintln!("settings.toml not found. Creating default ...");
            match fs::write("settings.toml", "") {
                Ok(()) => println!("File created successfully"),
                Err(err) => {
                    show_error(&format!("Failed to create settings.toml : {}", err));
                    return EmuSettings::default(); //exit function if can't create settings file
                }
            };
            String::new()
        }
    };

    let settings = match toml::from_str::<EmuSettings>(&file) {
        Ok(value) => value,
        Err(_err) => {
            eprintln!("Failed to parse settings, creating default settings and writing on the file ...");
            let default_settings = EmuSettings::default();
            add_toml(&default_settings);
            default_settings
        }
    };
    settings
}

pub fn create_app_space(path: &Path) {
    let root = &path.join("Chrysocolle");
    create_folder(root);
    create_folder(&root.join("Games"));
}

pub fn add_toml(settings: &EmuSettings) {
    let toml = match toml::to_string(&settings) {
        Ok(value) => value,
        Err(err) => {
            show_error(&format!("Error while Serializing settings : {}", err));
            return;
        }
    };

    match File::create("settings.toml") {
        Ok(mut value) => {
            if let Err(e) = value.write_all(toml.as_bytes()) {
                show_error(&format!("Error while writing data to settings.toml : {}", e));
            }
        }
        Err(err) => {
            show_error(&format!("Error while creating settings.toml : {}", err));
        }
    };
}

pub fn pick_file() -> PathBuf {
    let files = FileDialog::new().add_filter("*", &["*"]).set_directory("/").pick_file();
    match files {
        Some(i) => i,
        None => {
            println!("Error with the file");
            PathBuf::new()
        }
    }
}

pub fn pick_folder() -> PathBuf {
    let files = FileDialog::new().set_directory("/").pick_folder();
    match files {
        Some(i) => i,
        None => {
            println!("Error with the folder");
            PathBuf::new()
        }
    }
}

pub fn create_folder(destination: &Path) {
    if let Err(err) = DirBuilder::new().recursive(true).create(destination) {
        show_error(&format!("Error creating folder : {}", err));
    }
}

pub fn overwrite_folder(source: &Path, destination: &Path) -> Result<(), CapturedError> {
    // Ensure source has a valid folder name
    let name = source
        .file_name()
        .ok_or_else(|| CapturedError::msg("Source has no valid folder name"))?;

    // Prevent empty or suspicious names
    if name == OsStr::new("") {
        return Err(CapturedError::msg("Invalid source folder name"));
    }

    let target = destination.join(name);

    // Safety checks before deletion
    validate_safe_to_delete(&target, destination)?;

    // Try copy first
    if let Err(_) = copy_directory(source, destination) {
        // Only delete if target exists and is a directory
        if target.exists() {
            remove_dir_all(&target)?;
        }

        // Retry copy
        copy_directory(source, destination)?;
    }

    Ok(())
}

fn validate_safe_to_delete(target: &Path, base: &Path) -> Result<(), CapturedError> {
    let target = target.canonicalize().map_err(|_| CapturedError::msg("Invalid target path"))?;

    let base = base.canonicalize().map_err(|_| CapturedError::msg("Invalid base path"))?;

    // Prevent deleting root directories
    if target.parent().is_none() {
        return Err(CapturedError::msg("Refusing to delete root directory"));
    }

    // Ensure target is inside the destination directory
    if !target.starts_with(&base) {
        return Err(CapturedError::msg("Target is outside destination directory"));
    }

    // Prevent deleting the base directory itself
    if target == base {
        return Err(CapturedError::msg("Refusing to delete destination root"));
    }

    Ok(())
}

pub fn get_games(settings: &mut EmuSettings) {
    println!("GET GAMES");
    let games = &mut settings.games;
    let path = PathBuf::from(&settings.project_folder).join("Chrysocolle").join("Games");
    get_id(path, games);
    add_toml(settings);
}

//Recursive function that get the id of all game files inside a folder
fn get_id(path: PathBuf, games: &mut HashMap<u32, Game>) {
    match std::fs::read_dir(path) {
        Ok(v) => {
            for folder_entry in v {
                if let Ok(e) = folder_entry {
                    if let Ok(i) = e.file_type() {
                        if i.is_dir() {
                            get_id(PathBuf::from(e.path()), games);
                        } else if i.is_file() {
                            let file = File::open(e.path()).unwrap();
                            let mut reader = BufReader::new(file).take(8 * 1024 * 1024);
                            let mut hasher = Hasher::new();
                            let mut buffer = [0; 8192];

                            while let Ok(count) = reader.read(&mut buffer) {
                                if count == 0 {
                                    break;
                                }
                                hasher.update(&buffer[..count]);
                            }
                            let finali = hasher.finalize();
                            let name = e.file_name().to_string_lossy().into_owned();
                            games.insert(
                                finali,
                                Game {
                                    name: name,
                                    path: e.path(),
                                    fullscreen: false,
                                    emulator: Emulator::New(PathBuf::new()),
                                },
                            );
                        }
                    }
                }
            }
        }
        Err(err) => show_error(&format!("Error while opening folder for games scanning : {}", err)),
    };
}
