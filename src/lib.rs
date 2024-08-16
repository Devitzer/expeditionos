mod commands;
mod helpers;
mod data_structs;

/// Starts the bootloader.
/// This app is meant to be used in a global CLI environment (see GitHub for binaries), but we also allow for embedded solutions like this.
/// 
/// This function does not return anything, it simply gives you the menu prompts for the bootloader and acts accordingly.
pub fn start() {
    commands::start::start();
}

/// # init
/// Initializes the file necessary to run the game. You only need to run this once.
/// If you don't run it, the start function will return an error.
/// 
/// Files save to either the AppData/Roaming/cpu-game folder on Windows, or home/user/.cpu-game
pub fn init() {
    commands::init::init();
}

/// # delete
/// Deletes all your game data.
/// 
/// This should be run when: <br>
/// You are uninstaling expeditionos. <br>
/// You want to re-initialize your game data.
pub fn delete(delete_save: bool) {
    commands::delete::delete(delete_save);
}

/// # update
/// Updates the game, should be run whenever a new version of expeditionos is installed.
/// 
/// This is the same as running: <br>
/// expeditionos soft-delete <br>
/// expeditionos init
pub fn update() {
    commands::update::update();
}

/// # reset
/// Resets the game, deletes all your data (including saves) and re-initializes the game.
/// 
/// This is the same as running: <br>
/// expeditionos hard-delete <br>
/// expeditionos init
pub fn reset() {
    commands::reset::reset();
}