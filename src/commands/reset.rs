use super::delete;
use super::init;

use dialoguer::Confirm;
use colored::*;

pub fn reset() {
    let confirmation = Confirm::new()
        .with_prompt(format!("{} This will delete all your data, continue?", "Warning:".yellow().bold()))
        .default(true)
        .show_default(true)
        .interact()
        .unwrap();

    if confirmation {
    delete::delete(true);
    init::init();
    } else {
        println!("Cancelling reset operation.");
    }
}