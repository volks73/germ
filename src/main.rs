use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate asciinema cast files without recording")]
struct Args {
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

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);
}
