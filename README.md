# Rust Demos
An introduction to Rust.

[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

A few applications that showcases various Rust features.

## Installation

For the [Systemd](https://https://en.wikipedia.org/wiki/Systemd) utilities to work, do this:
`sudo apt-get install libsystemd-dev`

### Host Manager Demo

A collection of command line network utilities.

```bash
cargo run --example hostman fail2ban <server> <ip_address>
cargo run --example hostman hostname
cargo run --example hostman ping 1.1.1.1
cargo run --example search_email <server> <email>
```

### Main Demo

- Prints Hello, world!
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

### Sprite Demo

A sprite animation [example](https://github.com/PistonDevelopers/piston-examples) from PistonDevelopers.


#### Running the Sprite demo

```bash
cargo run --example sprite
```

## Testing

```bash
cargo test
```
