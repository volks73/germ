use anyhow::Result;
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

impl Header {
    pub fn to_writer<W>(&self, mut writer: W) -> Result<()>
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

    #[allow(dead_code)]
    #[serde(rename = "i")]
    Keypress,
}

impl Default for EventKind {
    fn default() -> Self {
        EventKind::Printed
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
struct Prompt<'a> {
    content: &'a str,
    start_delay: usize,
}

impl<'a> Prompt<'a> {
    pub fn to_writer<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        Event(
            self.start_delay as f64 / 1000.0,
            EventKind::default(),
            self.content,
        )
        .to_writer(&mut writer)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate asciicast files without using asciinema's recording functionality")]
struct AsciicastGen {
    /// The prompt to display before the command.
    #[structopt(short = "p", long = "prompt", default_value = "~$ ")]
    prompt: String,

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(short, long)]
    stdin: bool,

    #[structopt(short = "s", long = "speed")]
    speed: usize,

    #[structopt(short = "S", long = "start-delay", default_value = "0")]
    start_delay: usize,

    /// Input file
    #[structopt(short = "i", long = "input", value_name("FILE"), parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", value_name("FILE"), parse(from_os_str))]
    output_file: Option<PathBuf>,

    /// The command entered at the prompt
    #[structopt(requires("outputs"))]
    input: Option<String>,

    /// Output from the command
    #[structopt(min_values = 1)]
    outputs: Vec<String>,
}

impl AsciicastGen {
    pub fn execute(self) -> Result<()> {
        let commands: Vec<Command> = if let Some(input_file) = &self.input_file {
            let file = File::open(input_file)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)
        } else {
            Ok(vec![Command {
                input: self.input.clone().expect("Input positional argument"),
                outputs: self.outputs.clone(),
            }])
        }?;
        let mut writer: Box<dyn Write> = if let Some(output_file) = &self.output_file {
            Box::new(File::create(output_file)?)
        } else {
            Box::new(io::stdout())
        };
        Header::default().to_writer(&mut writer)?;
        commands
            .iter()
            .try_fold(self.start_delay, |start_delay, command| {
                self.write_command(command, start_delay, &mut writer)
            })?;
        Ok(())
    }

    fn write_command<W>(
        &self,
        command: &Command,
        start_delay: usize,
        mut writer: W,
    ) -> Result<usize>
    where
        W: Write,
    {
        Prompt {
            content: &self.prompt,
            start_delay,
        }
        .to_writer(&mut writer)?;
        let input_time =
            (DELAY_TYPE_START + DELAY_TYPE_CHAR * command.input.len() + DELAY_TYPE_SUBMIT)
                / self.speed;
        for (i, c) in command.input.chars().map(|c| c.to_string()).enumerate() {
            let char_delay = (start_delay
                + DELAY_TYPE_START / self.speed
                + (DELAY_TYPE_CHAR * i) / self.speed) as f64
                / 1000.0;
            if self.stdin {
                Event(char_delay, EventKind::Keypress, &c).to_writer(&mut writer)?;
            }
            Event(char_delay, EventKind::default(), &c).to_writer(&mut writer)?;
        }
        for (i, output) in command.outputs.iter().enumerate() {
            let show_delay = (start_delay + input_time + (DELAY_OUTPUT_LINE * (i + 1)) / self.speed)
                as f64
                / 1000.0;
            if i == 0 {
                Event(show_delay, EventKind::default(), "\r\n").to_writer(&mut writer)?;
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                Event(show_delay, EventKind::default(), &output_data).to_writer(&mut writer)?;
            }
        }
        let outputs_time = if command.outputs.is_empty() {
            0
        } else {
            (DELAY_OUTPUT_LINE * command.outputs.len()) / self.speed
        };
        Ok(start_delay + input_time + outputs_time)
    }
}

fn main() -> Result<()> {
    AsciicastGen::from_args().execute()
}
