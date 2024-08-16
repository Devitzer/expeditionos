use super::delete;
use super::init;

use dialoguer::Confirm;
use colored::*;

pub fn update() {
    let confirmation = Confirm::new()
        .with_prompt(format!("{} This will delete custom hardware. Continue?", "Warning:".yellow().bold()))
        .default(true)
        .show_default(true)
        .interact()
        .unwrap();

    if confirmation {
    println!("Deleting old hardware data...");
    delete::delete(false);
    init::init();
    println!("Update complete.")
    } else {
        println!("Cancelling update operation.")
    }
}