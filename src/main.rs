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
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{self, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

const ASCIICAST_VERSION: usize = 2;
const SEQUENCE_VERSION: usize = 1;
const WIDTH: usize = 188;
const HEIGHT: usize = 55;
const SHELL: &str = "/bin/bash";
const TERM: &str = "xterm-256color";
const MILLISECONDS_IN_A_SECOND: f64 = 1000.0;

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
                prompt: String::from("~$ "),
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
struct Sequence {
    version: usize,
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
            commands: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Command {
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

impl Default for Env {
    fn default() -> Self {
        Self {
            shell: env::var_os("SHELL")
                .map(|s| String::from(s.to_string_lossy()))
                .unwrap_or_else(|| String::from(SHELL)),
            term: env::var_os("TERM")
                .map(|s| String::from(s.to_string_lossy()))
                .unwrap_or_else(|| String::from(TERM)),
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
    pub fn to_writer<W>(&self, mut writer: W) -> Result<()>
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
            width: WIDTH,
            height: HEIGHT,
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
    pub fn to_writer<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
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
    pub fn to_writer<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        Event(self.start_delay + self.duration, EventKind::Printed, "").to_writer(&mut writer)
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
#[structopt(
    about = "Generate termainl session recording files without using rehearsing and recording"
)]
struct Germ {
    /// The delay before starting the simulated typing for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(long, default_value = "750", value_name = "ms")]
    delay_type_start: usize,

    /// The delay between simulating typing of characters for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(long, default_value = "35", value_name = "ms")]
    delay_type_char: usize,

    /// The delay between the simulated typing and output printing.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(long, default_value = "350", value_name = "ms")]
    delay_type_submit: usize,

    /// The delay between outputs for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(long, default_value = "500", value_name = "ms")]
    delay_output_line: usize,

    /// The prompt to display before the command.
    #[structopt(short = "p", long, default_value = "~$ ")]
    prompt: String,

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(long)]
    stdin: bool,

    /// Speed up or slow down the animation by this factor.
    #[structopt(short = "s", long, default_value = "1.0", value_name = "float")]
    speed: f64,

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
    #[structopt(short = "B", long, default_value = "0.0", value_name = "secs")]
    begin_delay: f64,

    /// The delay at the end of the animation.
    ///
    /// This is useful when looping/repeat is enabled and some time between
    /// iterations is needed and/or desired. Set the value to 0.0 if no hold is
    /// desired. The units are in seconds (s).
    #[structopt(short = "E", long, default_value = "2.0", value_name = "secs")]
    end_delay: f64,

    /// The title for the asciicast file.
    #[structopt(short = "T", long = "title")]
    title: Option<String>,

    /// Use the Germ JSON format for the output.
    ///
    /// This is equivalent to '-O,--output-format germ'.
    #[structopt(short = "G")]
    use_germ_format: bool,

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
        value_name = "format"
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
        value_name = "format"
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
    /// entered one at a time within the terminal.
    #[structopt(requires("outputs"))]
    input: Option<String>,

    /// Output from the command.
    ///
    /// A command can have multiple output.
    #[structopt(min_values = 1)]
    outputs: Vec<String>,
}

impl Germ {
    pub fn execute(self) -> Result<()> {
        if self.warranty {
            print_warranty();
            return Ok(());
        }
        let mut sequence = if let Some(input_file) = &self.input_file {
            let buf = BufReader::new(File::open(input_file)?);
            match self.input_format {
                InputFormats::Germ => serde_json::from_reader(buf),
                InputFormats::TermSheets => {
                    let termsheets: Vec<termsheets::Command> = serde_json::from_reader(buf)?;
                    Ok(Sequence::from(termsheets))
                }
            }
        } else if atty::is(Stream::Stdin) {
            Ok(Sequence::default())
        } else {
            let stdin = io::stdin();
            match self.input_format {
                InputFormats::Germ => serde_json::from_reader(stdin),
                InputFormats::TermSheets => {
                    let termsheets: Vec<termsheets::Command> = serde_json::from_reader(stdin)?;
                    Ok(Sequence::from(termsheets))
                }
            }
        }?;
        if let Some(input) = self.input.as_ref() {
            sequence.add(Command {
                prompt: self.prompt.clone(),
                input: input.clone(),
                outputs: self.outputs.clone(),
            });
        } else if self.input_file.is_none() && atty::is(Stream::Stdin) {
            let mut line = String::new();
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            loop {
                if stdin.lock().read_line(&mut line)? == 0 {
                    break;
                } else {
                    let trimmed_line = line.trim();
                    let mut child = process::Command::new("sh")
                        .stdin(Stdio::piped())
                        .stderr(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()?;
                    child
                        .stdin
                        .as_mut()
                        .expect("Child process stdin to be captured")
                        .write_all(trimmed_line.as_bytes())?;
                    let output = child.wait_with_output()?;
                    sequence.add(Command {
                        prompt: self.prompt.clone(),
                        input: trimmed_line.to_owned(),
                        outputs: vec![std::str::from_utf8(&output.stdout)?.to_owned()],
                    });
                    stdout.write_all(&output.stdout)?;
                }
            }
        }
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
                    ..Default::default()
                }
                .to_writer(&mut writer)?;
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
                    .to_writer(&mut writer)?;
                }
            }
        }
        Ok(())
    }

    fn write_command<W>(&self, command: &Command, start_delay: f64, mut writer: W) -> Result<f64>
    where
        W: Write,
    {
        Event(start_delay, EventKind::Printed, &command.prompt).to_writer(&mut writer)?;
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
                Event(char_delay, EventKind::Keypress, &c).to_writer(&mut writer)?;
            }
            Event(char_delay, EventKind::Printed, &c).to_writer(&mut writer)?;
        }
        for (i, output) in command.outputs.iter().enumerate() {
            let show_delay = start_delay
                + input_time
                + ((self.delay_output_line * (i + 1)) as f64)
                    .speed(self.speed)
                    .into_seconds();
            if i == 0 {
                Event(show_delay, EventKind::Printed, "\r\n").to_writer(&mut writer)?;
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                Event(show_delay, EventKind::Printed, &output_data).to_writer(&mut writer)?;
            }
        }
        let outputs_time = ((self.delay_output_line * command.outputs.len()) as f64)
            .speed(self.speed)
            .into_seconds();
        Ok(start_delay + input_time + outputs_time)
    }
}

fn main() -> Result<()> {
    Germ::from_args().execute()
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
