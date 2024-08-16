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

# 0.x meaning

A 0.x version means the game is not finished and there is no actual game yet. Meaning you cannot upgrade your parts.