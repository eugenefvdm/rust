# Rust Demo Applications
An introduction to Rust.

[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

Demo applications that showcases various Rust features.

## Installation

For the [Systemd](https://https://en.wikipedia.org/wiki/Systemd) utilities to work, do this:
`sudo apt-get install libsystemd-dev`

### Main Showcase

- Prints Hello World
- Reads a MySQL `users` table
- Get a Linux service status
- Prints a directory listing
- Use `Systemd` to restart a Linux service
- Get the size of a folder
- Get total disk size and available space left on the disk
- Write a text file to disk
- Read a text file from disk
- Find and replace text in a text file from disk
- Create a line on a surface and save it to a file using the Cairo library
- Draw some text characters using [sine and cosine](https://en.wikipedia.org/wiki/Sine_and_cosine) functions
- Create a console menu and handle keyboard input
- A rudimentary unit test

#### Running the Main Demo

`cargo run` or
`cargo build;./target/debug/hello_world_rust`

Build Release:

`cargo build --release`

### Seperate Demos

- Sprite (from https://github.com/PistonDevelopers/piston-examples)

#### Running the Sprite demo

`cargo run --example sprite`

## Testing

`cargo test`
