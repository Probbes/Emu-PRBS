use dioxus::{CapturedError, prelude::*};
use rfd::MessageDialog;
use std::fs::DirBuilder;
use std::path::Path;
use std::process::Command;

use crate::EmuSettings;
use crate::apputils;

pub fn git_pull(settings: Signal<EmuSettings>) {
    let s = settings.read();
    //let mut log_add = String::new();
    //log_add.push_str("Cloning: \n");
    //log_add.push_str(&String::from_utf8_lossy(&output.stdout).to_string());

    let dir: String = if s.git.get_directory().is_empty() {
        "./".to_string()
    } else {
        s.git.get_directory().to_string().clone()
    };

    let repo_path = format!("{}/{}/.git", dir, &s.git.get_repo());

    if Path::new(&repo_path).exists() == false {
        MessageDialog::new()
            .set_title("Error")
            .set_description("No repository files present at the git directory.")
            .set_buttons(rfd::MessageButtons::Ok)
            .set_level(rfd::MessageLevel::Error)
            .show();
    } else {
        let output = Command::new("git")
            .args(["pull"])
            .current_dir(format!("{dir}/{}", &s.git.get_repo()))
            .output()
            .expect("failed");

        println!("{output:?}");
    }
}

pub fn git_push(settings: Signal<EmuSettings>) {
    let s = settings.read();

    let dir = format!("{}/{}", &s.git.get_directory(), &s.git.get_repo());

    let output = Command::new("git").args(["add", "."]).current_dir(&dir).output().expect("failed");
    println!("{output:?}");

    let output = Command::new("git")
        .args(["commit", "-m", "commit"])
        .current_dir(&dir)
        .output()
        .expect("failed");
    println!("{output:?}");

    let output = Command::new("git").args(["push"]).current_dir(dir).output().expect("failed");
    println!("{output:?}");
}

pub fn add_repo_to_emu(settings: Signal<EmuSettings>, key: String, val: (String, String)) -> Result<(), CapturedError> {
    let s = settings.read();
    let git_path = Path::new(s.git.get_directory()).join(s.git.get_repo()).join(&key);

    match DirBuilder::new().create(&git_path) {
        Ok(()) => println!("Folder doesn't exists, creating..."),
        Err(_) => println!("Folder already exists"),
    }

    let destination = Path::new(&val.1);
    if let Some(dest_parent) = destination.parent() {
        if let Some(source_name) = destination.file_name() {
            apputils::overwrite_folder(&git_path.join(source_name), dest_parent)?;
        }
    }

    Ok(())
}

pub fn add_emu_to_repo(settings: Signal<EmuSettings>) -> Result<(), CapturedError> {
    let s = settings.read();
    let emulators = s.emulators.clone();

    for (key, val) in emulators {
        //Example : C:/Users/Probb/Desktop/test/repo/key
        let git_path = Path::new(s.git.get_directory()).join(s.git.get_repo()).join(&key);

        match DirBuilder::new().create(&git_path) {
            Ok(()) => println!("Folder doesn't exists, creating..."),
            Err(_) => println!("Folder already exists"),
        }

        let destination = Path::new(&val.1); //'C:\RetroArch-Win64\saves'
        apputils::overwrite_folder(&destination.to_path_buf(), &git_path)?
    }

    Ok(())
}
