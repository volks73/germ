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

use anyhow::Result;
use atty::Stream;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::clap::{self, value_t};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

const ASCIICAST_VERSION: usize = 2;
const SEQUENCE_VERSION: usize = 1;
const SHELL_VAR_NAME: &str = "SHELL";
const TERM_VAR_NAME: &str = "TERM";
const DEFAULT_BEGIN_DELAY: &str = "0.0";
const DEFAULT_END_DELAY: &str = "1.0";
const DEFAULT_DELAY_TYPE_START: &str = "750";
const DEFAULT_DELAY_TYPE_CHAR: &str = "35";
const DEFAULT_DELAY_TYPE_SUBMIT: &str = "350";
const DEFAULT_DELAY_OUTPUT_LINE: &str = "500";
const DEFAULT_INTERACTIVE_PROMPT: &str = ">>> ";
const DEFAULT_PROMPT: &str = "$ ";
const DEFAULT_HEIGHT: usize = 55;
const DEFAULT_SHELL: &str = "/bin/bash";
const DEFAULT_TERM: &str = "xterm-256color";
const DEFAULT_WIDTH: usize = 188;
const MILLISECONDS_IN_A_SECOND: f64 = 1000.0;
const MILLISECONDS_UNITS: &str = "ms";
const SECONDS_UNITS: &str = "secs";

mod termsheets {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Command {
        input: String,
        output: Vec<String>,
    }

    impl From<super::Command> for Command {
        fn from(c: super::Command) -> Self {
            Self {
                input: c.input,
                output: c.outputs,
            }
        }
    }

    impl From<Command> for super::Command {
        fn from(c: Command) -> Self {
            Self {
                comment: None,
                prompt: String::from(DEFAULT_PROMPT),
                input: c.input,
                outputs: c.output,
            }
        }
    }

    impl From<Sequence> for Vec<Command> {
        fn from(s: Sequence) -> Self {
            s.into_iter().map(Command::from).collect()
        }
    }

    impl From<Vec<Command>> for Sequence {
        fn from(t: Vec<Command>) -> Self {
            Self {
                commands: t.into_iter().map(super::Command::from).collect(),
                ..Default::default()
            }
        }
    }
}

trait ApplySpeed {
    type Output;

    fn speed(self, speed: f64) -> Self::Output;
}

impl ApplySpeed for f64 {
    type Output = Self;

    fn speed(self, speed: f64) -> Self::Output {
        self / speed
    }
}

trait SecondsConversions {
    type Output;

    fn into_seconds(self) -> Self::Output;

    fn into_milliseconds(self) -> Self::Output;
}

impl SecondsConversions for f64 {
    type Output = Self;

    fn into_seconds(self) -> Self::Output {
        self / MILLISECONDS_IN_A_SECOND
    }

    fn into_milliseconds(self) -> Self::Output {
        self * MILLISECONDS_IN_A_SECOND
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Timings {
    begin: f64,         // seconds
    end: f64,           // seconds
    type_start: usize,  // milliseconds
    type_char: usize,   // milliseconds
    type_submit: usize, // milliseconds
    output_line: usize, // milliseconds
}

impl<'a> From<&'a Cli> for Timings {
    fn from(g: &'a Cli) -> Self {
        Self {
            begin: g.begin_delay,
            end: g.end_delay,
            type_start: g.delay_type_start,
            type_char: g.delay_type_char,
            type_submit: g.delay_type_submit,
            output_line: g.delay_output_line,
        }
    }
}

impl Default for Timings {
    fn default() -> Self {
        Self {
            begin: DEFAULT_BEGIN_DELAY.parse().expect("Default float"),
            end: DEFAULT_END_DELAY.parse().expect("Default float"),
            type_start: DEFAULT_DELAY_TYPE_START.parse().expect("Default usize"),
            type_char: DEFAULT_DELAY_TYPE_CHAR.parse().expect("Default usize"),
            type_submit: DEFAULT_DELAY_TYPE_SUBMIT.parse().expect("Default usize"),
            output_line: DEFAULT_DELAY_OUTPUT_LINE.parse().expect("Default usize"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Sequence {
    version: usize,
    timings: Timings,
    commands: Vec<Command>,
}

impl Sequence {
    fn iter(&self) -> impl Iterator<Item = &Command> {
        self.commands.iter()
    }

    fn into_iter(self) -> impl Iterator<Item = Command> {
        self.commands.into_iter()
    }

    fn add(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            version: SEQUENCE_VERSION,
            timings: Timings::default(),
            commands: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    prompt: String,
    input: String,
    outputs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Env {
    #[serde(rename = "SHELL")]
    shell: String,

    #[serde(rename = "TERM")]
    term: String,
}

impl Env {
    pub fn shell() -> String {
        env::var_os(SHELL_VAR_NAME)
            .map(|s| String::from(s.to_string_lossy()))
            .unwrap_or_else(|| String::from(DEFAULT_SHELL))
    }

    pub fn term() -> String {
        env::var_os(TERM_VAR_NAME)
            .map(|s| String::from(s.to_string_lossy()))
            .unwrap_or_else(|| String::from(DEFAULT_TERM))
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            shell: Self::shell(),
            term: Self::term(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Theme {
    #[serde(rename = "fg")]
    foreground: String,

    #[serde(rename = "bg")]
    background: String,

    palette: String,
}

#[derive(Debug, Serialize)]
struct Header<'a> {
    version: usize,
    width: usize,
    height: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    idle_time_limit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Env>,

    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<Theme>,
}

impl<'a> Header<'a> {
    pub fn write_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        serde_json::to_writer(&mut writer, self)?;
        writeln!(&mut writer)?;
        Ok(())
    }
}

impl<'a> Default for Header<'a> {
    fn default() -> Self {
        Self {
            version: ASCIICAST_VERSION,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs()),
            duration: None,
            idle_time_limit: None,
            command: None,
            title: None,
            env: Some(Env::default()),
            theme: None,
        }
    }
}

#[derive(Debug, Serialize)]
enum EventKind {
    #[serde(rename = "o")]
    Printed,

    #[serde(rename = "i")]
    Keypress,
}

impl fmt::Display for EventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Printed => write!(f, "o"),
            Self::Keypress => write!(f, "i"),
        }
    }
}

#[derive(Debug, Serialize)]
struct Event<'a>(f64, EventKind, &'a str);

impl<'a> Event<'a> {
    pub fn write_to<W>(&mut self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        self.0 = (self.0 * MILLISECONDS_IN_A_SECOND).trunc() / MILLISECONDS_IN_A_SECOND;
        serde_json::to_writer(&mut writer, self)?;
        writeln!(&mut writer)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Hold {
    duration: f64,
    start_delay: f64,
}

impl Hold {
    pub fn write_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        Event(self.start_delay + self.duration, EventKind::Printed, "").write_to(&mut writer)
    }
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum InputFormats {
    Germ,
    TermSheets,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum OutputFormats {
    Germ,
    TermSheets,
    Asciicast,
}

#[derive(Debug, StructOpt)]
#[structopt(setting(clap::AppSettings::NoBinaryName))]
struct Interactive {
    #[structopt(flatten)]
    cli: Cli,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate terminal session recording files without rehearsing and recording")]
struct Cli {
    /// The delay before starting the simulated typing for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long,
        default_value = DEFAULT_DELAY_TYPE_START,
        value_name = MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_START"
    )]
    delay_type_start: usize,

    /// The delay between simulating typing of characters for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long,
        default_value = DEFAULT_DELAY_TYPE_CHAR,
        value_name = MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_CHAR"
    )]
    delay_type_char: usize,

    /// The delay between the simulated typing and output printing.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long,
        default_value = DEFAULT_DELAY_TYPE_SUBMIT,
        value_name = MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_SUBMIT"
    )]
    delay_type_submit: usize,

    /// The delay between outputs for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long,
        default_value = DEFAULT_DELAY_OUTPUT_LINE,
        value_name = MILLISECONDS_UNITS,
        env = "GERM_DELAY_OUTPUT_LINE"
    )]
    delay_output_line: usize,

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

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(long)]
    stdin: bool,

    /// Speed up or slow down the animation by this factor.
    #[structopt(short = "s", long, default_value = "1.0", value_name = "float")]
    speed: f64,

    /// The SHELL environment variable for the recording.
    #[structopt(short = "S", long, env = "SHELL", default_value = DEFAULT_SHELL)]
    shell: String,

    /// The TERM environment variable for the recording.
    #[structopt(short = "T", long, env = "TERM", default_value = DEFAULT_TERM)]
    term: String,

    /// The number of columns for the terminal.
    #[structopt(short = "W", long, default_value = "188", value_name = "cols")]
    width: usize,

    /// The number of rows for the terminal.
    #[structopt(
        short = "H",
        long = "height",
        default_value = "55",
        value_name = "rows"
    )]
    height: usize,

    /// The delay before starting the animation.
    ///
    /// The units are in seconds (s).
    #[structopt(
        short = "B",
        long,
        default_value = DEFAULT_BEGIN_DELAY,
        value_name = SECONDS_UNITS,
        env = "GERM_BEGIN_DELAY"
    )]
    begin_delay: f64,

    /// The delay at the end of the animation.
    ///
    /// This is useful when looping/repeat is enabled and some time between
    /// iterations is needed and/or desired. Set the value to 0.0 if no hold is
    /// desired. The units are in seconds (s).
    #[structopt(
        short = "E",
        long,
        default_value = DEFAULT_END_DELAY,
        value_name = SECONDS_UNITS,
        env = "GERM_END_DELAY"
    )]
    end_delay: f64,

    /// The title for the asciicast file.
    #[structopt(short, long)]
    title: Option<String>,

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
        default_value = "germ",
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
        default_value = "asciicast",
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
            Ok(Sequence {
                timings: Timings::from(self),
                ..Default::default()
            })
        } else {
            let stdin = io::stdin();
            self.read_from(stdin)
        }
    }

    fn read_from<R: Read>(&self, r: R) -> Result<Sequence> {
        match self.input_format {
            InputFormats::Germ => serde_json::from_reader(r).map_err(anyhow::Error::from),
            InputFormats::TermSheets => {
                let termsheets: Vec<termsheets::Command> = serde_json::from_reader(r)?;
                Ok(Sequence {
                    timings: Timings::from(self),
                    commands: termsheets
                        .into_iter()
                        .map(|c| Command {
                            comment: None,
                            prompt: self.prompt.clone(),
                            ..Command::from(c)
                        })
                        .collect(),
                    ..Default::default()
                })
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
        let outputs = if self.outputs.is_empty() {
            let output = process::Command::new(Env::shell())
                .args(&["-c", input])
                .output()?;
            vec![std::str::from_utf8(&output.stdout)?.to_owned()]
        } else {
            self.outputs.clone()
        };
        sequence.add(Command {
            comment: self.comment.clone(),
            prompt: self.prompt.clone(),
            input: input.to_owned(),
            outputs,
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
            // TODO: Capture error and print instead of failing.
            let words = shellwords::split(&line.expect("stdin line"))?;
            // TODO: Add `--no-print` flag to app. This does not write the
            // explicit outputs or the output from the command execution.
            let matches = Interactive::clap().get_matches_from(words);
            if matches.occurrences_of("interactive-prompt") != 0 {
                self.interactive_prompt = value_t!(matches, "interactive-prompt", String).unwrap();
            }
            if matches.occurrences_of("begin-delay") != 0 {
                self.begin_delay = value_t!(matches, "begin-delay", f64).unwrap();
            }
            if matches.occurrences_of("delay-type-start") != 0 {
                self.delay_type_start = value_t!(matches, "delay-type-start", usize).unwrap();
            }
            if matches.occurrences_of("delay-type-char") != 0 {
                self.delay_type_char = value_t!(matches, "delay-type-char", usize).unwrap();
            }
            if matches.occurrences_of("delay-type-submit") != 0 {
                self.delay_type_submit = value_t!(matches, "delay-type-start", usize).unwrap();
            }
            if matches.occurrences_of("delay-output-line") != 0 {
                self.delay_output_line = value_t!(matches, "delay-output-line", usize).unwrap();
            }
            if matches.occurrences_of("end-delay") != 0 {
                self.end_delay = value_t!(matches, "end-delay", f64).unwrap();
            }
            if matches.occurrences_of("title") != 0 {
                self.title = value_t!(matches, "title", String).ok();
            }
            if matches.occurrences_of("width") != 0 {
                self.width = value_t!(matches, "width", usize).unwrap();
            }
            if matches.occurrences_of("height") != 0 {
                self.height = value_t!(matches, "height", usize).unwrap();
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
                self.speed = value_t!(matches, "speed", f64).unwrap();
            }
            if matches.occurrences_of("shell") != 0 {
                self.shell = value_t!(matches, "shell", String).unwrap();
            }
            if matches.occurrences_of("term") != 0 {
                self.shell = value_t!(matches, "shell", String).unwrap();
            }
            if matches.occurrences_of("stdin") != 0 {
                self.stdin = true;
            }
            if matches.occurrences_of("use-germ-format") != 0 {
                self.use_germ_format = true;
            }
            if matches.is_present("license") {
                print_license();
            } else if matches.is_present("warranty") {
                print_warranty();
            } else {
                if let Some(input) = matches.value_of("input") {
                    if matches.is_present("outputs") {
                        sequence.add(Command {
                            comment: matches.value_of("comment").map(String::from),
                            prompt: self.prompt.clone(),
                            input: input.to_owned(),
                            outputs: matches
                                .values_of("outputs")
                                .unwrap()
                                .map(String::from)
                                .collect(),
                        });
                    } else {
                        let output = process::Command::new(Env::shell())
                            .args(&["-c", &input])
                            .output()?;
                        sequence.add(Command {
                            comment: matches.value_of("comment").map(String::from),
                            prompt: self.prompt.clone(),
                            input: input.to_owned(),
                            outputs: vec![std::str::from_utf8(&output.stdout)?.to_owned()],
                        });
                        stdout.write_all(&output.stdout)?;
                    }
                }
            }
            stdout.write_all(self.interactive_prompt.as_bytes())?;
            stdout.flush()?;
        }
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn write(&self, sequence: Sequence) -> Result<()> {
        let mut writer: Box<dyn Write> = if let Some(output_file) = &self.output_file {
            Box::new(File::create(output_file)?)
        } else {
            Box::new(io::stdout())
        };
        match self.output_format {
            OutputFormats::Germ => {
                serde_json::to_writer(&mut writer, &sequence)?;
            }
            OutputFormats::TermSheets => {
                let termsheets: Vec<termsheets::Command> = sequence.into();
                serde_json::to_writer(&mut writer, &termsheets)?;
            }
            OutputFormats::Asciicast => {
                Header {
                    width: self.width,
                    height: self.height,
                    title: self.title.as_deref(),
                    env: Some(Env {
                        shell: self.shell.clone(),
                        term: self.term.clone(),
                    }),
                    ..Default::default()
                }
                .write_to(&mut writer)?;
                let start_delay = sequence
                    .iter()
                    .try_fold(self.begin_delay, |start_delay, command| {
                        self.write_command(command, start_delay, &mut writer)
                    })?;
                if self.end_delay.into_milliseconds() as usize != 0 {
                    Hold {
                        duration: self.end_delay,
                        start_delay,
                    }
                    .write_to(&mut writer)?;
                }
            }
        }
        Ok(())
    }

    fn write_command<W>(&self, command: &Command, start_delay: f64, mut writer: W) -> Result<f64>
    where
        W: Write,
    {
        if let Some(mut comment) = command.comment.clone() {
            comment.push_str("\r\n");
            Event(start_delay, EventKind::Printed, &comment).write_to(&mut writer)?;
        }
        Event(start_delay, EventKind::Printed, &command.prompt).write_to(&mut writer)?;
        let input_time = ((self.delay_type_start
            + self.delay_type_char * command.input.len()
            + self.delay_type_submit) as f64)
            .speed(self.speed)
            .into_seconds();
        for (i, c) in command.input.chars().map(|c| c.to_string()).enumerate() {
            let char_delay = start_delay
                + ((self.delay_type_start + self.delay_type_char * i) as f64)
                    .speed(self.speed)
                    .into_seconds();
            if self.stdin {
                Event(char_delay, EventKind::Keypress, &c).write_to(&mut writer)?;
            }
            Event(char_delay, EventKind::Printed, &c).write_to(&mut writer)?;
        }
        for (i, output) in command.outputs.iter().enumerate() {
            let show_delay = start_delay
                + input_time
                + ((self.delay_output_line * (i + 1)) as f64)
                    .speed(self.speed)
                    .into_seconds();
            if i == 0 {
                Event(show_delay, EventKind::Printed, "\r\n").write_to(&mut writer)?;
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                Event(show_delay, EventKind::Printed, &output_data).write_to(&mut writer)?;
            }
        }
        let outputs_time = ((self.delay_output_line * command.outputs.len()) as f64)
            .speed(self.speed)
            .into_seconds();
        Ok(start_delay + input_time + outputs_time)
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
