# Freeup


`freeup` is a simple utility designed to free up disk space used by Rust and Web projects. It can remove `node_modules` directories from Node.js projects and run `cargo clean` for Rust projects in the current directory.


## Features

- Removes `node_modules` directories from projects that utilize `npm`.
- Runs `cargo clean` for Rust projects.

## Installation


### Prerequisites


Rust and Cargo must be installed on your system. You can download them from [Rustup](https://rustup.rs/).


### Installing `freeup`


To install `freeup` globally on your system, run the following command:


```sh
git clone https://github.com/hawav/freeup.git
cd freeup
cargo install --path .
```

## Usage

After installing freeup, you can run it with the following command:

```sh
cd THE_DIRECTORY
freeup
```