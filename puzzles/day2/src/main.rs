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

fn resolve(lines: &[String]) -> u16 {
    let report_list = ReportList::from(lines);

    report_list.0.iter().fold(
        0,
        |acc, report| {
            if is_report_safe(report) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[derive(Debug)]
struct Report(Vec<u16>);
struct ReportList(Vec<Report>);

impl From<&[String]> for ReportList {
    fn from(lines: &[String]) -> Self {
        ReportList(lines.iter().map(Report::from).collect())
    }
}

impl From<&String> for Report {
    fn from(value: &String) -> Self {
        let values = value
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        Report(values)
    }
}

fn is_report_safe(report: &Report) -> bool {
    let decr_or_incr = decreasing_or_increasing(&report.0);
    let adjacent = adjacent_levels(&report.0);

    decr_or_incr && adjacent
}

fn decreasing_or_increasing(levels: &[u16]) -> bool {
    let increasing = levels.iter().is_sorted_by(|a, b| a <= b);
    let decreasing = levels.iter().is_sorted_by(|a, b| a >= b);

    increasing || decreasing
}

fn adjacent_levels(levels: &[u16]) -> bool {
    let l1 = levels.iter();
    let l2 = levels.iter().skip(1);

    l1.zip(l2)
        .fold(true, |acc, (l1, l2)| acc && adjacent(*l1, *l2))
}

fn adjacent(l1: u16, l2: u16) -> bool {
    let diff = l1.abs_diff(l2);
    (1..=3).contains(&diff)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_adjacent_levels() {
        let v = vec![1, 2, 3, 4];
        assert!(adjacent_levels(&v));

        let v = vec![1, 1, 2, 3];
        assert!(!adjacent_levels(&v));

        let v = vec![1, 5, 6];
        assert!(!adjacent_levels(&v));

        let v = vec![63, 67, 70, 77, 78, 81];
        assert!(!adjacent_levels(&v));
    }

    #[test]
    fn test_decreasing_or_increasing() {
        let v1 = vec![1, 2, 3];
        assert!(decreasing_or_increasing(&v1));

        let v2 = vec![3, 2, 1];
        assert!(decreasing_or_increasing(&v2));

        let v3 = vec![1, 3, 2];
        assert!(!decreasing_or_increasing(&v3));
    }

    #[test]
    fn test_adjacent() {
        assert!(!adjacent(1, 1));
        assert!(adjacent(1, 2));
        assert!(!adjacent(1, 7));
    }

    #[test]
    fn test_is_report_safe_true() {
        let input = "7 6 4 2 1";
        let report = Report::from(&input.to_string());
        assert!(is_report_safe(&report));
    }

    #[test]
    fn test_is_report_safe_false() {
        let input = "1 2 7 8 9";
        let report = Report::from(&input.to_string());
        assert!(!is_report_safe(&report));
    }

    #[test]
    fn test_resolve_example() {
        let lines = vec![
            "7 6 4 2 1".to_string(), // Safe because the levels are all decreasing by 1 or 2.
            "1 2 7 8 9".to_string(), // Unsafe because 2 7 is an increase of 5.
            "9 7 6 2 1".to_string(), // Unsafe because 6 2 is a decrease of 4.
            "1 3 2 4 5".to_string(), // Unsafe because 1 3 is increasing but 3 2 is decreasing.
            "8 6 4 4 1".to_string(), // Unsafe because 4 4 is neither an increase or a decrease.
            "1 3 6 7 9".to_string(), // Safe because the levels are all increasing by 1, 2, or 3.
        ];

        assert_eq!(resolve(&lines), 2);
    }

    #[test]
    fn test_resolve_different_length() {
        let lines = vec![
            "7 6 4 2 1".to_string(), // Safe because the levels are all decreasing by 1 or 2.
            "1 2 7 8 9".to_string(), // Unsafe because 2 7 is an increase of 5.
            "9 7 6 2 1".to_string(), // Unsafe because 6 2 is a decrease of 4.
            "1 3 2 4 5".to_string(), // Unsafe because 1 3 is increasing but 3 2 is decreasing.
            "8 6 4 4 1".to_string(), // Unsafe because 4 4 is neither an increase or a decrease.
            "1 3 6 7 9 11 12".to_string(), // Safe because the levels are all increasing by 1, 2, or 3.
        ];

        assert_eq!(resolve(&lines), 2);
    }
}
