# Rust Demos
[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

A showcase of Rust features. We have [host manager](#host-manager), [main demo](#main-demo), [sprite demo](#sprite-demo), and [WHMCS demo](#whmcs-demo).

### Host Manager

A collection of command line network utilities.

- [fail2ban*](#fail2ban)
- [hostname](#hostname)
- [process_list](#process_list)
- [ping](#ping)
- [search_email](#search_email)

**These utilities store their output in a [SQLite](https://en.wikipedia.org/wiki/SQLite) database called `history.db`.*

#### fail2ban

```bash
cargo run --example hostman fail2ban <server> <ip_address>
```

- Check if an IP address has been listed by Fail2ban on a server.

#### hostname

```bash
cargo run --example hostman hostname
```

- Output the current hostname.

#### process_list

```bash
cargo run -- process_list <host> [port] [username]
```

- Show then number of processes running on a remote server.

#### ping

```bash
cargo run --example hostman ping 1.1.1.1
```

- Continiously ping a remote host with 2 counts and show the average time taken to reply.

#### search_email

```bash
cargo run --example hostman search_email <server> <email>
```

- Search a remote Postfix server for all emails going TO a person housed on that server. This utility will first present `pass` events and then local Postfix quemanager deliveries.

### Main Demo

- Prints Hello, world!
- Prints a pretty table using `prettytable`
- Reads a [`MySQL`](https://en.wikipedia.org/wiki/MySQL) table called `users`
- Gets [`nginx`](https://en.wikipedia.org/wiki/Nginx) service status
- Prints a directory listing
- Get the size of a directory
- Get total disk size and available space left on the disk
- Write a text file to disk
- Read a text file from disk
- Find and replace text in a text file on disk
- Draw a line on a surface and save it to a file using the Cairo library
- Draw some text characters using [sine and cosine](https://en.wikipedia.org/wiki/Sine_and_cosine) functions
- Use [`Systemd`](https://https://en.wikipedia.org/wiki/Systemd) to restart a Linux service
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

#### Running the Sprite Demo

```bash
cargo run --example sprite
```

### WHMCS Demo

An example of calling the WHMCS billing API and retrieving product values. Showcases async calls and reading from an environment file.

#### Setting up the WHMCS Demo

Copy `.env.example` to `.env` and make sure you have filled in the requisite values for `WHMCS_URL`, `WHMCS_IDENTIFIER`, and `WHMCS_SECRET`.

#### Running the WHMCS Demo

```bash
cargo run --example whmcs
```

## Installation

For the [Systemd](https://https://en.wikipedia.org/wiki/Systemd) utilities to work, do this:

```bash
sudo apt-get install libsystemd-dev
```
## Testing

```bash
cargo test
```
