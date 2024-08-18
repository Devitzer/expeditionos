# ExpeditionOS (Game)

This is a game where you upgrade your fictional computer with fictional parts to have the best computer.

# How to Run

## Installing Globally

1. Run `cargo install expeditionos`
2. Run `expeditionos init` to initialize the save and data files. Simple, right?

## Installing Locally

1. Run `cargo add expeditionos`
2. In your main.rs file, write `use expeditionos`
3. Initialize the files by running `expeditionos::init()`, this only needs to be done once (and can be ran - again if you need to reset the data)
4. Start the game using `expeditionos::start()`

# Updating your Installation

Upon installing an update that introduces new hardware, you should run `expeditionos update` to add that new hardware to your hardware data file. This will not overwrite your save, but it will delete custom hardware, so do not run it if you don't want to. Sometimes you will have to run it, such as if a new field is added to any of the hardware data that all the old data does not have, unless you want to manually add all the fields, we suggest the update command/function.

# Testing

We test on Windows 11 and Linux Mint 22. Sadly, macOS testing is currently unavailable, but if you still need to know, if it works on a Linux-based system, it likely works on macOS.

Operating systems that are tested less often will still be rated for the latest version based on this hierarchy:

- Highly Likely; Almost guaranteed to work with this OS.
- Likely; Might work with this OS. Usually rated down to this if a new dependency is introduced or something of the sorts.
- Unlikely; Probably won't work with this OS. Rarely rated here.

## Windows 11

- **Last Tested:** v0.4.2
- **Status:** Successful

## Linux Mint 22

- **Last Tested:** v0.3.0 (Highly Likely v0.4.2)
- **Status:** Successful

# 0.x meaning

A 0.x version means the game is not finished and there is no actual game yet. Meaning you cannot upgrade your parts. It also means that a minor version increase can introduce breaking changes. Check the [changelog](./CHANGELOG.md) before updating!