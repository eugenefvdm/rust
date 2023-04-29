# Rust
A couple of introductory Rust examples

[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

This is a demo application to showcase various Rust features.

- Print Hello World
- Write a text file to disk
- Read a text file from disk
- Find and replace text in a text file from disk
- Create a line on a surface and save it to a file using the Cairo library
- Draw some text characters using SINE and COSINE functions
- Create a menu and handle keyboard input
- Extremely rudimentary unit test based on output from ChatGPT
- Sprite (from https://github.com/PistonDevelopers/piston-examples)

## Running

`cargo build;./target/debug/hello_world_rust`

### Sprite

`cargo run --example sprite`

## Testing

`cargo test`
