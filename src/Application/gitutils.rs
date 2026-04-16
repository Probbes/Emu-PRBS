use dioxus::{CapturedError, prelude::*};
use rfd::MessageDialog;
use std::fs::DirBuilder;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::EmuSettings;
use crate::Emulator;
use crate::apputils;

//pull if .git present at the directory used for git
pub fn git_pull(settings: &EmuSettings) {
    let repo_dir = settings.git.get_directory();
    let repo_name = settings.git.get_repo_name();

    let full_repo_path = PathBuf::from(repo_dir).join(repo_name);
    let git_dir = full_repo_path.join(".git");

    if !git_dir.exists() {
        MessageDialog::new()
            .set_title("Error")
            .set_description("No repository files present at the git directory.")
            .set_buttons(rfd::MessageButtons::Ok)
            .set_level(rfd::MessageLevel::Error)
            .show();
    } else {
        let output = Command::new("git")
            .args(["pull"])
            .current_dir(full_repo_path)
            .output()
            .expect("failed");

        println!("{output:?}");
    }
}

// git add all, commit and push
pub fn git_push(settings: &EmuSettings) {
    let repo_dir = settings.git.get_directory();
    let repo_name = settings.git.get_repo_name();

    let repo_path = PathBuf::from(repo_dir).join(repo_name).join(".git");

    let output = Command::new("git")
        .args(["add", "."])
        .current_dir(&repo_path)
        .output()
        .expect("failed");
    println!("{output:?}");

    let output = Command::new("git")
        .args(["commit", "-m", "commit"]) //TODO : commit message with date and time
        .current_dir(&repo_path)
        .output()
        .expect("failed");
    println!("{output:?}");

    let output = Command::new("git").args(["push"]).current_dir(repo_path).output().expect("failed");
    println!("{output:?}");
}

//Add repository save files to the emulator
pub fn add_repo_to_emu(settings: &EmuSettings, emulator_name: &String, emulator_path: &PathBuf) -> Result<(), CapturedError> {
    let git_path = Path::new(settings.git.get_directory())
        .join(settings.git.get_repo_name())
        .join(emulator_name);

    match DirBuilder::new().create(&git_path) {
        Ok(()) => println!("Folder doesn't exists, creating..."),
        Err(_) => println!("Folder already exists"),
    }

    let destination = emulator_path;
    if let Some(dest_parent) = destination.parent() {
        if let Some(source_name) = destination.file_name() {
            apputils::overwrite_folder(&git_path.join(source_name), dest_parent)?;
        }
    }

    Ok(())
}

//Add emulators saves files to repository files to be pushed
pub fn add_emu_to_repo(settings: &EmuSettings) -> Result<(), CapturedError> {
    let emulators = settings.emulators.clone();

    for (key, val) in emulators {
        //Example : C:/Users/Probb/Desktop/test/repo/key
        let git_path = Path::new(settings.git.get_directory())
            .join(settings.git.get_repo_name())
            .join(&key);

        match DirBuilder::new().create(&git_path) {
            Ok(()) => println!("Folder doesn't exists, creating..."),
            Err(_) => println!("Folder already exists"),
        }

        let destination = match val {
            Emulator::RetroArch { path, .. } => path,
            Emulator::Other { path, .. } => path,
            Emulator::None {} => PathBuf::new(),
        };

        apputils::overwrite_folder(&destination.to_path_buf(), &git_path)?
    }

    Ok(())
}
