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

# Testing

We test on Windows 11 and Linux Mint 22. Sadly Mac testing is currently unavailable, but if you still need to know, we recommend you refer to the Linux Mint statistic as it should be quite similar for MacOS. If it works on Linux, it likely works on MacOS.

## Windows 11

- **Last Tested:** v0.3.1
- **Status:** Successful

## Linux Mint 22

- **Last Tested:** v0.3.0
- **Status:** Successful

# 0.x meaning

A 0.x version means the game is not finished and there is no actual game yet. Meaning you cannot upgrade your parts.