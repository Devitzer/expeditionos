// File to serialize/deserialize saves.

use serde::Deserialize;
use serde::Serialize;
use toml::from_str;
use std::fs;

use super::files;

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
    let data_dir = files::get_config_directory();

    fs::write(data_dir.join("save.toml"), toml_string).expect("Failed to save the save file.");
}

pub fn load_save() -> Save {
    let data_dir = files::get_config_directory();

    let save_toml_file_str = data_dir.join("save.toml");

    let save_toml_string = fs::read_to_string(save_toml_file_str)
        .expect("Failed to read save file.");
    let save: Save = from_str(&save_toml_string)
        .expect("Expected a proper save. It is not formatted correctly. Refer to the data_structs/cpu.rs structure.");

    save
}