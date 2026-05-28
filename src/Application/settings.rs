use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EmuSettings {
    pub username: String,
    pub game_size: u8,
    pub emu_size: u8,
    pub project_folder: PathBuf,
    pub pure_name: bool,
    pub games: HashMap<u32, Game>,
    pub emulators: HashMap<String, Emulator>,
    pub git: EmuGit,
}

impl Default for EmuSettings {
    fn default() -> Self {
        Self {
            game_size: 5,
            emu_size: 5,

            username: Default::default(),
            project_folder: Default::default(),
            pure_name: Default::default(),
            games: Default::default(),
            emulators: Default::default(),
            git: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct Game {
    pub name: String,
    pub extension: String,
    pub path: PathBuf,
    pub fullscreen: bool,
    pub emulator: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Emulator {
    RetroArch {
        name: String,
        path: PathBuf,
        default_fullscreen: bool,
        save_path: PathBuf,
        git_path: PathBuf,
        core: PathBuf,
    },
    Other {
        name: String,
        path: PathBuf,
        default_fullscreen: bool,
        save_path: PathBuf,
        git_path: PathBuf,
    },
    New(PathBuf),
}

impl Default for Emulator {
    fn default() -> Self {
        Self::New(PathBuf::new())
    }
}

impl Emulator {
    pub fn get_name(&self) -> &str {
        match self {
            Emulator::RetroArch { name, .. } => name,
            Emulator::Other { name, .. } => name,
            Emulator::New(_v) => "New Emulator",
        }
    }
    pub fn get_path(&self) -> &PathBuf {
        match self {
            Emulator::RetroArch { path, .. } => path,
            Emulator::Other { path, .. } => path,
            Emulator::New(path) => path,
        }
    }
    pub fn get_fullscreen(&self) -> &bool {
        match self {
            Emulator::RetroArch { default_fullscreen, .. } => default_fullscreen,
            Emulator::Other { default_fullscreen, .. } => default_fullscreen,
            Emulator::New(..) => &false,
        }
    }
    pub fn get_core(&self) -> PathBuf {
        match self {
            Emulator::RetroArch { core, .. } => core.clone(),
            _ => PathBuf::new(),
        }
    }
    pub fn get_save_path(&self) -> PathBuf {
        match self {
            Emulator::RetroArch { save_path, .. } => save_path.clone(),
            Emulator::Other { save_path, .. } => save_path.clone(),
            _ => PathBuf::new(),
        }
    }
    pub fn get_git_path(&self) -> PathBuf {
        match self {
            Emulator::RetroArch { git_path, .. } => git_path.clone(),
            Emulator::Other { git_path, .. } => git_path.clone(),
            _ => PathBuf::new(),
        }
    }
    pub fn set_name(&mut self, s: String) {
        match self {
            Emulator::RetroArch { name, .. } => *name = s,
            Emulator::Other { name, .. } => *name = s,
            _ => {}
        }
    }
    pub fn set_path(&mut self, p: PathBuf) {
        match self {
            Emulator::RetroArch { path, .. } => *path = p,
            Emulator::Other { path, .. } => *path = p,
            Emulator::New(path) => *path = p,
        }
    }
    pub fn set_fullscreen(&mut self, b: bool) {
        match self {
            Emulator::RetroArch { default_fullscreen, .. } => *default_fullscreen = b,
            Emulator::Other { default_fullscreen, .. } => *default_fullscreen = b,
            _ => {}
        }
    }
    pub fn set_core(&mut self, p: PathBuf) {
        match self {
            Emulator::RetroArch { core, .. } => *core = p,
            _ => {}
        }
    }
    pub fn set_save_path(&mut self, p: PathBuf) {
        match self {
            Emulator::RetroArch { save_path, .. } => *save_path = p,
            Emulator::Other { save_path, .. } => *save_path = p,
            _ => {}
        }
    }
    pub fn set_git_path(&mut self, p: PathBuf) {
        match self {
            Emulator::RetroArch { git_path, .. } => *git_path = p,
            Emulator::Other { git_path, .. } => *git_path = p,
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct EmuGit {
    repo_name: String,
    directory: PathBuf,
    save_dir: HashMap<String, PathBuf>,
}

impl EmuGit {
    pub fn get_directory(&self) -> &PathBuf {
        &self.directory
    }
    pub fn get_repo_name(&self) -> &str {
        &self.repo_name
    }
    pub fn set_directory(&mut self, p: PathBuf) {
        self.directory = p;
    }
    pub fn set_repo_name(&mut self, s: String) {
        self.repo_name = s;
    }
    pub fn get_save_dir(&self) -> &HashMap<String, PathBuf> {
        &self.save_dir
    }
    pub fn set_save_dir(&mut self, hash: HashMap<String, PathBuf>) {
        self.save_dir = hash;
    }
    pub fn add_save_dir(&mut self, k: String, v: PathBuf) {
        self.save_dir.insert(k, v);
    }
}
