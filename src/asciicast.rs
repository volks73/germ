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
use serde::Serialize;

use crate::sequence::{Command, Sequence, Timings};
use std::env;
use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

pub const VERSION: usize = 2;
pub const DEFAULT_HEIGHT: &str = "24";

pub const DEFAULT_SHELL: &str = "/bin/sh";
pub const DEFAULT_TERM: &str = "xterm-256color";
pub const DEFAULT_WIDTH: &str = "80";
pub const MILLISECONDS_IN_A_SECOND: f64 = 1000.0;
pub const MILLISECONDS_UNITS: &str = "ms";
pub const SHELL_VAR_NAME: &str = "SHELL";
pub const TERM_VAR_NAME: &str = "TERM";

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum ExecuteStringFlags {
    Unix,
    Pwsh,
    Cmd,
}

impl fmt::Display for ExecuteStringFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unix => write!(f, "-c"),
            Self::Pwsh => write!(f, "-Command"),
            Self::Cmd => write!(f, "/c"),
        }
    }
}

impl Default for ExecuteStringFlags {
    fn default() -> Self {
        Self::Unix
    }
}

#[derive(Debug, Serialize, StructOpt)]
pub struct Env {
    /// The SHELL environment variable for the recording.
    #[structopt(short = "S", long, env = "SHELL", default_value = DEFAULT_SHELL)]
    #[serde(rename = "SHELL")]
    pub shell: String,

    /// The TERM environment variable for the recording.
    #[structopt(short = "T", long, env = "TERM", default_value = DEFAULT_TERM)]
    #[serde(rename = "TERM")]
    pub term: String,

    /// The flag passed to the shell to execute the string as a command.
    #[structopt(
        short = "E",
        long,
        possible_values = ExecuteStringFlags::VARIANTS,
        case_insensitive = true,
        value_name = "shell",
        env = "GERM_EXECUTE_STRING_FLAG",
        default_value = "unix"
    )]
    #[serde(skip)]
    pub execute_string_flag: ExecuteStringFlags,
}

impl Env {
    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn term(&self) -> &str {
        &self.term
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            shell: env::var_os(SHELL_VAR_NAME)
                .map(|s| String::from(s.to_string_lossy()))
                .unwrap_or_else(|| String::from(DEFAULT_SHELL)),
            term: env::var_os(TERM_VAR_NAME)
                .map(|s| String::from(s.to_string_lossy()))
                .unwrap_or_else(|| String::from(DEFAULT_TERM)),
            execute_string_flag: ExecuteStringFlags::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Theme {
    #[serde(rename = "fg")]
    pub foreground: String,

    #[serde(rename = "bg")]
    pub background: String,

    pub palette: String,
}

#[derive(Debug, Serialize, StructOpt)]
pub struct Header {
    #[structopt(skip = VERSION)]
    version: usize,

    /// The number of columns for the terminal.
    #[structopt(short = "W", long, default_value = DEFAULT_WIDTH, value_name = "cols")]
    pub width: usize,

    /// The number of rows for the terminal.
    #[structopt(
        short = "H",
        long = "height",
        default_value = DEFAULT_HEIGHT,
        value_name = "rows"
    )]
    pub height: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(skip)]
    pub timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(skip)]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(skip)]
    pub idle_time_limit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(skip)]
    pub command: Option<String>,

    /// The title for the asciicast file.
    #[structopt(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[structopt(flatten)]
    pub env: Env,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(skip)]
    pub theme: Option<Theme>,
}

impl Header {
    pub fn write_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        serde_json::to_writer(&mut writer, self)?;
        writeln!(&mut writer)?;
        Ok(())
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: VERSION,
            width: DEFAULT_WIDTH.parse().expect("Default usize"),
            height: DEFAULT_HEIGHT.parse().expect("Default usize"),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs()),
            duration: None,
            idle_time_limit: None,
            command: None,
            title: None,
            env: Env::default(),
            theme: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum EventKind {
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
pub struct Event(pub f64, pub EventKind, pub String);

impl Event {
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

#[derive(Debug, StructOpt)]
pub struct Asciicast {
    #[structopt(flatten)]
    pub header: Header,

    #[structopt(skip)]
    events: Vec<Event>,

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(long)]
    pub stdin: bool,
}

impl Asciicast {
    pub fn add(&mut self, event: Event) -> &mut Self {
        self.events.push(event);
        self
    }

    pub fn append(&mut self, events: &mut Vec<Event>) -> &mut Self {
        self.events.append(events);
        self
    }

    pub fn append_from(&mut self, sequence: &Sequence) -> &mut Self {
        let start_delay = sequence
            .iter()
            .fold(sequence.timings().begin, |start_delay, command| {
                self.add_command(command, sequence.timings(), start_delay)
            });
        if sequence.timings().end.into_milliseconds() as usize != 0 {
            self.add(Event(
                start_delay + sequence.timings().end,
                EventKind::Printed,
                String::new(),
            ));
        }
        self
    }

    fn add_command(&mut self, command: &Command, timings: &Timings, start_delay: f64) -> f64 {
        if let Some(c) = command.comment() {
            let mut comment = c.to_owned();
            comment.push_str("\r\n");
            self.add(Event(start_delay, EventKind::Printed, comment));
        }
        self.add(Event(
            start_delay,
            EventKind::Printed,
            command.prompt().to_owned(),
        ));
        let input_time = ((timings.type_start
            + timings.type_char * command.input().len()
            + timings.type_submit) as f64)
            .speed(timings.speed)
            .into_seconds();
        for (i, c) in command.input().chars().map(|c| c.to_string()).enumerate() {
            let char_delay = start_delay
                + ((timings.type_start + timings.type_char * i) as f64)
                    .speed(timings.speed)
                    .into_seconds();
            if self.stdin {
                self.add(Event(char_delay, EventKind::Keypress, c.clone()));
            }
            self.add(Event(char_delay, EventKind::Printed, c));
        }
        for (i, output) in command.outputs().iter().enumerate() {
            let show_delay = start_delay
                + input_time
                + ((timings.output_line * (i + 1)) as f64)
                    .speed(timings.speed)
                    .into_seconds();
            if i == 0 {
                self.add(Event(show_delay, EventKind::Printed, String::from("\r\n")));
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                self.add(Event(show_delay, EventKind::Printed, output_data));
            }
        }
        let outputs_time = ((timings.output_line * command.outputs().len()) as f64)
            .speed(timings.speed)
            .into_seconds();
        start_delay + input_time + outputs_time
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn write_to<W: Write>(&mut self, mut writer: W) -> Result<()> {
        self.header.write_to(&mut writer)?;
        for event in self.events.iter_mut() {
            event.write_to(&mut writer)?;
        }
        Ok(())
    }
}

impl Default for Asciicast {
    fn default() -> Self {
        Self {
            header: Header::default(),
            events: Vec::new(),
            stdin: false,
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
