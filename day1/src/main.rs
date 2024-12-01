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

    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();

    if let Ok(file) = File::open(&args.input_file) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let splitted: Vec<&str> = content.split_ascii_whitespace().collect();
                    list_1.push(splitted[0].parse().unwrap());
                    list_2.push(splitted[1].parse().unwrap());
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        eprintln!("Could not open file: {}", args.input_file);
    }

    list_1.sort();
    list_2.sort();

    let sum = list_1
        .iter()
        .zip(list_2.iter())
        .fold(0, |acc, (l1, l2)| acc + l1.abs_diff(*l2));

    println!("Sum: {}", sum);
}
