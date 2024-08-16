use std::env;

mod data_structs;
mod helpers;
mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "init" {
        commands::init::init();
    } else if args.len() > 1 && args[1] == "update" {
        commands::update::update();
    } else if args.len() > 1 && args[1] == "reset" {
        commands::reset::reset();
    } else if args.len() > 1 && args[1] == "soft-delete" {
        commands::delete::delete(false);
    } else if args.len() > 1 && args[1] == "hard-delete" {
        commands::delete::delete(true);
    } else {
        commands::start::start();
    }
}
