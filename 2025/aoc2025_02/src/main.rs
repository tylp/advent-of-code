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
    println!("Invalid IDs sum is {result}.");
}

// take the ranges and reduce them to return the sum of all the invalid ids
fn solve(ranges: Vec<Range>) -> u64 {
    let res: Option<(u64, Vec<u64>)> = ranges
        .iter()
        .map(invalid_ids_in_range)
        .reduce(|acc, e| (acc.0 + e.0, [acc.1, e.1].concat()));

    if let Some((res, _ids)) = res {
        return res;
    }

    0
}

// Takes a range a return a tuple containing the sum of invalid ids in this range as well as the ids.
fn invalid_ids_in_range(range: &Range) -> (u64, Vec<u64>) {
    let invalid_ids: Vec<u64> = range
        .iter()
        .copied()
        .filter(|id| id_repeat_twice(*id))
        .collect();

    if invalid_ids.is_empty() {
        return (0, vec![]);
    }

    let sum = invalid_ids
        .iter()
        .copied()
        .reduce(|acc, e| acc + e)
        .expect("Failed to reduce sum");

    (sum, invalid_ids)
}

/// Takes an ind and returns wether it is invalid (true) or not (false).
fn id_repeat_twice(id: u64) -> bool {
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
    use crate::{id_repeat_twice, invalid_ids_in_range, solve};

    #[test]
    fn test_id_repeat_at_least_twice() {}

    #[test]
    fn test_id_repeat_twice() {
        assert!(id_repeat_twice(11));
        assert!(id_repeat_twice(1010));
        assert!(id_repeat_twice(1188511885));
        assert!(id_repeat_twice(222222));
        assert!(id_repeat_twice(446446));

        assert!(!id_repeat_twice(446445));
        assert!(!id_repeat_twice(0));
        assert!(!id_repeat_twice(1));
        assert!(!id_repeat_twice(993));
    }

    #[test]
    fn test_invalid_ids_in_range() {
        assert_eq!(
            invalid_ids_in_range(&(11..=22).collect()),
            (33, vec![11, 22])
        );
        assert_eq!(invalid_ids_in_range(&(95..=115).collect()), (99, vec![99]));
        assert_eq!(
            invalid_ids_in_range(&(998..=1012).collect()),
            (1010, vec![1010])
        );
        assert_eq!(
            invalid_ids_in_range(&(1188511880..=1188511890).collect()),
            (1188511885, vec![1188511885])
        );
        assert_eq!(
            invalid_ids_in_range(&(222220..=222224).collect()),
            (222222, vec![222222])
        );
        assert_eq!(
            invalid_ids_in_range(&(1698522..=1698528).collect()),
            (0, vec![])
        );
        assert_eq!(
            invalid_ids_in_range(&(446443..=446449).collect()),
            (446446, vec![446446])
        );
        assert_eq!(
            invalid_ids_in_range(&(38593856..=38593862).collect()),
            (38593859, vec![38593859])
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
        ];

        assert_eq!(solve(ranges), 1227775554);
    }
}
