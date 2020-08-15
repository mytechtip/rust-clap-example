# Rust Clap Example

This program demos a simple example of using clap to implement sub commands. 

## Build

Use the following command to build

```bash
cargo build
```

The built artifact will be in `target/debug`

## Run

You can either use `cargo run` or `target/debug/rust-clap-example` to run the program. It will show you the following help message.

```
rust-clap-example 1.0
Robin Wang<mytechtip@github>
This program can say 'hello' to you, show you 'quote' of the day and can even help you 'add' up numbers!

USAGE:
    rust-clap-example <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add up a list of numbers
    hello    Say hello
    help     Prints this message or the help of the given subcommand(s)
    quote    Get a number of quotes to start your day
```


