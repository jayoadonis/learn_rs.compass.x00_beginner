# learn_rs.compass.x00_beginner ![Rust Project](https://img.shields.io/badge/RUST_PROJECT-gray?logo=rust&logoColor=orange&style=for-the-badge)

<span style="display:flex; flex-direction:row; gap:1rem;">[![License: GPL v3](https://img.shields.io/badge/LICENSE-GPL_v3-blue?logo=gnu&style=for-the-badge)]()
[![License: AGPL v3](https://img.shields.io/badge/LICENSE-AGPL_v3-blue?logo=gnu&style=for-the-badge)]()</span>

## Discription
A personal journal exploring Rust, the project contain a beginner journey.

## Disclaimer
> [!IMPORTANT]
> *All trademarks, third-party assets/logos and brand names used in this repository/project are the property of their respective owners. This project is an independent educational, resource and is not sanctioned, sponsored, or manage by any third-party tramemark holders.*

## Cargo project structure hierarchy
```bash
[workspace] ---> package ---> crate ---> module
```

## Feature
+ [x] &#128279;[x00_setup](./x00_setup/)*
+ [ ] [x00_data_type_handling]()

## Requirement
+ &#128279;[rustup](https://rust-lang.org/tools/install/)

## Installation
```bash
$ git clone --recursive <repo_url>
$ cd <project_workspace_filepath> #REM: current workspace (e.g. learn_rs.compass.x00_beginner)
$ cd <project_package_filepath> #REM: If we want the concrete project (e.g. x00_setup, x00_data_type_handling, or etc...)
```

## Usage
#### At workspace
```bash
$ cargo build -p <package_name>
$ cargo build -p <package_name> --lib
$ cargo build -p <pacakge_name> --bin <bin_crate_name>

$ cargo run -p <package_name> --bin <bin_crate_name>

$ cargo test -p <package_name> [-- [--show-output | --no-capture]]
$ cargo test -p <package_name> --lib 
$ cargo test -p <package_name> --bin <bin_crate_name> 
$ cargo test -p <package_name> --test <integration_test_crate_name> 
```
#### At package
```bash
$ cargo build #REM: build all
$ cargo build --lib #REM: build library
$ cargo build --bin <bin_crate_name> #REM: build executable

$ cargo run --bin <bin_crate_name> #REM: run executable

$ cargo test [-- [--show-output | --no-capture]] #REM: test all
$ cargo test --lib #REM: unit-test
$ cargo test --bin <bin_crate_name> #REM: unit-test
$ cargo test --test <integration_test_crate_name>
```
<!-- ## License -->

<!-- ## How to contribute -->

<!-- ## Author and Maintainer -->