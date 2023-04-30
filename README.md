# Rust Demo Applications
An introduction to Rust.

[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

Demo applications that showcases various Rust features.

## Installation

For the [Systemd](https://https://en.wikipedia.org/wiki/Systemd) utilities to work, do this:
`sudo apt-get install libsystemd-dev`

### Main Demo

- Prints Hello World
- Reads a `MySQL` table called `users`
- Gets nginx service status
- Prints a directory listing
- Get the size of a directory
- Get total disk size and available space left on the disk
- Write a text file to disk
- Read a text file from disk
- Find and replace text in a text file on disk
- Draw a line on a surface and save it to a file using the Cairo library
- Draw some text characters using [sine and cosine](https://en.wikipedia.org/wiki/Sine_and_cosine) functions
- Use `Systemd` to restart a Linux service
- Create a console menu and handle keyboard input
- A rudimentary unit test

#### Running the Main Demo

```bash
cargo run
```

or

```bash
cargo build
./target/debug/hello_world_rust
```

or

```bash
cargo build --release
./target/release/hello_world_rust
```

### Host Manager Demo

The host manager demo is a collection of command line network utilities. Examples:

```bash
cargo run --example hostman hostname
cargo run --example hostman ping 1.1.1.1
```

### Sprite Demo

This demo is in the examples directory.

- Sprite (from https://github.com/PistonDevelopers/piston-examples)

#### Running the Sprite demo

`cargo run --example sprite`

## Testing

`cargo test`
