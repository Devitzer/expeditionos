mod helpers;
mod data_structs;
use std::fs;

use dialoguer::Select;
use colored::*;

/// Starts the bootloader.
/// This app is meant to be used in a global CLI environment (see GitHub for binaries), but we also allow for embedded solutions like this.
/// 
/// This function does not return anything, it simply gives you the menu prompts for the bootloader and acts accordingly.
pub fn start() {
    let save_file: helpers::save::Save = helpers::save::load_save();

    let oses: data_structs::os::Config = helpers::hardware::load_os();

    let boot_options = &["Boot PC", "Diagnostics"];

    let boot = Select::new()
        .with_prompt("Expedition Bootloader v1.0")
        .items(boot_options)
        .default(0)
        .interact()
        .unwrap();

    // handle the initial boot
    let os_styled = oses.OSes[save_file.os].Name.blue();

    match boot {
        0 => helpers::boot::boot_animation(os_styled.to_string(), 2000),
        1 => helpers::boot::diagnostics(&save_file),
        _ => println!("Unknown boot option")
    }

    // simply re-write the save, this happens every time, so that if the save file is changed it will be resaved without issues.
    helpers::save::make_save(save_file);
}

/// Initializes the file necessary to run the game. You only need to run this once.
/// If you don't run it, the start function will return an error.
/// 
/// Files save to either the AppData/Roaming/cpu-game folder on Windows, or home/user/.cpu-game
pub fn init() {
    let config_dir = helpers::files::get_config_directory();

    // Create the configuration directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    let data_dir = config_dir.join("data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
    }

    // Create default configuration files
    let save_file = include_str!("assets/save.toml");
    let data_cpu = include_str!("assets/data/cpu.toml");
    let data_os = include_str!("assets/data/os.toml");
    let data_ram = include_str!("assets/data/ram.toml");

    let save_file_dir = config_dir.join("save.toml");
    if !save_file_dir.exists() {
        let default_config = save_file;
        fs::write(save_file_dir, default_config).expect("Failed to create default config file");
    }
    let data_cpu_file_dir = config_dir.join("data/cpu.toml");
    if !data_cpu_file_dir.exists() {
        let default_config = data_cpu;
        fs::write(data_cpu_file_dir, default_config).expect("Failed to create default config file");
    }
    let data_os_file_dir = config_dir.join("data/os.toml");
    if !data_os_file_dir.exists() {
        let default_config = data_os;
        fs::write(data_os_file_dir, default_config).expect("Failed to create default config file");
    }
    let ram_data_file_dir = config_dir.join("data/ram.toml");
    if !ram_data_file_dir.exists() {
        let default_config = data_ram;
        fs::write(ram_data_file_dir, default_config).expect("Failed to create default config file");
    }

    println!("Initialization complete. Configuration files have been set up.");
}