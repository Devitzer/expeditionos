// Functions related to loading hardware.
// This loads the TOML files in the data/ directory that contains all the hardware used within the game.

use std::{env, fs, path::PathBuf};

use toml::de::from_str;

use crate::data_structs;

pub fn load_cpu() -> data_structs::cpu::Config {
    let exe_path = env::current_exe().expect("Failed to get cpu-game.exe path to executable.");

    let cpu_toml: PathBuf = exe_path
        .parent()
        .expect("Failed to get parent directory of the cpu-game.exe executable. This is required to retrieve the data for your game hardware.")
        .join("data/cpu.toml");

    let cpu_toml_file_str = cpu_toml.to_str().expect("Failed to convert game data path to string.");

    let cpu_toml_string = fs::read_to_string(cpu_toml_file_str)
        .expect("Failed to read CPU config file.");
    let cpus: data_structs::cpu::Config = from_str(&cpu_toml_string)
        .expect("Expected a proper CPU config. It is not formatted correctly. Refer to the data_structs/cpu.rs structure.");

    cpus
}

pub fn load_ram() -> data_structs::ram::Config {
    let exe_path = env::current_exe().expect("Failed to get cpu-game.exe path to executable.");

    let ram_toml: PathBuf = exe_path
        .parent()
        .expect("Failed to get parent directory of the cpu-game.exe executable. This is required to retrieve the data for your game hardware.")
        .join("data/ram.toml");

    let ram_toml_file_str = ram_toml.to_str().expect("Failed to convert game data path to string.");

    let ram_toml_string = fs::read_to_string(ram_toml_file_str)
        .expect("Failed to read RAM config file.");
    let rams: data_structs::ram::Config = from_str(&ram_toml_string)
        .expect("Expected a proper RAM config. It is not formatted correctly. Refer to the data_structs/ram.rs structure.");

    rams
}

pub fn load_os() -> data_structs::os::Config {
    let exe_path = env::current_exe().expect("Failed to get cpu-game.exe path to executable.");

    let os_toml: PathBuf = exe_path
        .parent()
        .expect("Failed to get parent directory of the cpu-game.exe executable. This is required to retrieve the data for your game hardware.")
        .join("data/os.toml");

    let os_toml_file_str = os_toml.to_str().expect("Failed to convert game data path to string.");

    let os_toml_string = fs::read_to_string(os_toml_file_str)
        .expect("Failed to read RAM config file.");
    let oses: data_structs::os::Config = from_str(&os_toml_string)
        .expect("Expected a proper RAM config. It is not formatted correctly. Refer to the data_structs/ram.rs structure.");

    oses
}