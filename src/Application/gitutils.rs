use dioxus::{CapturedError, prelude::*};
use rfd::MessageDialog;
use std::fs::DirBuilder;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::apputils;

use crate::Application::settings::*;

//pull if .git present at the directory used for git
pub fn git_pull(settings: &EmuSettings) {
    let repo_dir = settings.git.get_directory();
    let repo_name = settings.git.get_repo_name();

    let full_repo_path = PathBuf::from(repo_dir).join(repo_name);
    let git_dir = full_repo_path.join(".git");

    println!("repo_path : {:?}", git_dir);

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

    let repo_path = PathBuf::from(repo_dir).join(repo_name);

    println!("{:?}", repo_path);
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
pub fn add_repo_to_emu(settings: &EmuSettings, emulator_name: &String) -> Result<(), CapturedError> {
    let git_path = settings.emulators.get(emulator_name).unwrap_or(&Emulator::default()).get_git_path();

    match DirBuilder::new().create(&git_path) {
        Ok(()) => println!("Folder doesn't exists, creating..."),
        Err(_) => println!("Folder already exists"),
    }

    let emulator_save_path = match settings.emulators.get(emulator_name) {
        Some(emulator) => emulator.get_save_path(),
        None => {
            return Err(CapturedError::msg(
                "Error occured while adding repository save files to emulator : Can't get the save path of the emulator",
            ));
        }
    };
    println!("{:?}", emulator_save_path);
    if let Some(dest_parent) = emulator_save_path.parent() {
        if let Some(source_name) = emulator_save_path.file_name() {
            println!("overwrite_folder({:?} - {:?})", &git_path.join(source_name), dest_parent);
            apputils::overwrite_folder(&git_path.join(source_name), dest_parent)?;
            Ok(())
        } else {
            return Err(CapturedError::msg("Error occured while getting the final component of path"));
        }
    } else {
        return Err(CapturedError::msg("Error occured while getting the parent of the directory"));
    }
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

        let destination = val.get_save_path();
        println!("overwrite_folder({:?} - {:?})", &destination.to_path_buf(), &git_path);
        apputils::overwrite_folder(&destination.to_path_buf(), &git_path)?
    }

    Ok(())
}
