# Rust Examples
[![Rust build](https://github.com/eugenevdm/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/eugenevdm/rust/actions/workflows/rust.yml)

## About

Rust in action.

- [host manager](#host-manager)
- [main demo](#main-demo)
- [sprite demo](#sprite-demo)
- [Virtualmin API](#virtualmin-api)
- [WHMCS API](#whmcs-api)

### Host Manager

A collection of command line network utilities.

- [fail2ban*](#fail2ban)
- [history](#history)
- [hostname*](#hostname)
- [process_list*](#process_list)
- [ping*](#ping)
- [search_email](#search_email)
- [search_email_log*](#search_email_log)

**These utilities store their output in a [SQLite](https://en.wikipedia.org/wiki/SQLite) database called `history.db`.*

#### fail2ban

```bash
cargo run --example hostman fail2ban <server> <ip_address>
```

Check if an IP address has been listed by Fail2ban on a server. Example output:

```bash
IP address A.B.C.D was banned at 2023-05-01 13:47:26
```

#### history

```bash
cargo run --example hostman history
```

Output the history table.

#### hostname

```bash
cargo run --example hostman hostname
```

Output the current hostname in fully qualified domain name format. Example output:

```bash
user-laptop.example.com
```

#### process_list

```bash
cargo run -- process_list <host> [port] [username]
```

Show then number of processes running on a remote server. Example output:

```bash
1256 processes running.
```

#### ping

```bash
cargo run --example hostman ping 1.1.1.1
```

Continiously ping a remote host with 2 counts and show the average time taken to reply. Example output:

```bash
Average ping time for 1.1.1.1 = 4.738 ms
Average ping time for 1.1.1.1 = 3.075 ms
Average ping time for 1.1.1.1 = 3.373 ms
Average ping time for 1.1.1.1 = 4.863 ms
^C
```

Pressing control-C will stop the ping.

#### search_email

```bash
cargo run --example hostman search_email <server> <email>
```

Search a remote [`Postfix`](https://en.wikipedia.org/wiki/Postfix_(software)) server for all emails going TO a person housed on that server. This utility will first filter `pass` events and then show local [`qmgr`](https://www.postfix.org/qmgr.8.html) deliveries.

#### search_email_log

```bash
cargo run --example hostman search_email_log <server> <ip_address>
```

This will search a remote `Postfix` server's log file for occurances of Dovecot `imap-login` and `auth failed` on `<ip_address>` and report the first time this happened.

### Main Demo

This is a sequential demo that will run through all the below bullet items one by one. There is a pause in between to see output.

- Prints Hello, world!
- Outputs threads
- Prints a pretty table using [`prettytable`](https://github.com/phsym/prettytable-rs)
- Reads a [`MySQL`](https://en.wikipedia.org/wiki/MySQL) table called `users`
- Gets [`nginx`](https://en.wikipedia.org/wiki/Nginx) service status
- Prints a directory listing
- Get the size of a directory
- Get total disk size and available space left on the disk
- Write a text file to disk
- Read a text file from disk
- Find and replace text in a text file on disk
- Draw a line on a surface and save it to a file using the Cairo library
- Draw some text characters using [`sine and cosine`](https://en.wikipedia.org/wiki/Sine_and_cosine) functions
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

### Virtualmin API

An example of calling the Virtualmin API and retrieving mailboxes sizes.

#### Setting up the Virtualmin API

Copy `.env.example` to `.env` and make sure you have filled in the environment variables for `VIRTUALMIN_USERNAME` and `VIRTUALMIN_PASSWORD`.

#### Virtualmin API Commands

##### List-Users Output Demo

```bash
cargo run --example virtualmin demo list-users
```

`list-users` example JSON output intepretation from the Virtualmin API. Useful for studying JSON output format and result code.

##### Get Mailbox Sizes

```bash
cargo run --example virtualmin mailboxes <domain> <server>
```

Use the Virtualmin Remote API to log into a remote <server> and retrieve all the <mailboxes> and sizes for a <domain>.

### WHMCS API

An example of calling the WHMCS billing API and retrieving product values. Showcases async calls and reading from an environment file.

#### Setting up the WHMCS API

Copy `.env.example` to `.env` and make sure you have filled in the requisite values for `WHMCS_URL`, `WHMCS_IDENTIFIER` and `WHMCS_SECRET`.

#### Running the WHMCS API

```bash
cargo run --example whmcs
```

## Installation

For the [`Systemd`](https://https://en.wikipedia.org/wiki/Systemd) utilities to work, do this:

```bash
sudo apt-get install libsystemd-dev
```
## Testing

```bash
cargo test
```
