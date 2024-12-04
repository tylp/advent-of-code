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

fn resolve(lines: &[String]) -> i32 {
    lines.iter().map(|line| extract(line)).sum()
}

fn extract(line: &str) -> i32 {
    let mut enabled = true;
    let mut result = 0;

    let do_regex = regex::Regex::new(r"^do\(\)").unwrap();
    let dont_regex = regex::Regex::new(r"^don't\(\)").unwrap();
    let mul_regex = regex::Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut idx = 0;
    while idx < line.len() {
        let slice = &line[idx..];

        if let Some(mat) = do_regex.find(slice) {
            enabled = true;
            idx += mat.end();
        } else if let Some(mat) = dont_regex.find(slice) {
            enabled = false;
            idx += mat.end();
        } else if let Some(caps) = mul_regex.captures(slice) {
            if enabled {
                let a: i32 = caps[1].parse().unwrap();
                let b: i32 = caps[2].parse().unwrap();
                result += a * b;
            }
            idx += caps.get(0).unwrap().end();
        } else {
            idx += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let lines = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        assert_eq!(resolve(&lines), 48);
    }
}
