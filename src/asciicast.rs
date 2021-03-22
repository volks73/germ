use anyhow::Result;
use serde::Serialize;

use crate::{MILLISECONDS_IN_A_SECOND, SHELL_VAR_NAME, TERM_VAR_NAME};
use std::env;
use std::fmt;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub const VERSION: usize = 2;
pub const DEFAULT_HEIGHT: usize = 55;
pub const DEFAULT_SHELL: &str = "/bin/bash";
pub const DEFAULT_TERM: &str = "xterm-256color";
pub const DEFAULT_WIDTH: usize = 188;

#[derive(Debug, Serialize)]
pub struct Env {
    #[serde(rename = "SHELL")]
    pub shell: String,

    #[serde(rename = "TERM")]
    pub term: String,
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
pub struct Theme {
    #[serde(rename = "fg")]
    pub foreground: String,

    #[serde(rename = "bg")]
    pub background: String,

    pub palette: String,
}

#[derive(Debug, Serialize)]
pub struct Header {
    pub version: usize,
    pub width: usize,
    pub height: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_time_limit: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
            env: Some(Env::default()),
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
pub struct Event<'a>(pub f64, pub EventKind, pub &'a str);

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
pub struct Hold {
    pub duration: f64,
    pub start_delay: f64,
}

impl Hold {
    pub fn write_to<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        Event(self.start_delay + self.duration, EventKind::Printed, "").write_to(&mut writer)
    }
}
