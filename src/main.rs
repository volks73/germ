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
//! [asciinema]: https://asciinema.org
//! [TermSheets]: https://neatsoftware.github.io/term-sheets/
//! [Termyanl]: https://github.com/ines/termynal
//! [Type]: https://typer.tiangolo.com/
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
//! shells.
//!
//! ["Hello, World!" program]: https://en.wikipedia.org/wiki/%22Hello,_World!%22_program
//! [`echo`]: https://en.wikipedia.org/wiki/Echo_(command)
//! [asciicast v2]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
//! [JSON]: https://en.wikipedia.org/wiki/JSON
//! [asciinema.org]: https://asciinema.org
//! [asciinema player]: https://github.com/asciinema/asciinema-player

use anyhow::Result;
use atty::Stream;
use germ::asciicast::Asciicast;
use germ::sequence::{Command, Sequence, Timings, DEFAULT_PROMPT};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process;
use structopt::clap::{self, value_t, ArgMatches};
use structopt::StructOpt;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

pub const DEFAULT_INTERACTIVE_PROMPT: &str = ">>> ";

#[derive(Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum InputFormats {
    Germ,
    TermSheets,
}

impl Default for InputFormats {
    fn default() -> Self {
        Self::Germ
    }
}

#[derive(Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum OutputFormats {
    Germ,
    TermSheets,
    Asciicast,
}

impl Default for OutputFormats {
    fn default() -> Self {
        Self::Asciicast
    }
}

#[derive(Debug, StructOpt)]
#[structopt(settings(&[
    clap::AppSettings::NoBinaryName,
    clap::AppSettings::DisableHelpFlags,
    clap::AppSettings::DisableVersion,
    clap::AppSettings::NextLineHelp]),
    usage("[FLAGS] [OPTIONS] [INPUT] [OUTPUTS...]")
)]
struct Interactive {
    #[structopt(flatten)]
    cli: Cli,

    /// Prints the current sequence.
    ///
    /// The format is determined by the last output format used.
    #[structopt(long)]
    print: bool,

    /// Prints help information.
    #[structopt(short = "h")]
    short_help: bool,

    /// Prints more help information.
    #[structopt(long = "help")]
    long_help: bool,

    /// Prints version information.
    #[structopt(short = "V", long = "version")]
    version: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate terminal session recording files without rehearsing and recording")]
struct Cli {
    #[structopt(flatten)]
    timings: Timings,

    #[structopt(flatten)]
    asciicast: Asciicast,

    /// A comment about the command.
    ///
    /// A line will be "printed" in the terminal session above the prompt and input.
    #[structopt(short, long)]
    comment: Option<String>,

    /// The prompt to display before the command.
    #[structopt(short = "p", long, default_value = DEFAULT_PROMPT, env = "GERM_PROMPT")]
    prompt: String,

    /// The prompt displayed in interactive mode.
    #[structopt(short ="P", long, default_value = DEFAULT_INTERACTIVE_PROMPT, env = "GERM_INTERACTIVE_PROMPT")]
    interactive_prompt: String,

    /// Use the Germ JSON format for the output.
    ///
    /// This is equivalent to '-O,--output-format germ'.
    #[structopt(short = "G")]
    use_germ_format: bool,

    /// Prints the license information.
    ///
    /// This is as recommended by the GPL-3.0 license.
    #[structopt(long)]
    license: bool,

    /// Prints the warranty information.
    ///
    /// This is as recommended by the GPL-3.0 license.
    #[structopt(long)]
    warranty: bool,

    /// The format of the input.
    #[structopt(
        short = "I",
        long,
        possible_values = InputFormats::VARIANTS,
        case_insensitive = true,
        default_value,
        value_name = "format",
        env = "GERM_INPUT_FORMAT"
    )]
    input_format: InputFormats,

    /// Input file in the commands JSON format.
    ///
    /// If not present, then stdin if it is piped or redirected.
    #[structopt(short = "i", long = "input", value_name("file"), parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// The format for the output.
    #[structopt(
        short = "O",
        long,
        possible_values = OutputFormats::VARIANTS,
        case_insensitive = true,
        default_value,
        default_value_if("use-germ-format", None, "germ"),
        value_name = "format",
        env = "GERM_OUTPUT_FORMAT"
    )]
    output_format: OutputFormats,

    /// Output file, stdout if not present.
    ///
    /// This is useful if using the application in interactive mode.
    #[structopt(short = "o", long = "output", value_name("file"), parse(from_os_str))]
    output_file: Option<PathBuf>,

    /// The command entered at the prompt.
    ///
    /// If not present and the -i,--input option is not used, then the
    /// application enters an interactive mode where commands are manually
    /// entered one at a time within the terminal. If present, then the command
    /// is appended to the sequence of commands from any input file or stdin.
    ///
    /// Note, if present without any output, then the input will be executed
    /// within a child shell process and the execution output will be used.
    input: Option<String>,

    /// Output from the command.
    ///
    /// If no output is provided, then the input will be execute within a child
    /// shell process and execution output will be used.
    outputs: Vec<String>,
}

impl Cli {
    pub fn execute(mut self) -> Result<()> {
        if self.license {
            print_license();
            return Ok(());
        }
        if self.warranty {
            print_warranty();
            return Ok(());
        }
        let mut sequence = self.read()?;
        self.append(&mut sequence)?;
        self.write(sequence)
    }

    fn read(&self) -> Result<Sequence> {
        if let Some(input_file) = &self.input_file {
            self.read_from(BufReader::new(File::open(input_file)?))
        } else if atty::is(Stream::Stdin) {
            Ok(Sequence::from(self.timings))
        } else {
            let stdin = io::stdin();
            self.read_from(stdin)
        }
    }

    fn read_from<R: Read>(&self, r: R) -> Result<Sequence> {
        match self.input_format {
            InputFormats::Germ => serde_json::from_reader(r).map_err(anyhow::Error::from),
            InputFormats::TermSheets => {
                let termsheets: Vec<germ::termsheets::Command> = serde_json::from_reader(r)?;
                let mut sequence = Sequence::from(self.timings);
                sequence.append(
                    &mut termsheets
                        .into_iter()
                        .map(|c| {
                            let mut cmd = Command::from(c);
                            cmd.set_prompt(&self.prompt);
                            cmd
                        })
                        .collect(),
                );
                Ok(sequence)
            }
        }
    }

    fn append(&mut self, sequence: &mut Sequence) -> Result<()> {
        if let Some(input) = self.input.as_ref() {
            self.append_arguments(sequence, input)
        } else if self.input_file.is_none() && atty::is(Stream::Stdin) {
            self.append_interactively(sequence)
        } else {
            Ok(())
        }
    }

    fn append_arguments(&self, sequence: &mut Sequence, input: &str) -> Result<()> {
        let mut outputs = if self.outputs.is_empty() {
            let output = self.execute_cmd(input)?;
            vec![std::str::from_utf8(&output.stdout)?.to_owned()]
        } else {
            self.outputs.clone()
        };
        sequence.add({
            let mut cmd = Command::from(input);
            cmd.set_comment(self.comment.as_deref());
            cmd.set_prompt(&self.prompt);
            cmd.append(&mut outputs);
            cmd
        });
        Ok(())
    }

    fn append_interactively(&mut self, sequence: &mut Sequence) -> Result<()> {
        print_interactive_notice();
        println!();
        let mut stdout = io::stdout();
        stdout.write_all(self.interactive_prompt.as_bytes())?;
        stdout.flush()?;
        for line in io::stdin().lock().lines() {
            let words = shellwords::split(&line.expect("stdin line"))?;
            let mut app = Interactive::clap();
            match app.get_matches_from_safe_borrow(words) {
                Ok(matches) => {
                    if matches.is_present("short-help") {
                        app.write_help(&mut stdout)?;
                        stdout.write_all(b"\n")?;
                    } else if matches.is_present("long-help") {
                        app.write_long_help(&mut stdout)?;
                        stdout.write_all(b"\n")?;
                    } else if matches.is_present("short-version") {
                        app.write_version(&mut stdout)?;
                        stdout.write_all(b"\n")?;
                    } else if matches.is_present("long-version") {
                        app.write_long_version(&mut stdout)?;
                        stdout.write_all(b"\n")?;
                    } else if matches.is_present("license") {
                        print_license();
                    } else if matches.is_present("warranty") {
                        print_warranty();
                    } else if matches.is_present("print") {
                        self.write_to(&mut stdout, &sequence)?;
                        if !matches!(self.output_format, OutputFormats::Asciicast) {
                            stdout.write_all(b"\n")?;
                        }
                    } else {
                        self.update_from(&matches);
                        if let Some(input_file) = matches.value_of("input-file").map(PathBuf::from)
                        {
                            sequence.append_from(
                                self.read_from(BufReader::new(File::open(input_file)?))?,
                            );
                        }
                        if let Some(input) = matches.value_of("input") {
                            let mut outputs = if matches.is_present("outputs") {
                                matches
                                    .values_of("outputs")
                                    .unwrap()
                                    .map(String::from)
                                    .collect()
                            } else {
                                let output = self.execute_cmd(&input)?;
                                stdout.write_all(&output.stdout)?;
                                vec![std::str::from_utf8(&output.stdout)?.to_owned()]
                            };
                            sequence.add({
                                let mut cmd = Command::from(input);
                                cmd.set_comment(
                                    matches.value_of("comment").map(String::from).as_deref(),
                                );
                                cmd.set_prompt(&self.prompt);
                                cmd.append(&mut outputs);
                                cmd
                            });
                        }
                    }
                }
                Err(err) => eprintln!("{}", err),
            }
            stdout.write_all(self.interactive_prompt.as_bytes())?;
            stdout.flush()?;
        }
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn write(&mut self, sequence: Sequence) -> Result<()> {
        let writer: Box<dyn Write> = if let Some(output_file) = &self.output_file {
            Box::new(File::create(output_file)?)
        } else {
            Box::new(io::stdout())
        };
        self.write_to(writer, &sequence)
    }

    fn write_to<W: Write>(&mut self, mut writer: W, sequence: &Sequence) -> Result<()> {
        match self.output_format {
            OutputFormats::Germ => {
                serde_json::to_writer(&mut writer, &sequence)?;
            }
            OutputFormats::TermSheets => {
                let termsheets: Vec<germ::termsheets::Command> = sequence.into();
                serde_json::to_writer(&mut writer, &termsheets)?;
            }
            OutputFormats::Asciicast => {
                self.asciicast
                    .append_from(&sequence)
                    .write_to(&mut writer)?;
            }
        }
        Ok(())
    }

    fn update_from(&mut self, matches: &ArgMatches) {
        if matches.occurrences_of("interactive-prompt") != 0 {
            self.interactive_prompt = value_t!(matches, "interactive-prompt", String).unwrap();
        }
        if matches.occurrences_of("begin-delay") != 0 {
            self.timings.begin = value_t!(matches, "begin-delay", f64).unwrap();
        }
        if matches.occurrences_of("delay-type-start") != 0 {
            self.timings.type_start = value_t!(matches, "delay-type-start", usize).unwrap();
        }
        if matches.occurrences_of("delay-type-char") != 0 {
            self.timings.type_char = value_t!(matches, "delay-type-char", usize).unwrap();
        }
        if matches.occurrences_of("delay-type-submit") != 0 {
            self.timings.type_submit = value_t!(matches, "delay-type-start", usize).unwrap();
        }
        if matches.occurrences_of("delay-output-line") != 0 {
            self.timings.output_line = value_t!(matches, "delay-output-line", usize).unwrap();
        }
        if matches.occurrences_of("end-delay") != 0 {
            self.timings.end = value_t!(matches, "end-delay", f64).unwrap();
        }
        if matches.occurrences_of("title") != 0 {
            self.asciicast.header.title = value_t!(matches, "title", String).ok();
        }
        if matches.occurrences_of("width") != 0 {
            self.asciicast.header.width = value_t!(matches, "width", usize).unwrap();
        }
        if matches.occurrences_of("height") != 0 {
            self.asciicast.header.height = value_t!(matches, "height", usize).unwrap();
        }
        if matches.occurrences_of("input-format") != 0 {
            self.input_format = value_t!(matches, "input-format", InputFormats).unwrap();
        }
        if matches.occurrences_of("output-format") != 0 {
            self.output_format = value_t!(matches, "output-format", OutputFormats).unwrap();
        }
        if matches.occurrences_of("output-file") != 0 {
            self.output_file = value_t!(matches, "output-file", PathBuf).ok();
        }
        if matches.occurrences_of("prompt") != 0 {
            self.prompt = value_t!(matches, "prompt", String).unwrap();
        }
        if matches.occurrences_of("speed") != 0 {
            self.timings.speed = value_t!(matches, "speed", f64).unwrap();
        }
        if matches.occurrences_of("shell") != 0 {
            self.asciicast.header.env.shell = value_t!(matches, "shell", String).unwrap();
        }
        if matches.occurrences_of("term") != 0 {
            self.asciicast.header.env.term = value_t!(matches, "shell", String).unwrap();
        }
        if matches.occurrences_of("stdin") != 0 {
            self.asciicast.stdin = true;
        }
        if matches.occurrences_of("use-germ-format") != 0 {
            self.use_germ_format = true;
        }
    }

    fn execute_cmd(&self, input: &str) -> Result<process::Output> {
        process::Command::new(self.asciicast.header.env.shell())
            .args(&[
                &format!("{}", self.asciicast.header.env.execute_string_flag),
                input,
            ])
            .output()
            .map_err(anyhow::Error::from)
    }
}

fn print_interactive_notice() {
    println!(
        r#"Copyright (C) 2021  Christopher R. Field
This program comes with ABSOLUTELY NO WARRANTY; for details use the `--warranty`
flag. This is free software, and you are welcome to redistirbute it under
certain conditions; use the `--license` flag for details.

You have entered interactive mode. The prompt has similar arguments, options,
flags, and functionality to the command line interface. Use the --help flag to
print the help text.

Type CTRL+D (^D) to exit and generate output or CTRL+C (^C) to abort."#
    )
}

fn print_warranty() {
    println!(
        r#"THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY
APPLICABLE LAW.  EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT
HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM "AS IS" WITHOUT WARRANTY
OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO,
THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM
IS WITH YOU.  SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF
ALL NECESSARY SERVICING, REPAIR OR CORRECTION."#
    )
}

fn print_license() {
    println!(
        r#"Copyright (C) 2021  Christopher R. Field

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>."#
    )
}

fn main() -> Result<()> {
    Cli::from_args().execute()
}
