use clap::Parser;
use std::fs::File;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required = true)]
    pub input_file: String,
}

/// Reads lines from a file and returns them as a vector of strings.
pub fn init() -> Vec<String> {
    let args = Args::parse();

    let file = File::open(args.input_file).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    reader.lines().for_each(|line| {
        lines.push(line.unwrap());
    });

    lines
}
