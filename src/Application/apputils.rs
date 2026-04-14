use dioxus::{CapturedError, prelude::*};
use fs_utils::copy::copy_directory;
use rfd::FileDialog;
use rfd::MessageDialog;
use std::ffi::OsStr;
use std::fs::remove_dir_all;
use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::path::Path;

use crate::EmuSettings;

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

pub fn pick_file() -> String {
    let files = FileDialog::new().add_filter("*", &["*"]).set_directory("/").pick_file();
    match files {
        Some(i) => i.into_os_string().into_string().unwrap_or(String::new()),
        None => {
            println!("Error with the file");
            String::new()
        }
    }
}

pub fn pick_folder() -> String {
    let files = FileDialog::new().set_directory("/").pick_folder();
    match files {
        Some(i) => i.into_os_string().into_string().unwrap_or(String::new()),
        None => {
            println!("Error with the folder");
            String::new()
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
