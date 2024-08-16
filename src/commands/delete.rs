use std::fs;

use crate::helpers;

pub fn delete(delete_save: bool) {
    let config_dir = helpers::files::get_config_directory();

    if delete_save { 
        fs::remove_dir_all(config_dir).expect("Failed to delete all data.");
    } else {
        fs::remove_dir_all(config_dir.join("data")).expect("Failed to delete hardware data.");
    }
}