mod helpers;
mod data_structs;
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