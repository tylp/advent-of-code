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
    let file = File::open(&args.input_file).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    reader.lines().for_each(|line| {
        lines.push(line.unwrap());
    });

    println!("Solution: {:?}", resolve(&lines));
}

fn resolve(lines: &[String]) -> u32 {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();

    lines.iter().for_each(|line| {
        let splitted: Vec<&str> = line.split_whitespace().collect();
        list_1.push(splitted[0].parse::<u32>().unwrap());
        list_2.push(splitted[1].parse::<u32>().unwrap());
    });

    list_1.sort();
    list_2.sort();

    let sum = list_1
        .iter()
        .zip(list_2.iter())
        .fold(0, |acc, (l1, l2)| acc + l1.abs_diff(*l2));

    println!("Sum distance: {:?}", sum);

    let mut similarities = 0;
    list_1.iter().for_each(|l| {
        // Find the number of occurences in l2
        let occurences = list_2.iter().filter(|&x| x == l).count() as u32;
        similarities += occurences * l;
    });

    println!("Sum similarities: {:?}", similarities);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let lines = vec![
            "20   17".to_string(),
            "10   11".to_string(),
            "0   5".to_string(),
        ];
        assert_eq!(resolve(&lines), 9);
    }
}
