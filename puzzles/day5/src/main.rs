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

    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    println!("Solution: {:?}", resolve(&input));
}

fn resolve(lines: &str) -> i32 {
    let (mut ordering, updates) = parse_input(lines);

    // Sort the rules by the first page number
    ordering.sort_by(|a, b| a.0.cmp(&b.0));

    // Loop through each update and compare indexes n and n+1.
    // Verify that n and n+1 respect the ordering.
    let valid_updates = filter_correct_updates(&ordering, &updates);

    sum_middle_pages(&valid_updates)
}
type PageOrdering = Vec<(i32, i32)>;
type Update = Vec<i32>;
type Updates = Vec<Update>;

fn sum_middle_pages(updates: &Updates) -> i32 {
    updates.iter().fold(0, |acc, u| {
        let len = u.len();
        let value = u[len / 2];

        acc + value
    })
}

fn filter_correct_updates(rules: &PageOrdering, updates: &Updates) -> Updates {
    updates
        .iter()
        .filter(|update| is_valid_update(rules, update))
        .cloned()
        .collect()
}

fn is_valid_update(rules: &PageOrdering, update: &Update) -> bool {
    let mut flag = true;

    for (pos, page) in update.iter().enumerate() {
        // Find the page in the rules.
        let rules: Vec<&(i32, i32)> = rules.iter().filter(|(a, _)| a == page).collect();

        rules.iter().for_each(|(_, b)| {
            let index = update.iter().position(|x| x == b);

            if let Some(index) = index {
                if index < pos {
                    flag = false;
                }
            }
        });
    }

    flag
}

fn parse_input(intput: &str) -> (PageOrdering, Updates) {
    // Split the black line
    let mut parts = intput.split("\n\n");

    let ordering_section = parts.next().unwrap();
    let updates_section = parts.next().unwrap();

    let ordering = parse_page_ordering(ordering_section);
    let updates = parse_updates(updates_section);

    (ordering, updates)
}

fn parse_page_ordering(input: &str) -> PageOrdering {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            let page = parts.next().unwrap().trim().parse().unwrap();
            let order = parts.next().unwrap().trim().parse().unwrap();
            (page, order)
        })
        .collect()
}

fn parse_updates(input: &str) -> Updates {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|part| part.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_middle_pages() {
        let updates = vec![
            vec![10, 10, 1, 0, 0],
            vec![19, 2, 3, 1, 2],
            vec![0, 3, 4, 7, 2],
            vec![1, 4, 3],
        ];

        let expected = 1 + 3 + 4 + 4;
        assert_eq!(sum_middle_pages(&updates), expected);
    }

    #[test]
    fn test_parse_input() {
        let input = "0|0\n1|1\n\n0,1,23";
        let (ordering, updates) = parse_input(input);

        assert_eq!(ordering, vec![(0, 0), (1, 1)]);
        assert_eq!(updates, vec![vec![0, 1, 23]]);
    }

    #[test]
    fn test_is_valid_update_true_1() {
        let rules = vec![(1, 2), (1, 5), (2, 5)];
        let update = vec![1, 2, 5];

        assert!(is_valid_update(&rules, &update));
    }

    #[test]
    fn test_is_valid_update_false_1() {
        let rules = vec![(1, 2), (5, 1), (2, 5)];
        let update = vec![1, 2, 5];

        assert!(!is_valid_update(&rules, &update));
    }

    #[test]
    fn test_filter_valid_updates() {
        let rules = vec![(1, 2), (1, 5), (2, 5)];
        let updates = vec![
            vec![1, 2, 5], // valid
            vec![1, 5, 2], // invalid
            vec![2, 5, 1], // invalid
        ];

        let valid_updates = filter_correct_updates(&rules, &updates);

        assert_eq!(valid_updates, vec![vec![1, 2, 5]]);
    }
}
