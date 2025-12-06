type Joltage = u32;

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

    /// Find the largest possible voltage for the bank by finding the two
    /// highest values with preserved order.
    pub fn largest_possible_two_digit_joltage(&self) -> u32 {
        let mut best = 0u32;
        let len = self.batteries.len();

        let digits: Vec<u32> = self.batteries.iter().map(|b| b.joltage).collect();

        for i in 0..len {
            for j in (i + 1)..len {
                let val = digits[i] * 10 + digits[j];
                if val > best {
                    best = val;
                }
            }
        }

        best
    }

    /// Return the largest possible joltage for the bank by turning on
    /// a given number of digits.
    pub fn larget_possible_joltage_for_digits(&self, digits: u32) -> u32 {
        todo!()
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries: Vec<Battery> = value
            .chars()
            .map(|c| c.to_digit(10))
            .filter(|item| item.is_some())
            .map(|joltage| Battery::new(joltage.unwrap()))
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

    let total_output = banks
        .iter()
        .fold(0, |acc, e| acc + e.largest_possible_two_digit_joltage());

    println!("Total output of the banks: {total_output}");
}

#[cfg(test)]
mod tests {
    use crate::{Battery, BatteryBank};

    #[test]
    fn test_example() {
        let bank = BatteryBank::from("987654321111111");
        assert_eq!(bank.largest_possible_two_digit_joltage(), 98);

        let bank = BatteryBank::from("811111111111119");
        assert_eq!(bank.largest_possible_two_digit_joltage(), 89);

        let bank = BatteryBank::from("234234234234278");
        assert_eq!(bank.largest_possible_two_digit_joltage(), 78);

        let bank = BatteryBank::from("818181911112111");
        assert_eq!(bank.largest_possible_two_digit_joltage(), 92);
    }
}
