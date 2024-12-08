use clap::Parser;
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

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    println!("Solution: {:?}", resolve(&buffer));
}

fn resolve(buffer: &str) -> i32 {
    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.find_iter(buffer).fold(0, |acc, m| {
        let slice = &buffer[0..m.start()];
        let dont = slice.rfind("don't()");
        let do_ = slice.rfind("do()");

        let regex = regex::Regex::new(r"(\d+),(\d+)").unwrap();
        let captures = regex.captures(m.as_str()).unwrap();

        let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

        if (do_.is_none() && dont.is_none())
            || (do_.is_some() && dont.is_none())
            || (do_.is_some() && dont.is_some() && do_.unwrap() > dont.unwrap())
        {
            return acc + a * b;
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let buffer = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(resolve(buffer), 48);
    }
}
