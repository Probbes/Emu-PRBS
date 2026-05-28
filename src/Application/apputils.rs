use crc32fast::Hasher;
use dioxus::{CapturedError, prelude::*};
use dioxus_desktop::tao::window::Icon;
use fs_extra::copy_items;
use rfd::FileDialog;
use rfd::MessageDialog;
use std::ffi::OsStr;
use std::fs::{self, DirBuilder, File};
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::EmuSettings;
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
    create_folder(&root.join("Saves"));
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
    let name = source
        .file_name()
        .ok_or_else(|| CapturedError::msg("Source has no valid folder name"))?;

    if name == OsStr::new("") {
        return Err(CapturedError::msg("Invalid source folder name"));
    }

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        ..Default::default()
    };

    copy_items(&[source], destination, &copy_options)?;

    Ok(())
}

pub fn get_save_path(settings: &mut EmuSettings) {
    println!("get saves");
    let path = settings.git.get_directory().join(settings.git.get_repo_name());
    let vec = get_all_folders(&path);
    for v in vec.iter() {
        let name = String::from(v.file_name().unwrap_or(OsStr::new("")).to_string_lossy());
        if name != String::from(".git") {
            settings.git.add_save_dir(name, v.to_path_buf());
        }
    }
}

pub fn get_games(settings: &mut EmuSettings) {
    println!("get games");
    let games = &mut settings.games;
    let path = PathBuf::from(&settings.project_folder).join("Games");
    let mut ids_vec: Vec<(u32, PathBuf)> = Vec::new();
    get_all_ids(path, &mut ids_vec);

    for (key, path) in ids_vec.iter() {
        games.entry(*key).or_insert(Game {
            name: path.file_prefix().unwrap_or_default().to_string_lossy().into_owned(),
            extension: path.extension().unwrap_or_default().to_string_lossy().into_owned(),
            path: path.clone(),
            fullscreen: false,
            emulator: String::new(),
        });
    }

    add_toml(settings);
}

//Recursive function that get the id of all game files inside a folder
fn get_all_ids(path: PathBuf, vec: &mut Vec<(u32, PathBuf)>) {
    match std::fs::read_dir(path) {
        Ok(v) => {
            for folder_entry in v {
                if let Ok(e) = folder_entry {
                    if let Ok(i) = e.file_type() {
                        if i.is_dir() {
                            get_all_ids(PathBuf::from(e.path()), vec);
                        } else if i.is_file() {
                            let file = File::open(e.path()).unwrap();
                            let finali = get_id(file);
                            vec.push((finali, e.path()));
                            //todo : remove if file not anymore (but keep save)
                        }
                    }
                }
            }
        }
        Err(err) => show_error(&format!("Error while opening folder for games scanning : {}", err)),
    };
}

fn get_id(file: File) -> u32 {
    let mut reader = BufReader::new(file).take(8 * 1024 * 1024);
    let mut hasher = Hasher::new();
    let mut buffer = [0; 8192];

    while let Ok(count) = reader.read(&mut buffer) {
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    hasher.finalize()
}

pub fn get_all_folders(path: &PathBuf) -> Vec<PathBuf> {
    let mut vec: Vec<PathBuf> = Vec::new();
    match std::fs::read_dir(path) {
        Ok(v) => {
            for folder_entry in v {
                if let Ok(e) = folder_entry {
                    if let Ok(file_type) = e.file_type() {
                        if file_type.is_dir() {
                            vec.push(e.path());
                        }
                    }
                }
            }
        }
        Err(err) => show_error(&format!("Error while opening folder for dir scanning : {}", err)),
    };
    vec
}

pub fn create_icon() -> Icon {
    let icon_bytes = include_bytes!("../../assets/chrysocolle.png");
    let img = image::load_from_memory_with_format(icon_bytes, image::ImageFormat::Png)
        .expect("Failed to decode embedded icon")
        .into_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create window icon")
}
