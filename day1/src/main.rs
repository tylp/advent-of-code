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
                    let splitted: Vec<&str> = content.split("   ").collect();
                    list_1.push(splitted[0].parse().unwrap());
                    list_2.push(splitted[1].parse().unwrap());
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        eprintln!("Could not open file: {}", args.input_file);
    }

    let min_l1 = list_1.iter().min().unwrap();
    let min_l2 = list_2.iter().min().unwrap();

    let diff = min_l1.abs_diff(*min_l2);

    println!("Min of list 1: {}", min_l1);
    println!("Max of list 2: {}", min_l2);
    println!("Min difference: {}", diff);
}
