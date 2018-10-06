# bkm

bkm is a simple bookmark manager written in Rust.


## Installation

This requires at least Rust 1.27 and Cargo to be installed. Type the following in the terminal:

```
$ cargo install --git https://github.com/n2kia4/bkm.git
```

If you want to contribute to bkm, type the following in the terminal:

```
$ git clone https://github.com/n2kia4/bkm.git && cd bkm
$ cargo build
```


## Usage

bkm requires to specify a subcommand. In order to see what subcommands are available, run `bkm -h`

```
Bookmark manager

USAGE:
    bkm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    print     Print bookmark
    add       Add bookmark
    delete    Delete bookmark
    update    Update bookmark
    help      Prints this message or the help of the given subcommand(s)
```

To check options of the subcommand, run `bkm <SUBCOMMAND> -h`

```
Add bookmark

USAGE:
    bkm add [OPTIONS] <URL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tag <tag>...     Add tags to bookmark
    -i, --title <title>    Decide bookmark title yourself

ARGS:
    <URL>    Bookmark URL
```
