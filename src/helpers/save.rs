// File to serialize/deserialize saves.

use serde::Deserialize;
use serde::Serialize;
use toml::from_str;
use std::env;
use std::fs;
use std::path::PathBuf;

// The save struct

#[derive(Serialize, Deserialize, Debug)]
pub struct Save {
    pub cpu: usize,
    pub os: usize,
    pub ram: usize
}

// Encode the save file.
// The save arguement should be the Save struct..
pub fn make_save(save: Save) {
    let toml_string = toml::to_string(&save).expect("Failed to serialize save game.");
    
    let exe_path = env::current_exe().expect("Failed to get cpu-game.exe path.");

    let save_toml: PathBuf = exe_path
        .parent()
        .expect("Failed to retrieve cpu-game.exe parent directory.")
        .join("save.toml");

    fs::write(save_toml, toml_string).expect("Failed to save the save file.");
}

pub fn load_save() -> Save {
    let exe_path = env::current_exe().expect("Failed to get cpu-game.exe path to executable.");

    let save_toml: PathBuf = exe_path
        .parent()
        .expect("Failed to get parent directory of the cpu-game.exe executable. This is required to retrieve the data for your game hardware.")
        .join("save.toml");

    let save_toml_file_str = save_toml.to_str().expect("Failed to convert game data path to string.");

    let save_toml_string = fs::read_to_string(save_toml_file_str)
        .expect("Failed to read save file.");
    let save: Save = from_str(&save_toml_string)
        .expect("Expected a proper save. It is not formatted correctly. Refer to the data_structs/cpu.rs structure.");

    save
}