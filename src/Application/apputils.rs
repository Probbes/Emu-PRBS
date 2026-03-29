use dioxus::prelude::*;
use fs_utils::copy::copy_directory;
use rfd::FileDialog;
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{
    fs::{remove_dir_all, DirBuilder},
    path::PathBuf,
};

use crate::EmuSettings;

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

pub fn add_repo_to_emu(settings: Signal<EmuSettings>, key: String, val: (String, String)) {
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
            overwrite_folder(&git_path.join(source_name), parent);
        }
    }
}

pub fn add_emu_to_repo(settings: Signal<EmuSettings>) {
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
        overwrite_folder(&destination.to_path_buf(), &git_path);
    }
}

pub fn overwrite_folder(source: &PathBuf, destination: &Path) {
    if copy_directory(source, destination).is_ok() {
        println!(
            "Copied {} inside {:?}",
            source.to_string_lossy(),
            destination
        );
    }

    // If copy failed, try removing existing folder
    let name = match source.file_name() {
        Some(n) => n,
        None => return,
    };

    if let Err(err) = remove_dir_all(destination.join(name)) {
        println!("{err}");
    }

    // Try copy again
    match copy_directory(source, destination) {
        Ok(_) => println!(
            "Copied {} inside {:?}",
            source.to_string_lossy(),
            destination
        ),
        Err(err) => println!("{err}"),
    }
}
