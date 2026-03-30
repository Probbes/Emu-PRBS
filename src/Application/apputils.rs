use dioxus::{prelude::*, CapturedError};
use fs_utils::copy::copy_directory;
use rfd::FileDialog;
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::fs::{remove_dir_all, DirBuilder};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::EmuSettings;


//TODO!!! Sync emulators name from the cloud (git pull gives emulators folders, get those names in the emulators Hashmap)

pub fn init_settings() -> EmuSettings {
    let file = match fs::read_to_string("settings.toml") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("settings.toml not found. Creating default ...");

            fs::write("settings.toml", "").expect("Failed to create settings.toml");

            String::new()
        }
    };

    let settings = match toml::from_str::<EmuSettings>(&file) {
        Ok(parsed) => parsed,
        Err(_) => {
            eprintln!("Failed to parse settings, creating default settings ...");
            let default_settings = EmuSettings {
                ..Default::default()
            };

            let toml_string =
                toml::to_string(&default_settings).expect("Failed to serialize default settings");

            fs::write("settings.toml", &toml_string).expect("Failed to create settings.toml");
            default_settings
        }
    };

    settings
}

pub fn add_toml(settings: &EmuSettings) {
    let toml = toml::to_string(&settings).unwrap();

    let mut file = File::create("settings.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

pub fn pick_file() -> String {
    let files = FileDialog::new()
        .add_filter("*", &["*"])
        .set_directory("/")
        .pick_file();
    match files {
        Some(i) => i.into_os_string().into_string().unwrap(),
        None => {
            println!("Error with the file");
            String::new()
        }
    }
}

pub fn pick_folder() -> String {
    let files = FileDialog::new().set_directory("/").pick_folder();
    match files {
        Some(i) => i.into_os_string().into_string().unwrap(),
        None => {
            println!("Error with the folder");
            String::new()
        }
    }
}

pub fn git_pull(settings: Signal<EmuSettings>) {
    println!("GITPULL");
    let s = settings.read();
    //let mut log_add = String::new();
    //log_add.push_str("Cloning: \n");
    //log_add.push_str(&String::from_utf8_lossy(&output.stdout).to_string());

    let dir: String = if s.git.get_directory().is_empty() {
        "./".to_string()
    } else {
        s.git.get_directory().to_string().clone()
    };

    let repo_url = format!(
        "https://{}:{}@github.com/{}/{}.git",
        &s.git.get_username(),
        &s.git.get_token(),
        &s.git.get_username(),
        &s.git.get_repo()
    );
    let repo_path = format!("{}/{}/.git", dir, &s.git.get_repo());

    if Path::new(&repo_path).exists() == false {
        let confirm = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Git issue")
            .set_description(
                "No git repo at the specified directory.
            Do you want to clone ?",
            )
            .set_buttons(MessageButtons::YesNo)
            .show();
        if confirm == MessageDialogResult::Yes {
            git_clone(settings);
        }
    } else {
        let output = Command::new("git")
            .args(["pull", &repo_url])
            .current_dir(format!("{dir}/{}", &s.git.get_repo()))
            .output()
            .expect("failed");

        println!("{output:?}");
    }
}

pub fn git_push(settings: Signal<EmuSettings>) {
    let s = settings.read();

    let dir = format!("{}/{}", &s.git.get_directory(), &s.git.get_repo());

    let repo_url = format!(
        "https://{}:{}@github.com/{}/{}.git",
        &s.git.get_username(),
        &s.git.get_token(),
        &s.git.get_username(),
        &s.git.get_repo()
    );

    let output = Command::new("git")
        .args(["add", "."])
        .current_dir(&dir)
        .output()
        .expect("failed");
    println!("{output:?}");

    let output = Command::new("git")
        .args(["commit", "-m", "commit"])
        .current_dir(&dir)
        .output()
        .expect("failed");
    println!("{output:?}");

    let output = Command::new("git")
        .args(["push", &repo_url])
        .current_dir(dir)
        .output()
        .expect("failed");
    println!("{output:?}");
}

pub fn git_clone(settings: Signal<EmuSettings>) {
    let s = settings.read();

    let dir: String = if s.git.get_directory().is_empty() {
        "./".to_string()
    } else {
        s.git.get_directory().to_string().clone()
    };

    let repo_url = format!(
        "https://{}:{}@github.com/{}/{}.git",
        &s.git.get_username(),
        &s.git.get_token(),
        &s.git.get_username(),
        &s.git.get_repo()
    );

    let output = Command::new("git")
        .args(["clone", &repo_url])
        .current_dir(dir)
        .output()
        .expect("failed");
    println!("{output:?}");
}

pub fn add_repo_to_emu(
    settings: Signal<EmuSettings>,
    key: String,
    val: (String, String),
) -> Result<(), CapturedError> {
    let s = settings.read();
    let git_path = Path::new(s.git.get_directory())
        .join(s.git.get_repo())
        .join(&key);

    match DirBuilder::new().create(&git_path) {
        Ok(()) => println!("Folder doesn't exists, creating..."),
        Err(_) => println!("Folder already exists"),
    }

    let destination = Path::new(&val.1);
    if let Some(parent) = destination.parent() {
        if let Some(source_name) = destination.file_name() {
            overwrite_folder(&git_path.join(source_name), parent)?;
        }
    }

    Ok(())
}

pub fn add_emu_to_repo(settings: Signal<EmuSettings>) -> Result<(), CapturedError> {
    let s = settings.read();
    let emulators = s.emulators.clone();

    for (key, val) in emulators {
        //Example : C:/Users/Probb/Desktop/test/repo/key
        let git_path = Path::new(s.git.get_directory())
            .join(s.git.get_repo())
            .join(&key);

        match DirBuilder::new().create(&git_path) {
            Ok(()) => println!("Folder doesn't exists, creating..."),
            Err(_) => println!("Folder already exists"),
        }

        let destination = Path::new(&val.1); //'C:\RetroArch-Win64\saves'
        overwrite_folder(&destination.to_path_buf(), &git_path)?
    }

    Ok(())
}

pub fn overwrite_folder(source: &Path, destination: &Path) -> Result<(), CapturedError> {
    println!("Validating 6");
    // Ensure source has a valid folder name
    let name = source
        .file_name()
        .ok_or_else(|| CapturedError::msg("Source has no valid folder name"))?;

    // Prevent empty or suspicious names
    if name == OsStr::new("") {
        return Err(CapturedError::msg("Invalid source folder name"));
    }

    let target = destination.join(name);
    println!("Validating 7");
    // Safety checks before deletion
    validate_safe_to_delete(&target, destination)?;
    println!("Validating 8");
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
    println!("Validating 1");
    let target = target
        .canonicalize()
        .map_err(|_| CapturedError::msg("Invalid target path"))?;
    println!("Validating 2");
    let base = base
        .canonicalize()
        .map_err(|_| CapturedError::msg("Invalid base path"))?;
    println!("Validating 3");
    // Prevent deleting root directories
    if target.parent().is_none() {
        return Err(CapturedError::msg("Refusing to delete root directory"));
    }
    println!("Validating 4");
    // Ensure target is inside the destination directory
    if !target.starts_with(&base) {
        return Err(CapturedError::msg(
            "Target is outside destination directory",
        ));
    }
    println!("Validating 5");
    // Prevent deleting the base directory itself
    if target == base {
        return Err(CapturedError::msg("Refusing to delete destination root"));
    }

    Ok(())
}
