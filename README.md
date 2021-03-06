# <em>G</em>enerate t<em>ERM</em>inal: An application to generate terminal session recordings without rehearsing or recording

This is a command line application for generating terminal session recording files, such as [asciinema]'s [asciicast files], from a payload of inputs and outputs instead of recording a terminal session. The name comes from the progression of "generate terminal" to "genterm" to "gterm" to "germ". This is heavily inspired by the [TermSheets] web application, which creates animated terminal presentations from a simple JSON schema. According to the TermSheets's creator:

>...Other solutions usually involve recording a live screen. I wanted a way to simply provide a payload of instructions so I didn't have to rehearse my typing and wait for network output...

While asciinema and its ecosystem is great, it is one of these "Other solutions". Similiarly, I wanted to generate terminal animations without rehearsing, and really appreciated the payload-based implementation. However, I still wanted to create the recordings from within the terminal and be able to easily share asciicasts through the [asciinema.org] website. The TermSheets web application required generating JSON within the website and/or copying-n-pasting from a text editor. Additional inspiration is taken from the [Termynal] project, which was put to excellent use by the [Typer] team for their documentation, but the output for Termynal is not embeddable in GitHub README files.

This is all possible because of the excellent documentation, support, and openness of all of the mentioned projects. Thank you!

[![Build Status](https://github.com/volks73/germ/workflows/CI/badge.svg)](https://github.com/volks73/germ/actions)

[asciinema]: https://asciinema.org/
[asciicast files]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
[TermSheets]: https://neatsoftware.github.io/term-sheets/
[asciinema.org]: https://asciinema.org
[Termynal]: https://github.com/ines/termynal
[Typer]: https://typer.tiangolo.com/

## Quick Start

Start a terminal and then execute the following commands:

[![asciicast](https://asciinema.org/a/JvgdZbR7YSpA44y1PgPL9PB0w.svg)](https://asciinema.org/a/JvgdZbR7YSpA44y1PgPL9PB0w?autoplay=1&loop=1)

## Installation

### Source

``` sh
~$ git clone https://github.com/volks73/germ.git
~$ cd germ
~/germ$ cargo install --path .
```

## Usage

``` sh
~$ germ "echo 'Hello World'" "Hello World"
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
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
~$ germ -G "echo 'Hello World'" "Hello World" | germ -G "echo 'Hello World Again'" "Hello World Again" | germ
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
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
[2.23,"o","$ "]
[2.98,"o","e"]
[3.015,"o","c"]
[3.05,"o","h"]
[3.085,"o","o"]
[3.12,"o"," "]
[3.1550000000000002,"o","'"]
[3.19,"o","H"]
[3.225,"o","e"]
[3.26,"o","l"]
[3.295,"o","l"]
[3.33,"o","o"]
[3.365,"o"," "]
[3.4,"o","W"]
[3.435,"o","o"]
[3.4699999999999998,"o","r"]
[3.505,"o","l"]
[3.54,"o","d"]
[3.575,"o"," "]
[3.61,"o","A"]
[3.645,"o","g"]
[3.6799999999999997,"o","a"]
[3.715,"o","i"]
[3.75,"o","n"]
[3.785,"o","'"]
[4.67,"o","\r\n"]
[4.67,"o","Hello World Again\r\n"]
```

``` sh
~$ germ -G "echo 'Hello World'" "Hello World"
{
    "version": 1,
    "commands": [
        {
            "input": "echo 'Hello World'",
            "outputs": ["Hello World"],
        },
    ]
}
~$ germ -G "echo 'Hello World'" "Hello World" > commands.json
~$ germ < commands.json
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
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
~$ cat commands.json | germ
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
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
~$ germ -i commands.json
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
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
germ -p "`printf '\u001b[32m$\u001b[39m '`" "echo 'Hello World'" "Hello World"
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","\u001b[32m$ \u001b[39m"]
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
[4.23,"o",""]
```

``` sh
~$ germ
Copyright (C) 2021  Christopher R. Field
This program comes with ABSOLUTELY NO WARRANTY; for details use the `--warranty`
flag. This is free software, and you are welcome to redistirbute it under
certain conditions; use the `--license` flag for details.

You have entered interactive mode. The prompt has similar arguments, options,
flags, and functionality to the command line interface. Use the --help flag to
print the help text.

Type CTRL+D (^D) to exit and generate output or CTRL+C (^C) to abort.

>>> "echo Hello World"
World Hello
>>> "echo Hello World Again"
Hello World Again
>>> --print
{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
[0.75,"o","e"]
[0.785,"o","c"]
[0.82,"o","h"]
[0.855,"o","o"]
[0.89,"o"," "]
[0.925,"o","\""]
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
[1.345,"o","\""]
[2.23,"o","\r\n"]
[2.23,"o","Hello World\r\n"]
[2.23,"o","$ "]
[2.98,"o","e"]
[3.015,"o","c"]
[3.05,"o","h"]
[3.085,"o","o"]
[3.12,"o"," "]
[3.155,"o","\""]
[3.19,"o","H"]
[3.225,"o","e"]
[3.26,"o","l"]
[3.295,"o","l"]
[3.33,"o","o"]
[3.365,"o"," "]
[3.4,"o","W"]
[3.435,"o","o"]
[3.469,"o","r"]
[3.505,"o","l"]
[3.54,"o","d"]
[3.575,"o","\""]
[3.61,"o","\n"]
[3.645,"o","e"]
[3.679,"o","c"]
[3.715,"o","h"]
[3.75,"o","o"]
[3.785,"o"," "]
[3.82,"o","\""]
[3.855,"o","H"]
[3.889,"o","e"]
[3.925,"o","l"]
[3.96,"o","l"]
[3.995,"o","o"]
[4.03,"o"," "]
[4.064,"o","W"]
[4.1,"o","o"]
[4.135,"o","r"]
[4.17,"o","l"]
[4.205,"o","d"]
[4.24,"o"," "]
[4.275,"o","A"]
[4.31,"o","g"]
[4.345,"o","a"]
[4.38,"o","i"]
[4.415,"o","n"]
[4.45,"o","\""]
[5.335,"o","\r\n"]
[5.335,"o","Hello World Again\r\n"]
[7.335,"o",""]
```

## License

The `germ` project is licensed under either the [GPL-3.0]. See the [LICENSE] file for more information about licensing and copyright.

[GPL-3.0]: https://opensource.org/licenses/GPL-3.0
[LICENSE]: https://github.com/volks73/germ/blob/main/LICENSE
