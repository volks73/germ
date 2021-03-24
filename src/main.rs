// Copyright (C) 2021  Christopher R. Field
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # `germ` command line application binary
//!
//! The goal of the `germ` project is to create terminal session recordings from
//! a user-defined sequence of inputs and outputs instead of recording a
//! session. This eliminates rehearsing and re-recordings while providing a
//! consistent pace in playback.
//!
//! Gratitude and acknowledgements are given for the [asciinema], [TermSheets],
//! [Termynal], and [Typer] projects for inspiration, documentation, openness,
//! and feedback.
//!
//! ## Table of Contents
//!
//! - [Quick Start](#quick-start)
//! - [Examples](#examples)
//!
//! ## Quick Start
//!
//! Ensure the [asciinema] application is installed and start a suitable
//! terminal and execute the following commands:
//!
//! ```sh
//! ~$ germ "echo Hello World" "Hello World" > quick-start.cast
//! ~$ asciinema play quick-start.cast
//! $ echo Hello World
//! Hello World
//! ~$
//! ```
//!
//! The above sequence of commands will create an [asciicast] file from the
//! `echo Hello World` input with the `Hello World` output and then playback the
//! asciicast file with the asciinema application. The double quotes are needed
//! to group words into an input and a separate output string. The input will be
//! replayed as a series of typed characters, while the output is replayed as a
//! single line of text.
//!
//! [asciinema]: https://asciinema.org/docs/installation
//! [asciicast]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
//!
//! ## Examples
//!
//! Begin each example by starting an appropriate terminal. Each example assumes
//! execution from the user's `$HOME` directory, i.e. `~`, unless explicitly
//! stated to the contrary.
//!
//! Let's start with the simplest example, which is akin to the ["Hello, World!" program]:
//!
//! ```sh
//! ~$ germ "echo 'Hello, World!'"
//! {"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
//! [0.0,"o","$ "]
//! [0.75,"o","e"]
//! [0.785,"o","c"]
//! [0.82,"o","h"]
//! [0.855,"o","o"]
//! [0.89,"o"," "]
//! [0.925,"o","'"]
//! [0.96,"o","H"]
//! [0.995,"o","e"]
//! [1.03,"o","l"]
//! [1.065,"o","l"]
//! [1.1,"o","o"]
//! [1.135,"o",","]
//! [1.17,"o"," "]
//! [1.205,"o","W"]
//! [1.24,"o","o"]
//! [1.275,"o","r"]
//! [1.31,"o","l"]
//! [1.345,"o","d"]
//! [1.38,"o","!"]
//! [1.415,"o","'"]
//! [2.3,"o","\r\n"]
//! [2.3,"o","Hello, World!\r\n"]
//! [3.3,"o",""]
//! ~$
//! ```
//!
//! A string surrounded by double quotes, `"`, containing a shell command and
//! its arguments is specified as the input positional argument for the `germ`
//! application. Here, the [`echo`] shell command is used to print the a string
//! to the terminal. Since no outputs are specified as additional positional
//! arguments _after_ the first input positional argument, the input string is
//! executed within a child shell process. The sequence generated from executing
//! the input string as a shell command and its captured output is written to
//! stdout.
//!
//! The default is to output a sequence using the [asciicast v2] file
//! format, which is contains [JSON] elements, but it is not exactly valid JSON.
//! This default has been selected so that the germ application quickly and
//! easily provides output that can be played back and/or shared within the
//! [asciinema.og] ecosystem. There is a header with various information useful
//! for playing back the cast file with the [asciinema player] and then a series
//! of events. Within the event stream, the input string is "typed" one
//! character at a time, while the output is "printed" as a single entity.
//!
//! The previous example executes the input to obtain the outputs, but it is
//! possible to skip the execution and explicitly define the outputs by adding
//! the outputs as additional positional arguments _after_ the first positional
//! input argument. The previous example can be re-created as follows without
//! executing the input as a shell comman:
//!
//! ```sh
//! ~$ germ "echo 'Hello, World!'" "'Hello, World!'"
//! {"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
//! [0.0,"o","$ "]
//! [0.75,"o","e"]
//! [0.785,"o","c"]
//! [0.82,"o","h"]
//! [0.855,"o","o"]
//! [0.89,"o"," "]
//! [0.925,"o","'"]
//! [0.96,"o","H"]
//! [0.995,"o","e"]
//! [1.03,"o","l"]
//! [1.065,"o","l"]
//! [1.1,"o","o"]
//! [1.135,"o",","]
//! [1.17,"o"," "]
//! [1.205,"o","W"]
//! [1.24,"o","o"]
//! [1.275,"o","r"]
//! [1.31,"o","l"]
//! [1.345,"o","d"]
//! [1.38,"o","!"]
//! [1.415,"o","'"]
//! [2.3,"o","\r\n"]
//! [2.3,"o","Hello, World!\r\n"]
//! [3.3,"o",""]
//! ~$
//! ```
//!
//! Note the single quotes around the output positional argument are needed
//! because of the exclamation mark, `!`, which needs to be escaped in most
//! shells. These examples demonstrate creating simple asciicast formatted
//! output from command line specified input and outputs but printing to stdout
//! is not very useful since the asciinema ecosystem uses asciicast files as
//! input and output. Thus, the previous two examples can be modified to save to
//! an asciicast file either using [redirection] or the `-o,--output-file` option.
//!
//! ```sh
//! ~$ germ "echo 'Hello, World!'" > example1.cast
//! ~$ germ -o example2.cast "echo 'Hello, world!'" "'Hello, World!'"
//! ~$ ls
//! example1.cast  example2.cast
//! ~$
//! ```
//!
//! Both of the cast files created in the above example can be replayed using
//! the `asciinema play example1.cast` or `asciinema play example2.cast`
//! commands if the asciinema application is installed. The cast files can be
//! uploaded to [asciinema.org] using the `asciinema upload example1.cast` and
//! `asciinema upload example2.cast` commands. Currently, the asciinema
//! application does not accept input via stdin. The file extension does _not_
//! need to be `.cast`. Any file extension can be used, but `.cast` is the most
//! common for asciicast files.
//!
//! Previous examples demonstrated creating an asciicast file from a single
//! input with zero or more outputs, but asciicast files based on multiple
//! inputs and their respective outputs can be created in a variety of ways. All
//! methods for creating multi-input cast files utilize the `germ` output
//! format, which can be selected using the `-O,--output-format` option or the
//! `-G` flag as a shortcut for `-O germ` because germ does not support parsing
//! asciicast files. Germ is only capable of creating/writing asciicast files.
//! An example of outputting germ's JSON-based format for sequences of inputs
//! and their respective outputs can be seen in the following:
//!
//! ```sh
//! ~$ germ --output-format germ "echo 'Hello, World!'"
//! {"version":1,"timings":{"begin":0.0,"end":1.0,"type_start":750,"type_char":35,"type_submit":350,"output_line":500,"speed":1.0},"commands":[{"prompt":"$ ","input":"echo 'Hello, World!'","outputs":["Hello, World!\n"]}]}
//! ```
//!
//! which is not very readable by a human, so let's pipe stdout from the germ
//! application into the stdin for the [jq] application to make the JSON more
//! human readable:
//!
//! ```sh
//! ~$ germ --output-format germ "echo 'Hello, World!'" | jq
//! {
//!   "version": 1,
//!   "timings": {
//!     "begin": 0,
//!     "end": 1,
//!     "type_start": 750,
//!     "type_char": 35,
//!     "type_submit": 350,
//!     "output_line": 500,
//!     "speed": 1
//!   },
//!   "commands": [
//!     {
//!       "prompt": "$ ",
//!       "input": "echo 'Hello, World!'",
//!       "outputs": [
//!         "Hello, World!\n"
//!       ]
//!     }
//!   ]
//! }
//! ```
//!
//! The JSON-based germ sequence format contains timing information for
//! generating a cast file as well as a list or array of all inputs and their
//! respective outputs. Germ is capable of reading, modifying, and writing
//! JSON-based sequence format. Furthermore, while asciinema does not read from
//! stdin, the germ application is capable of reading from stdin. The
//! combination of all of these features can be used to create multi-input cast
//! files.
//!
//! [asciinema]: https://asciinema.org
//! [TermSheets]: https://neatsoftware.github.io/term-sheets/
//! [Termyanl]: https://github.com/ines/termynal
//! [Typer]: https://typer.tiangolo.com/
//! ["Hello, World!" program]: https://en.wikipedia.org/wiki/%22Hello,_World!%22_program
//! [`echo`]: https://en.wikipedia.org/wiki/Echo_(command)
//! [asciicast v2]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
//! [JSON]: https://en.wikipedia.org/wiki/JSON
//! [asciinema.org]: https://asciinema.org
//! [asciinema player]: https://github.com/asciinema/asciinema-player
//! [redirection]: https://en.wikipedia.org/wiki/Redirection_(computing)
//! [jq]: https://stedolan.github.io/jq/

use anyhow::Result;
use germ::Cli;
use structopt::StructOpt;

fn main() -> Result<()> {
    Cli::from_args().execute()
}
