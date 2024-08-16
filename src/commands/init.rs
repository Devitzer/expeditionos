use std::fs;

use crate::helpers;

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
    let save_file = include_str!("../assets/save.toml");
    let data_cpu = include_str!("../assets/data/cpu.toml");
    let data_os = include_str!("../assets/data/os.toml");
    let data_ram = include_str!("../assets/data/ram.toml");

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
    println!("Initialization/re-initialization complete.");
}