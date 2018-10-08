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
    open      Open bookmark
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


## Examples

### `Print`

Print all bookmarks:

```
$ bkm print
```

Print bookmarks at index 1 and 2:

```
$ bkm print 1 2
```

Print all tags:

```
$ bkm print -T
```

### `Add`

Add bookmark with tags "git" and "hosting service":

```
$ bkm add https://github.com -t git "hosting service"
1 The world’s leading software development platform · GitHub
    https://github.com
    git, hosting service
```

Add bookmark with tags "git" and "hosting service", title "GitHub":

```
$ bkm add https://github.com -t git "hosting service" -i GitHub
1 GitHub
    https://github.com
    git, hosting service
```

### `delete`

Delete all bookmarks:

```
$ bkm delete
```

Delete bookmarks at index 1 and 2:

```
$ bkm delete 1 2
```

Delete all tags:

```
$ bkm delete -T
```

Delete tags at index 1 and 2:

```
$ bkm delete -t 1 2
```

### `update`

Update bookmark URL and title at index 1:

```
$ bkm update 1 -u https://github.com/n2kia4/bkm -i "bkm"
```

Update bookmark tags at index 1:

```
$ bkm update 1 -t github "bookmark manager" Rust
```

Get and update title from bookmark URL at index 1:

```
$ bkm update 1
```

### `open`

Open bookmark at index 1 and 2 in browser:

```
$ bkm open 1 2
```
