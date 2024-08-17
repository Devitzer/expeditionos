use std::{env, path::PathBuf};

pub fn get_config_directory() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(env::var("APPDATA").unwrap_or_else(|_| "C:\\ProgramData".to_string()))
            .join("expeditionos")
    } else {
        PathBuf::from(env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
            .join(".expeditionos")
    }
}