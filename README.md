# simple-cast-gen: An application to generate [asciinema] cast files without recording

[![Crates.io](https://img.shields.io/crates/v/simple-cast-gen.svg)](https://crates.io/crates/cargo-wix)
[![GitHub release](https://img.shields.io/github/release/volks73/simple-cast-gen.svg)](https://github.com/volks73/simple-cast-gen/releases)
[![Crates.io](https://img.shields.io/crates/l/simple-cast-gen.svg)](https://github.com/volks73/simple-cast-gen#license)
[![Build Status](https://github.com/volks73/simple-cast-gen/workflows/CI/badge.svg?branch=master)](https://github.com/volks73/simple-cast-gen/actions?query=branch%3main)

[asciinema]: https://asciinema.org/

## Quick Start

Start a terminal and then execute the following commands:

```bash
~$ simple-cast-gen "echo 'Hello World'" "Hello World"
TODO: STDOUT dump of JSON for a cast file
```

The [asciinema] cast file will be available in the current working directory (cwd).

[asciinema]: https://asciinema.org/

## Installation

### Binary

See the [Releases] page for pre-built binaries and distributions.

[Releases]: https://github.com/volks73/simple-cast-gen/releases

### Source

```sh
~$ git clone https://github.com/volks73/simple-cast-gen.git
~$ cd simple-cast-gen
~/simple-cast-gen$ cargo build --release
~/simple-cast-gen$ cargo install --path ./
```

### Crates.io

```sh
~$ cargo install simple-cast-gen
```

## Usage

```sh
~$ simple-cast-gen "echo 'Hello World!'" "Hello World!"
TODO: Add STDOUT of JSON cast file that is generated
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
