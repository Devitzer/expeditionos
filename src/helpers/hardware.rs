// Functions related to loading hardware.
// This loads the TOML files in the data/ directory that contains all the hardware used within the game.

use std::fs;

use toml::de::from_str;

use crate::data_structs;

use super::files;

pub fn load_cpu() -> data_structs::cpu::Config {
    let data_dir = files::get_config_directory();

    let cpu_toml_file_str = data_dir.join("data/cpu.toml");

    let cpu_toml_string = fs::read_to_string(cpu_toml_file_str)
        .expect("Failed to read CPU config file.");
    let cpus: data_structs::cpu::Config = from_str(&cpu_toml_string)
        .expect("Expected a proper CPU config. It is not formatted correctly. Refer to the data_structs/cpu.rs structure.");

    cpus
}

pub fn load_ram() -> data_structs::ram::Config {
    let data_dir = files::get_config_directory();

    let ram_toml_file_str = data_dir.join("data/ram.toml");

    let ram_toml_string = fs::read_to_string(ram_toml_file_str)
        .expect("Failed to read RAM config file.");
    let rams: data_structs::ram::Config = from_str(&ram_toml_string)
        .expect("Expected a proper RAM config. It is not formatted correctly. Refer to the data_structs/ram.rs structure.");

    rams
}

pub fn load_os() -> data_structs::os::Config {
    let data_dir = files::get_config_directory();

    let os_toml_file_str = data_dir.join("data/os.toml");

    let os_toml_string = fs::read_to_string(os_toml_file_str)
        .expect("Failed to read RAM config file.");
    let oses: data_structs::os::Config = from_str(&os_toml_string)
        .expect("Expected a proper RAM config. It is not formatted correctly. Refer to the data_structs/ram.rs structure.");

    oses
}