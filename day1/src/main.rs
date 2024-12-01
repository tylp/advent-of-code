use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.input_file);

    if let Ok(file) = File::open(&args.input_file) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        eprintln!("Could not open file: {}", args.input_file);
    }
}
