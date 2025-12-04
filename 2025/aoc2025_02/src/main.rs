type Range = Vec<u64>;

fn main() {
    let lines = aoc::init();
    let ranges: Vec<Range> = lines
        .first()
        .expect("Failed to parse input")
        .split(",")
        .map(|chunk| {
            let (start, end) = chunk.split_once("-").expect("Failed to parse a chunk");
            let start = start.parse::<u64>().expect("Start is not a number");
            let end = end.parse::<u64>().expect("End is not a number");

            (start..=end).collect()
        })
        .collect();

    let result = solve(ranges);
    println!("Invalid IDs sum is {:?}.", result);
}

// take the ranges and reduce them to return the sum of all the invalid ids
fn solve(ranges: Vec<Range>) -> (u64, u64) {
    let res: Option<(u64, u64)> = ranges
        .iter()
        .map(invalid_ids_in_range)
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1));

    if let Some((sum_twice, sum_at_least_twice)) = res {
        return (sum_twice, sum_at_least_twice);
    }

    (0, 0)
}

// Takes a range a return a tuple containing the sum of ids repeating exactly twice, and the sum
// of ids repeating at least twice.
fn invalid_ids_in_range(range: &Range) -> (u64, u64) {
    let mut sum_repeating_twice = 0;
    let mut sum_repeating_at_least_twice = 0;

    let ids_repeating_twice: Vec<u64> = range
        .iter()
        .copied()
        .filter(|id| repeat_exactly_twice(*id))
        .collect();

    let ids_repeating_at_least_twice: Vec<u64> = range
        .iter()
        .copied()
        .filter(|id| repeat_at_least_twice(*id))
        .collect();

    if !ids_repeating_twice.is_empty() {
        sum_repeating_twice = ids_repeating_twice
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .expect("Failed to reduce sum");
    }

    if !ids_repeating_at_least_twice.is_empty() {
        sum_repeating_at_least_twice = ids_repeating_at_least_twice
            .iter()
            .copied()
            .reduce(|acc, e| acc + e)
            .expect("Failed to reduce sum");
    }

    (sum_repeating_twice, sum_repeating_at_least_twice)
}

fn repeat_at_least_twice(id: u64) -> bool {
    let str = id.to_string();

    if str.len() <= 1 {
        return false;
    }

    // If the number is of pair length, we can check at most half its size by chunk.
    // If its not pair, at most half its size - 1.
    let max_chunk_size = match str.len() % 2 {
        0 => str.len() / 2,
        _ => (str.len() - 1) / 2,
    };

    let chunk_sizes_to_check: Vec<usize> = (1..=max_chunk_size).collect();

    for chunk_size in chunk_sizes_to_check {
        // Split the str into parts of chunk_size length and check if they are all the same
        let chunks: Vec<&str> = str
            .as_bytes()
            .chunks(chunk_size)
            .map(|chunk| str::from_utf8(chunk).expect("Failed to parse chunk to utf8"))
            .collect();

        let first_chunk = chunks.first();
        let all_equals = first_chunk.map(|first| chunks.iter().all(|c| c == first));

        if let Some(true) = all_equals {
            // This id repeats the first_chunk at least twice !
            return true;
        }
    }

    false
}

/// Takes an ind and returns wether it is invalid (true) or not (false).
fn repeat_exactly_twice(id: u64) -> bool {
    let str = id.to_string();

    // Since we're looking for a sequence of digits repeated *twice*,
    // we filter-out digits that does not have a pair length.
    if str.len() % 2 != 0 || str.is_empty() {
        return false;
    }

    let parts = str.split_at(str.len() / 2);

    // None of the numbers have leading zeros
    if parts.0.starts_with("0") || parts.1.starts_with("0") {
        return false;
    }

    parts.0.eq(parts.1)
}

#[cfg(test)]
mod tests {
    use crate::{invalid_ids_in_range, repeat_at_least_twice, repeat_exactly_twice, solve};

    #[test]
    fn test_id_repeat_at_least_twice() {
        assert!(repeat_at_least_twice(111));
        assert!(repeat_at_least_twice(999));
        assert!(repeat_at_least_twice(565656));
        assert!(repeat_at_least_twice(824824824));
        assert!(repeat_at_least_twice(2121212121));

        assert!(!repeat_at_least_twice(10123));
    }

    #[test]
    fn test_id_repeat_twice() {
        assert!(repeat_exactly_twice(11));
        assert!(repeat_exactly_twice(1010));
        assert!(repeat_exactly_twice(1188511885));
        assert!(repeat_exactly_twice(222222));
        assert!(repeat_exactly_twice(446446));

        assert!(!repeat_exactly_twice(446445));
        assert!(!repeat_exactly_twice(0));
        assert!(!repeat_exactly_twice(1));
        assert!(!repeat_exactly_twice(993));
    }

    #[test]
    fn test_invalid_ids_in_range() {
        assert_eq!(invalid_ids_in_range(&(11..=22).collect()), (33, 33));
        assert_eq!(invalid_ids_in_range(&(95..=115).collect()), (99, 99 + 111));
        assert_eq!(
            invalid_ids_in_range(&(998..=1012).collect()),
            (1010, 999 + 1010)
        );
        assert_eq!(
            invalid_ids_in_range(&(1188511880..=1188511890).collect()),
            (1188511885, 1188511885)
        );
        assert_eq!(
            invalid_ids_in_range(&(222220..=222224).collect()),
            (222222, 222222)
        );
        assert_eq!(invalid_ids_in_range(&(1698522..=1698528).collect()), (0, 0));
        assert_eq!(
            invalid_ids_in_range(&(446443..=446449).collect()),
            (446446, 446446)
        );
        assert_eq!(
            invalid_ids_in_range(&(38593856..=38593862).collect()),
            (38593859, 38593859)
        );

        assert_eq!(
            invalid_ids_in_range(&(565653..=565659).collect()),
            (0, 565656)
        );

        assert_eq!(
            invalid_ids_in_range(&(824824821..=824824827).collect()),
            (0, 824824824)
        );

        assert_eq!(
            invalid_ids_in_range(&(2121212118..=2121212124).collect()),
            (0, 2121212121)
        );
    }

    #[test]
    fn test_example() {
        let ranges = vec![
            (11..=22).collect(),
            (95..=115).collect(),
            (998..=1012).collect(),
            (1188511880..=1188511890).collect(),
            (222220..=222224).collect(),
            (1698522..=1698528).collect(),
            (446443..=446449).collect(),
            (38593856..=38593862).collect(),
            (565653..=565659).collect(),
            (824824821..=824824827).collect(),
            (2121212118..=2121212124).collect(),
        ];

        assert_eq!(solve(ranges), (1227775554, 4174379265));
    }
}
