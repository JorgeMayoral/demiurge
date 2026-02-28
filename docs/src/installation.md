# Installation

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with `cargo`)
- [paru](https://github.com/Morganamilo/paru) — required for package management

## Install from source

Clone the repository and install with cargo:

```sh
git clone https://github.com/yorch/demiurge
cd demiurge
cargo install --path .
```

This installs the `dmrg` binary into your cargo bin directory (typically `~/.cargo/bin/`). Make sure that directory is in your `PATH`.

## Verify the installation

```sh
dmrg --version
```
