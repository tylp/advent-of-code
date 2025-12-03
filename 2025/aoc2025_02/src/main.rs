fn main() {
    let lines = aoc::init();
    let ranges: Vec<Vec<i32>> = lines
        .first()
        .expect("Failed to parse input")
        .split(",")
        .map(|chunk| {
            let (start, end) = chunk.split_once("-").expect("Failed to parse a chunk");
            let start = start.parse::<i32>().expect("Start is not a number");
            let end = end.parse::<i32>().expect("End is not a number");

            (start..=end).collect()
        })
        .collect();

    let result = solve(ranges);
    println!("Invalid IDs sum is {result}.");
}

fn solve(ranges: Vec<Vec<i32>>) -> i32 {
    todo!()
}

fn invalid_ids_in_range(range: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{invalid_ids_in_range, solve};

    #[test]
    fn test_invalid_ids_in_range() {
        assert_eq!(invalid_ids_in_range((11..22).collect()), 2);
    }

    #[test]
    fn test_example() {
        let ranges = vec![
            (11..22).collect(),
            (95..115).collect(),
            (998..1012).collect(),
            (1188511880..1188511890).collect(),
            (222220..222224).collect(),
            (1698522..1698528).collect(),
            (446443..446449).collect(),
            (38593856..38593862).collect(),
        ];

        assert_eq!(solve(ranges), 1227775554);
    }
}
