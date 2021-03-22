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

use crate::{MILLISECONDS_IN_A_SECOND, SHELL_VAR_NAME, TERM_VAR_NAME};
use std::env;
use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

pub const VERSION: usize = 2;
pub const DEFAULT_HEIGHT: usize = 55;
pub const DEFAULT_SHELL: &str = "/bin/bash";
pub const DEFAULT_TERM: &str = "xterm-256color";
pub const DEFAULT_WIDTH: usize = 188;

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
    #[structopt(skip)]
    pub version: usize,

    /// The number of columns for the terminal.
    #[structopt(short = "W", long, default_value = "188", value_name = "cols")]
    pub width: usize,

    /// The number of rows for the terminal.
    #[structopt(
        short = "H",
        long = "height",
        default_value = "55",
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

#[derive(Debug)]
pub struct Hold {
    pub duration: f64,
    pub start_delay: f64,
}

impl Hold {
    pub fn write_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        Event(
            self.start_delay + self.duration,
            EventKind::Printed,
            String::new(),
        )
        .write_to(&mut writer)
    }
}

#[derive(Debug, StructOpt)]
pub struct Asciicast {
    #[structopt(flatten)]
    pub header: Header,

    #[structopt(skip)]
    pub event_stream: Vec<Event>,

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(long)]
    pub stdin: bool,
}
