use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub const VERSION: usize = 1;
pub const DEFAULT_PROMPT: &str = "$ ";
pub const DEFAULT_SPEED: &str = "1.0";
pub const DEFAULT_BEGIN_DELAY: &str = "0.0";
pub const DEFAULT_END_DELAY: &str = "1.0";
pub const DEFAULT_DELAY_TYPE_START: &str = "750";
pub const DEFAULT_DELAY_TYPE_CHAR: &str = "35";
pub const DEFAULT_DELAY_TYPE_SUBMIT: &str = "350";
pub const DEFAULT_DELAY_OUTPUT_LINE: &str = "500";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, StructOpt)]
pub struct Timings {
    /// The delay before starting the animation.
    ///
    /// The units are in seconds (s).
    #[structopt(
        short = "B",
        long = "begin-delay",
        default_value = DEFAULT_BEGIN_DELAY,
        value_name = crate::SECONDS_UNITS,
        env = "GERM_BEGIN_DELAY"
    )]
    pub begin: f64, // seconds

    /// The delay at the end of the animation.
    ///
    /// This is useful when looping/repeat is enabled and some time between
    /// iterations is needed and/or desired. Set the value to 0.0 if no hold is
    /// desired. The units are in seconds (s).
    #[structopt(
        short = "E",
        long = "end-delay",
        default_value = DEFAULT_END_DELAY,
        value_name = crate::SECONDS_UNITS,
        env = "GERM_END_DELAY"
    )]
    pub end: f64, // seconds

    /// The delay before starting the simulated typing for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long = "delay-type-start",
        default_value = DEFAULT_DELAY_TYPE_START,
        value_name = crate::MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_START"
    )]
    pub type_start: usize, // milliseconds

    /// The delay between simulating typing of characters for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long = "delay-type-char",
        default_value = DEFAULT_DELAY_TYPE_CHAR,
        value_name = crate::MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_CHAR"
    )]
    pub type_char: usize, // milliseconds

    /// The delay between the simulated typing and output printing.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long = "delay-type-submit",
        default_value = DEFAULT_DELAY_TYPE_SUBMIT,
        value_name = crate::MILLISECONDS_UNITS,
        env = "GERM_DELAY_TYPE_SUBMIT"
    )]
    pub type_submit: usize, // milliseconds

    /// The delay between outputs for the command.
    ///
    /// The units are in milliseconds (ms).
    #[structopt(
        long = "delay-output-line",
        default_value = DEFAULT_DELAY_OUTPUT_LINE,
        value_name = crate::MILLISECONDS_UNITS,
        env = "GERM_DELAY_OUTPUT_LINE",
        hide_env_values = true
    )]
    pub output_line: usize, // milliseconds

    /// Speed up or slow down the animation by this factor.
    #[structopt(short = "s", long, default_value = "1.0", value_name = "float")]
    pub speed: f64, // Factor
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
            speed: DEFAULT_SPEED.parse().expect("Default speed"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sequence {
    version: usize,
    timings: Timings,
    commands: Vec<Command>,
}

impl Sequence {
    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.commands.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Command> {
        self.commands.into_iter()
    }

    pub fn add(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }

    pub fn append(&mut self, commands: &mut Vec<Command>) -> &mut Self {
        self.commands.append(commands);
        self
    }

    pub fn append_from(&mut self, s: Sequence) -> &mut Self {
        let Sequence { commands, .. } = s;
        for command in commands {
            self.add(command);
        }
        self
    }

    pub fn timings(&self) -> &Timings {
        &self.timings
    }

    pub fn into_timings(self) -> Timings {
        self.timings
    }

    pub fn version(&self) -> usize {
        self.version
    }
}

impl From<Vec<Command>> for Sequence {
    fn from(c: Vec<Command>) -> Self {
        Self {
            commands: c,
            ..Default::default()
        }
    }
}

impl From<Timings> for Sequence {
    fn from(t: Timings) -> Self {
        Self {
            timings: t,
            ..Default::default()
        }
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            version: VERSION,
            timings: Timings::default(),
            commands: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub prompt: String,
    pub input: String,
    pub outputs: Vec<String>,
}
