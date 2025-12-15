type Joltage = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Battery {
    joltage: Joltage,
}

impl Battery {
    pub fn new(joltage: Joltage) -> Self {
        Self { joltage }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BatteryBank {
    batteries: Vec<Battery>,
}

impl BatteryBank {
    pub fn new(batteries: Vec<Battery>) -> Self {
        Self { batteries }
    }

    /// Return the largest possible joltage for the bank by turning on
    /// a given number of digits.
    ///
    /// # Example
    ///
    /// - **98**7654321111111 returns 98 on two digits.
    /// - **987654321111**111 returns 987654321111 on twelve digits.
    ///
    /// - **8**1111111111111**9** returns 89 on two digits.
    /// - **81111111111**111**9** returns 89 on two digits.
    ///
    pub fn largest_possible_joltage_for_digits(&self, digits: usize) -> u64 {
        let joltages: Vec<u64> = self.batteries.iter().map(|b| b.joltage).collect();

        if digits == 0 || joltages.is_empty() {
            return 0;
        }

        let mut to_remove = joltages.len() - digits;
        let mut stack = Vec::new();

        for joltage in joltages {
            while to_remove > 0 {
                match stack.last() {
                    Some(&last) if last < joltage => {
                        stack.pop();
                        to_remove -= 1;
                    }
                    _ => break,
                }
            }
            stack.push(joltage);
        }

        if to_remove > 0 {
            let new_len = stack.len() - to_remove;
            stack.truncate(new_len);
        }

        stack.truncate(digits);

        stack.into_iter().fold(0, |acc, e| acc * 10 + e)
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries: Vec<Battery> = value
            .chars()
            .map(|c| u64::from(c.to_digit(10).expect("Failed to get char digits")))
            .map(Battery::new)
            .collect();
        BatteryBank::new(batteries)
    }
}

fn main() {
    let lines = aoc::init();
    let banks: Vec<BatteryBank> = lines
        .iter()
        .map(|line| BatteryBank::from(line.as_str()))
        .collect();

    let two_digits_output = banks
        .iter()
        .fold(0, |acc, e| acc + e.largest_possible_joltage_for_digits(2));

    let twelve_digits_output = banks
        .iter()
        .fold(0, |acc, e| acc + e.largest_possible_joltage_for_digits(12));

    println!("Two digits output: {two_digits_output}");
    println!("Twelve digits output: {twelve_digits_output}");
}

#[cfg(test)]
mod tests {
    use crate::BatteryBank;

    #[test]
    fn test_example() {
        let bank = BatteryBank::from("987654321111111");
        assert_eq!(bank.largest_possible_joltage_for_digits(2), 98);

        let bank = BatteryBank::from("811111111111119");
        assert_eq!(bank.largest_possible_joltage_for_digits(2), 89);

        let bank = BatteryBank::from("234234234234278");
        assert_eq!(bank.largest_possible_joltage_for_digits(2), 78);

        let bank = BatteryBank::from("818181911112111");
        assert_eq!(bank.largest_possible_joltage_for_digits(2), 92);

        let bank = BatteryBank::from("987654321111111");
        assert_eq!(bank.largest_possible_joltage_for_digits(12), 987654321111);

        let bank = BatteryBank::from("811111111111119");
        assert_eq!(bank.largest_possible_joltage_for_digits(12), 811111111119);

        let bank = BatteryBank::from("234234234234278");
        assert_eq!(bank.largest_possible_joltage_for_digits(12), 434234234278);

        let bank = BatteryBank::from("818181911112111");
        assert_eq!(bank.largest_possible_joltage_for_digits(12), 888911112111);
    }
}
