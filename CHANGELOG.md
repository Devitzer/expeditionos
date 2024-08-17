# 0.4.1 (2024-08-17)

## Upgrades

- Upgrade `serde` from `v1.0.207` to `v1.0.208`

# 0.4.0 (2024-08-16)

## Added

- Descriptions to arguements for library functions.
- Testing info to README.md.
- New RAM types up to 1995.

## Changed

- **BREAKING CHANGE**: Data for the game will now be stored in the following directory: <br>
**Windows:** C:\Users\(username here)\AppData\Roaming\expeditionos <br>
**Linux/MacOS:** ~/.expeditionos

**How To Transfer Saves:** Simple, move your save.toml from the cpu-game (or .cpu-game) to the expeditionos (or .expeditionos) directory. After this, run `expeditionos init` again and you will be good to go.

- Made RAM capacities simpler to understand.

## Notice

0.x branches may see breaking changes without necessarily increasing the major branch number. Please check the changelog before you update your game.

# 0.3.0 (2024-08-16)

## Added

- Added new commands (reset, soft-delete, hard-delete, update)
- Dates to Changelog versions.

# 0.2.0

## Added

- New CPUs up to 1995.
- Added a Changelog to the project.