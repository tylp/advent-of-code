use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.input_file).unwrap();
    let mut reader = io::BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let (part1, part2) = resolve(&input);
    println!(
        "#### Solutions ####\n Part 1: {:?}\n Part 2: {:?}",
        part1, part2
    );
}

fn resolve(lines: &str) -> (i32, i32) {
    let stones = parse_input(lines);
    let blink_25 = blink_nth(stones, 25);

    (blink_25.len() as i32, 0)
}

// Cache values for each step. Eg [value, [step, computed value]]
type Cache = std::collections::HashMap<u64, HashMap<u64, u64>>;

/// Blink a given number of time on the given stones.
fn blink_nth(stones: Vec<u64>, times: u32) -> Vec<u64> {
    // TODO: Part 2. Implement a cache to store the result of every value at a given step.
    // For example:
    // {value: 125: {2, 253000}} <-- the value 125 gives 253000 when blinekd 2 times.
    let _cache: Cache = HashMap::new();

    let mut s = stones;
    for _ in 0..times {
        s = blink(s)
    }

    s
}

// Blink and transform the stones by applying the given rules.
fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut s: Vec<u64> = Vec::new();

    stones.iter().for_each(|stone| {
        let is_zero = *stone == 0;
        let is_even = stone.to_string().len() % 2 == 0;

        // rule 1
        if is_zero {
            s.push(1);
        }

        // rule 2
        if is_even {
            let str = stone.to_string();
            let len = str.len();
            let p1: u64 = str[0..len / 2].parse().unwrap();
            let p2: u64 = str[len / 2..len].parse().unwrap();

            s.push(p1);
            s.push(p2);
        }

        // rule 3
        if !is_even && !is_zero {
            s.push(*stone * 2024);
        }
    });

    s
}

/// Parse the input to return a list of stones
fn parse_input(lines: &str) -> Vec<u64> {
    lines
        .split(" ")
        .map(|block| block.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let input = vec![0, 1, 10, 99, 999];
        let output = vec![1, 2024, 1, 0, 9, 9, 2021976];

        assert_eq!(blink(input), output);
    }

    #[test]
    fn test_blink_nth() {
        let input = vec![125, 17];
        let output = vec![253, 0, 2024, 14168];

        assert_eq!(blink_nth(input, 2), output);
    }
}
