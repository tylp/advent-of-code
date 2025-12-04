type Joltage = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BatteryState {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Battery {
    joltage: Joltage,
    state: BatteryState,
}

impl Battery {
    pub fn new(joltage: Joltage) -> Self {
        Self {
            joltage,
            state: BatteryState::Off,
        }
    }

    pub fn switch_on(&mut self) {
        self.state = BatteryState::On
    }

    pub fn switch_off(&mut self) {
        self.state = BatteryState::Off
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

    pub fn largest_possible_joltage(&self) -> u8 {
        // We do not want to move battery ordering so we clone it
        let mut batteries = self.batteries.clone();

        batteries.sort_by_key(|battery| battery.joltage);

        let first = batteries.first();
        let second = batteries.get(1);

        match (first, second) {
            (None, None) => 0,
            (None, Some(second)) => second.joltage,
            (Some(first), None) => first.joltage,
            (Some(first), Some(second)) => format!("{}{}", first.joltage, second.joltage)
                .parse::<u8>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to parse first and second: {}, {}",
                        first.joltage, second.joltage
                    )
                }),
        }
    }
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries: Vec<Battery> = value
            .as_bytes()
            .iter()
            .map(|joltage| Battery::new(*joltage))
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

    println!("Battery banks: {:?}", banks);
}

fn solve() -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{Battery, BatteryBank};

    #[test]
    fn test_example() {
        let input = "987654321111111";
        let bank = BatteryBank::from(input);

        assert_eq!(bank.largest_possible_joltage(), 98);
    }
}
