use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use env_logger::fmt::Color as LogColor;
use env_logger::Builder;
use log::Level;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

const VERSION: usize = 2;
const WIDTH: usize = 188;
const HEIGHT: usize = 55;
const SHELL: &str = "/bin/bash";
const TERM: &str = "xterm-256color";
const DELAY_TYPE_START: usize = 750;
const DELAY_TYPE_CHAR: usize = 35;
const DELAY_TYPE_SUBMIT: usize = 350;
const DELAY_OUTPUT_LINE: usize = 500;

#[derive(Debug, Deserialize, Serialize)]
struct Command {
    input: String,

    #[serde(rename = "output")]
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
struct Header {
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
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Env>,

    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<Theme>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            version: VERSION,
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
    Received,

    #[allow(dead_code)]
    #[serde(rename = "i")]
    Sent,
}

impl Default for EventKind {
    fn default() -> Self {
        EventKind::Received
    }
}

#[derive(Debug, Serialize)]
struct Event(f64, EventKind, String);

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate asciinema cast files without recording")]
struct Args {
    #[structopt(flatten)]
    verbose: Verbosity,

    /// Input file
    #[structopt(short = "i", long = "input", value_name("FILE"), parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", value_name("FILE"), parse(from_os_str))]
    output_file: Option<PathBuf>,

    /// Input command
    #[structopt(requires("outputs"))]
    input: Option<String>,

    /// Output from input command
    #[structopt(min_values = 1)]
    outputs: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    if let Some(level) = args.verbose.log_level() {
        Builder::new()
            .format(|buf, record| {
                // This implmentation for a format is copied from the default format implemented for the
                // `env_logger` crate but modified to use a colon, `:`, to separate the level from the
                // message and change the colors to match the previous colors used by the `loggerv` crate.
                let mut level_style = buf.style();
                let level = record.level();
                match level {
                    // Light Gray, or just Gray, is not a supported color for non-ANSI enabled Windows
                    // consoles, so TRACE and DEBUG statements are differentiated by boldness but use the
                    // same white color.
                    Level::Trace => level_style.set_color(LogColor::White).set_bold(false),
                    Level::Debug => level_style.set_color(LogColor::White).set_bold(true),
                    Level::Info => level_style.set_color(LogColor::Green).set_bold(true),
                    Level::Warn => level_style.set_color(LogColor::Yellow).set_bold(true),
                    Level::Error => level_style.set_color(LogColor::Red).set_bold(true),
                };
                let write_level = write!(buf, "{:>5}: ", level_style.value(level));
                let write_args = writeln!(buf, "{}", record.args());
                write_level.and(write_args)
            })
            .filter(Some("simple-cast-gen"), level.to_level_filter())
            .filter(None, Level::Warn.to_level_filter())
            .try_init()?;
    }
    let mut start_delay = 0;
    let speed = 1;
    let commands: Vec<Command> = if let Some(input_file) = args.input_file {
        let file = File::open(input_file)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    } else {
        Ok(vec![Command {
            input: args.input.expect("Input positional argument"),
            outputs: args.outputs,
        }])
    }?;
    let mut stdout = io::stdout();
    serde_json::to_writer(&mut stdout, &Header::default())?;
    writeln!(&mut stdout)?;
    serde_json::to_writer(
        &mut stdout,
        &Event(
            start_delay as f64 / 1000.0,
            EventKind::default(),
            String::from("~$ "),
        ),
    )?;
    writeln!(&mut stdout)?;
    for command in commands.into_iter() {
        let input_time =
            (DELAY_TYPE_START + DELAY_TYPE_CHAR * command.input.len() + DELAY_TYPE_SUBMIT) / speed;
        for (i, c) in command.input.chars().enumerate() {
            let char_delay =
                (start_delay + DELAY_TYPE_START / speed + (DELAY_TYPE_CHAR * i) / speed) as f64
                    / 1000.0;
            serde_json::to_writer(
                &mut stdout,
                &Event(char_delay, EventKind::default(), String::from(c)),
            )?;
            writeln!(&mut stdout)?;
        }
        for (i, output) in command.outputs.iter().enumerate() {
            let show_delay =
                (start_delay + input_time + (DELAY_OUTPUT_LINE * (i + 1)) / speed) as f64 / 1000.0;
            if i == 0 {
                serde_json::to_writer(
                    &mut stdout,
                    &Event(show_delay, EventKind::default(), String::from("\r\n")),
                )?;
                writeln!(&mut stdout)?;
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                serde_json::to_writer(
                    &mut stdout,
                    &Event(show_delay, EventKind::default(), output_data),
                )?;
                writeln!(&mut stdout)?;
            }
        }
        let outputs_time = if command.outputs.is_empty() {
            0
        } else {
            (DELAY_OUTPUT_LINE * command.outputs.len()) / speed
        };
        start_delay = start_delay + input_time + outputs_time;
    }
    Ok(())
}
