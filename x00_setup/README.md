<span style="display:flex; flex-direction:row; gap:1rem; ">[![License: GPL v3](https://img.shields.io/badge/LICENSE-GPL_v3-blue?logo=gnu&style=for-the-badge)]()
[![License: AGPL v3](https://img.shields.io/badge/LICENSE-AGPL_v3-blue?logo=gnu&style=for-the-badge)]()
</span>

[![Change log](https://img.shields.io/badge/CHANGELOG-0.0.0-limegreen?logo=markdown&style=for-the-badge)](CHANGELOG.md)

> [!IMPORTANT]
> <hr/>
>
> ### Disclaimer
> *All trademarks, third-party assets/logos and brand names in this repository are the property of their respective owner. This project is an independent educational, resource and is not sanctioned, sponsored, or manage by any third-party trademark holders.*

## Cargo project structure hierarchy
```bash
workspace ---> [package] ---> crate ---> module
```
#
# x00_setup
Covers installation, cargo basics, hello world (*hi there*) program, initial (*customize project layout*) setup for Rust development.

## Installation
```bash
#REM: Linux or WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#REM: And for Widows, download an (installer) executable compatible to the architecture

#REM: 'check'
cargo --version

#REM: If failed at 'check', update the system/user env variables/path.
#REM: In Linux or WSL temp fix.
source "$HOME/.cargo/env"
#REM: In Windows do the GUI way.
```
## Customize Basic Project/folder structure
```tree
x00_setup/
|--- .gitignore
|--- README.md
|--- Cargo.toml
|--- rustfmt.toml
|--- src/
|    |--- main/
|    |    |--- rs/
|    |    |    \--- x00_setup/
|    |    |         |--- lib.rs
|    |    |         \--- main.rs
|    |    \--- resources/
|    \--- test/
|         |--- rs/
|         |    \--- x00_setup/
|         |         \--- lib.rs
|         \--- resources/
|--- target/
```
## Cargo *Build-Script* Config file [![Cargo.toml](https://img.shields.io/badge/Cargo\.toml-_-red?logo=rust&style=flat-square)](./Cargo.toml)
```toml
#REM: see @ x00_setup/Cargo.toml
[package]
name = "x00_setup"
version = "0.0.0"
edition = "2024"

[lib] #REM: library (public-interface/exported-api)
crate-type = [
  "rlib", "cdylib"
]
name = "x00_setup"
path = "src/main/rs/x00_setup/lib.rs"

[[bin]] #REM: executable (main-entry-point)
name = "x00_setup"
path = "src/main/rs/x00_setup/main.rs"

[[test]] #REM: integration test (main-entry-point)
name = "x00_setup_test"
path = "src/test/rs/x00_setup/lib.rs"

#REM: ...
```
## Rust *Syntax* Formatter config file [![rustfmt.toml](https://img.shields.io/badge/rustfmt\.toml-_-red?logo=rust&style=flat-square)](./rustfmt.toml)
```toml
#REM: see @ x00_setup/rustfmt.toml
max_width = 80
hard_tabs = false
tab_spaces = 2

brace_layout = "SameLineWhere"

#REM: ...
```

## Cargo Basic Command
```bash
> cargo build --lib
> cargo build --bin <bin_crate_name>
> cargo run --bin <bin_crate_name>
> cargo test --lib #REM: unit-test
> cargo test --bin <bin_crate_name> #REM: unit-test
> cargo test --test <integration_test_crate_name>
> cargo +nightly fmt #REM: rust syntax formatter
> cargo +nightly clippy #REM: rust syntax formatter plugin
> cargo workspaces list [--all | --json | --long]
```