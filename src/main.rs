use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use env_logger::fmt::Color as LogColor;
use env_logger::Builder;
use log::Level;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

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
    println!("{:?}", args);
    Ok(())
}
