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

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate asciicast files without using asciinema's recording functionality")]
struct AsciicastGen {
    /// The prompt to display before the command.
    #[structopt(short = "p", long = "prompt", default_value = "~$ ")]
    prompt: String,

    /// Mimic keypress logging functionality of the asciinema record functionality.
    #[structopt(short, long)]
    stdin: bool,

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

fn write_prompt<W>(mut writer: W, prompt: &str, start_delay: usize) -> Result<()>
where
    W: Write,
{
    serde_json::to_writer(
        &mut writer,
        &Event(start_delay as f64 / 1000.0, EventKind::default(), prompt),
    )?;
    writeln!(&mut writer)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = AsciicastGen::from_args();
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
    let mut writer: Box<dyn Write> = if let Some(output_file) = args.output_file {
        Box::new(File::create(output_file)?)
    } else {
        Box::new(io::stdout())
    };
    serde_json::to_writer(&mut writer, &Header::default())?;
    writeln!(&mut writer)?;
    for command in commands.iter() {
        write_prompt(&mut writer, &args.prompt, start_delay)?;
        let input_time =
            (DELAY_TYPE_START + DELAY_TYPE_CHAR * command.input.len() + DELAY_TYPE_SUBMIT) / speed;
        for (i, c) in command.input.chars().map(|c| c.to_string()).enumerate() {
            let char_delay =
                (start_delay + DELAY_TYPE_START / speed + (DELAY_TYPE_CHAR * i) / speed) as f64
                    / 1000.0;
            if args.stdin {
                serde_json::to_writer(&mut writer, &Event(char_delay, EventKind::Keypress, &c))?;
                writeln!(&mut writer)?;
            }
            serde_json::to_writer(&mut writer, &Event(char_delay, EventKind::default(), &c))?;
            writeln!(&mut writer)?;
        }
        for (i, output) in command.outputs.iter().enumerate() {
            let show_delay =
                (start_delay + input_time + (DELAY_OUTPUT_LINE * (i + 1)) / speed) as f64 / 1000.0;
            if i == 0 {
                serde_json::to_writer(
                    &mut writer,
                    &Event(show_delay, EventKind::default(), "\r\n"),
                )?;
                writeln!(&mut writer)?;
            }
            for line in output.lines() {
                let mut output_data = String::from(line);
                output_data.push_str("\r\n");
                serde_json::to_writer(
                    &mut writer,
                    &Event(show_delay, EventKind::default(), &output_data),
                )?;
                writeln!(&mut writer)?;
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
