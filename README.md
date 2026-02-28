# Demiurge

Demiurge is a declarative configuration tool for Arch Linux. You describe your system state in TypeScript (hostname, packages, services, users, and dotfiles) and `dmrg` makes the changes.

## Features

- **TypeScript configuration**: write your config with full type checking and editor support
- **Named configurations**: keep multiple configs (e.g. per machine) in a single file and choose which to apply
- **Packages**: install and remove packages via [paru](https://github.com/Morganamilo/paru) or [cargo](https://doc.rust-lang.org/cargo/)
- **Services**: enable and disable systemd services
- **Users**: manage user group membership
- **Dotfiles**: manage dotfiles as symlinks from a source directory to their target locations
- **System**: configure system settings like the hostname
- **Dry run**: preview changes before applying them
- **Static configs**: export your config to JSON or YAML and apply it from a file or stdin

## Disclaimer

This project is currently tailored for a specific personal setup and is in an early stage. The API may change drastically.

- Requires [paru](https://github.com/Morganamilo/paru) on the system for AUR package management
- Only tested on CachyOS / Arch Linux
