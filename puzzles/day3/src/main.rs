use clap::Parser;
use regex::Regex;
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
    let file = File::open(&args.input_file).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    reader.lines().for_each(|line| {
        lines.push(line.unwrap());
    });

    println!("Solution: {:?}", resolve(&lines));
}

fn resolve(lines: &[String]) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: u32 = 0;
    lines.iter().for_each(|line| {
        let lsum = regex.captures_iter(line).fold(0, |acc, cap| {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();

            acc + x * y
        });

        sum += lsum;
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let lines = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        assert_eq!(resolve(&lines), 161);
    }
}
