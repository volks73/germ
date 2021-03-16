# asciicast-gen: An application to generate [asciinema] cast files without recording

This is a command line application for generating [asciinema] [cast files], a.k.a. asciicast files, without using asciinema's record functionality. This is heavily inpsired by the [TermSheets] web application, which creates animated terminal presentations from a simple JSON schema. According to the TermSheets's creator:

>...Other solutions usually involve recording a live screen. I wanted a way to simply provide a payload of instructions so I didn't have to rehearse my typing and wait for network output...

The asciinema application is one of these "Other solutions". Similiarly, I wanted to generate terminal animations without rehearsing, and really appreciated the payload-based implementation. However, I still wanted to create the animations from within the terminal and be able to easily share asciicasts through the [asciinema.org] website. Additional inspiration is taken from the [Termynal] project as well, which was put to excellent use by the [Typer] team for their documentation, but the output for Termynal is not embedable in GitHub README files.

This is all possible because of the excellent documentation, support, and openness of all of the mentioned projects. Thank you!

[![Crates.io](https://img.shields.io/crates/v/simple-cast-gen.svg)](https://crates.io/crates/cargo-wix)
[![GitHub release](https://img.shields.io/github/release/volks73/simple-cast-gen.svg)](https://github.com/volks73/simple-cast-gen/releases)
[![Crates.io](https://img.shields.io/crates/l/simple-cast-gen.svg)](https://github.com/volks73/simple-cast-gen#license)
[![Build Status](https://github.com/volks73/simple-cast-gen/workflows/CI/badge.svg?branch=master)](https://github.com/volks73/simple-cast-gen/actions?query=branch%3main)

[asciinema]: https://asciinema.org/
[cast files]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
[TermSheets]: https://neatsoftware.github.io/term-sheets/
[asciinema.org]: https://asciinema.org
[Termynal]: https://github.com/ines/termynal
[Typer]: https://typer.tiangolo.com/

## Quick Start

Start a terminal and then execute the following commands:

``` sh
~$ asciicast-gen "echo 'Hello World'" "Hello World" > tmp.cast
~$ asciinema play tmp.cast
~$ echo 'Hello World'
Hello World
```

[asciinema]: https://asciinema.org/

## Installation

### Binary

See the [Releases] page for pre-built binaries and distributions.

[Releases]: https://github.com/volks73/simple-cast-gen/releases

### Source

``` sh
~$ git clone https://github.com/volks73/simple-cast-gen.git
~$ cd simple-cast-gen
~/simple-cast-gen$ cargo build --release
~/simple-cast-gen$ cargo install --path ./
```

### Crates.io

``` sh
~$ cargo install simple-cast-gen
```

## Usage

``` sh
~$ simple-cast-gen "echo 'Hello World!'" "Hello World!"
{"version":2,"width":188,"height":55,"timestamp":1615856410,"env":{"SHELL":"/usr/bin/zsh","TERM":"xterm-256color"}}
[0.0,"o","~$ "]
[0.75,"o","e"]
[0.785,"o","c"]
[0.82,"o","h"]
[0.855,"o","o"]
[0.89,"o"," "]
[0.925,"o","'"]
[0.96,"o","H"]
[0.995,"o","e"]
[1.03,"o","l"]
[1.065,"o","l"]
[1.1,"o","o"]
[1.135,"o"," "]
[1.17,"o","W"]
[1.205,"o","o"]
[1.24,"o","r"]
[1.275,"o","l"]
[1.31,"o","d"]
[1.345,"o","'"]
[2.23,"o","\r\n"]
[2.23,"o","Hello World\r\n"]
```

``` sh
~$ simple-cast-gen -o example.cast "echo Hello" "Hello"
~$ simple-cast-gen -a example.cast "echo World!" "World!"
~$ cat example.cast
TODO: ADD STDOUT of JSON cast file that is generated
```

``` sh
~$ cat commands.json
[
    {
        "input": "echo Hello",
        "output": ["Hello"],
    },
    {
        "input": "echo World!",
        "output": ["World!"]
    }
]
~$ simple-cast-gen < commands.json
TODO: Add STDOUT of JSON cast file that is generated
```

``` sh
~$ cat commands.json
[
    {
        "input": "echo Hello",
        "output": ["Hello"],
    },
    {
        "input": "echo World!",
        "output": ["World!"]
    }
]
~$ cat commands.json | simple-cast-gen
TODO: Add STDOUT of JSON cast file that is generated
```

``` sh
~$ cat commands.json
[
    {
        "input": "echo Hello",
        "output": ["Hello"],
    },
    {
        "input": "echo World!",
        "output": ["World!"]
    }
]
~$ simple-cast-gen -i commands.json
TODO: Add STDOUT of JSON cast file that is generated
```

``` sh
~$ simple-cast-gen -I -o example.cast
echo "Hello"
Hello
echo "World!"
World!
~$ cat example.cast
TODO: Add STDOUT of JSON cast file that is generated
```

## Tests

```sh
~$ cargo test
```

## License

The `simple-cast-gen` project is licensed under either the [GPL-3.0]. See the [LICENSE] file for more information about licensing and copyright.

[GPL-3.0]: https://opensource.org/licenses/GPL-3.0
[LICENSE]: https://github.com/volks73/simple-cast-gen/blob/master/LICENSE
