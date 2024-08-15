// The helpers related to booting the OS.

use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use colored::*;

use super::*;
use crate::data_structs;

// The small boot animation that looks like this: Booting into (your OS)...
// os arg is the name of the OS as a &str
// delay arg is the delay for each . (3 dots total) in milliseconds.
pub fn boot_animation(os: String, delay: u64) {
    let mut stdout = io::stdout();

    for i in 0..=3 {
        // Clear the current line
        print!("\rBooting into {}", os);
        
        // Add dots for animation
        for _ in 0..i {
            print!(".");
        }
        
        // Flush the output to ensure it appears immediately
        stdout.flush().expect("Failed to flush stdout");

        // Sleep for a short duration to create the animation effect
        sleep(Duration::from_millis(delay));
    }

    // Move cursor back to the beginning of the line and print "Booted!"
    print!("\rBooted!                                        ");
    
    // Flush the output to ensure it appears immediately
    stdout.flush().expect("Failed to flush stdout");
    
    // Ensure the final message stays on the screen
    print!("\n");
}

// below are options related to the boot, such as diagnostics or shiggle giggle

pub fn diagnostics(save: &save::Save) {
    // load all the hardware to give the diagnostics (obviously)
    let cpus: data_structs::cpu::Config = hardware::load_cpu();
    let rams: data_structs::ram::Config = hardware::load_ram();
    let oses: data_structs::os::Config = hardware::load_os();

    println!("{}", "Diagnostics".bold());
    println!("  Currently Installed OS: {}", oses.OSes[save.os].Name.green().bold());
    println!("  CPU: {} @ {}{}", cpus.CPUs[save.cpu].Name.green().bold(), cpus.CPUs[save.cpu].Clock_Mhz.to_string().green().bold(), "MHz".green().bold());
    println!("  RAM: {}{} {}", rams.RAM[save.ram].Capacity.to_string().green().bold(), "KB".green().bold(), rams.RAM[save.ram].Technology.green().bold());
}