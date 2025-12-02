#[derive(Debug, Clone)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let (direction, distance) = value.split_at(1);

        match (direction, distance) {
            ("L", distance) => Self::Left(
                distance
                    .parse::<i32>()
                    .expect("Left input distance is invalid!"),
            ),
            ("R", distance) => Self::Right(
                distance
                    .parse::<i32>()
                    .expect("Right input distance is invalid!"),
            ),
            (_, _) => panic!("Invalid input!"),
        }
    }
}

struct Safe {
    dial: i32,
    dial_stopped_at_zero: i32,
    times_passed_through_zero: i32,
    pins: Vec<i32>,
}

impl Safe {
    pub fn new(dial_start: i32, pins_size: i32) -> Self {
        let pins: Vec<i32> = (0..=pins_size).collect();

        Self {
            dial: dial_start,
            dial_stopped_at_zero: 0,
            times_passed_through_zero: 0,
            pins,
        }
    }

    pub fn unlock(&mut self, rotations: Vec<Rotation>) {
        rotations.iter().for_each(|rotation| match rotation {
            Rotation::Left(distance) => self.left(*distance),
            Rotation::Right(distance) => self.right(*distance),
        });
    }

    pub fn left(&mut self, distance: i32) {
        let mut iter = self.pins.iter().rev().cycle();
        let len = self.pins.len();
        let dial = self.dial;
        // Move the iterator to the dial position
        for _ in 0..len {
            if let Some(pin) = iter.next()
                && *pin == dial
            {
                break;
            }
        }

        // Then move for the given distance
        (0..distance).for_each(|step| {
            if let Some(pin) = iter.next() {
                // we are at zero AND there are steps remaining
                if *pin == 0 && step != distance {
                    self.times_passed_through_zero += 1;
                }

                self.dial = *pin;
            }
        });

        // Check if the dial is at zero
        if self.dial == 0 {
            self.dial_stopped_at_zero += 1;
        }
    }

    pub fn right(&mut self, distance: i32) {
        let mut iter = self.pins.iter().cycle();
        let len = self.pins.len();
        let dial = self.dial;
        // Move the iterator to the dial position
        for _ in 0..len {
            if let Some(pin) = iter.next()
                && *pin == dial
            {
                break;
            }
        }

        // Then move for the given distance
        (0..distance).for_each(|step| {
            if let Some(pin) = iter.next() {
                // we are at zero AND there are steps remaining
                if *pin == 0 && step != distance {
                    self.times_passed_through_zero += 1;
                }

                self.dial = *pin;
            }
        });

        // Check if the dial is at zero
        if self.dial == 0 {
            self.dial_stopped_at_zero += 1;
        }
    }

    pub fn dial(&self) -> i32 {
        self.dial
    }

    pub fn dial_stopped_at_zero(&self) -> i32 {
        self.dial_stopped_at_zero
    }

    pub fn times_passed_through_zero(&self) -> i32 {
        self.times_passed_through_zero
    }
}

fn main() {
    let lines = aoc::init();
    let rotations: Vec<Rotation> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(Rotation::from)
        .collect();
    let mut safe = Safe::new(50, 99);

    safe.unlock(rotations);

    let final_dial = safe.dial();
    let zero_passes = safe.dial_stopped_at_zero();
    let total_pass_zero = safe.times_passed_through_zero();

    println!(
        "Final dial: {final_dial}. Finished at zero: {zero_passes} times. Passed by zero {total_pass_zero} times"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut safe = Safe::new(50, 99);

        let rotations = vec![
            Rotation::Left(68),
            Rotation::Left(30),
            Rotation::Right(48),
            Rotation::Left(5),
            Rotation::Right(60),
            Rotation::Left(55),
            Rotation::Left(1),
            Rotation::Left(99),
            Rotation::Right(14),
            Rotation::Left(82),
        ];

        safe.unlock(rotations);

        assert_eq!(safe.dial_stopped_at_zero(), 3);
        assert_eq!(safe.times_passed_through_zero(), 6);
    }

    #[test]
    fn test_left_right() {
        let mut safe = Safe::new(50, 99);

        safe.right(1);
        assert_eq!(safe.dial(), 51);

        safe.right(1);
        assert_eq!(safe.dial(), 52);

        safe.left(10);
        assert_eq!(safe.dial(), 42);

        safe.left(43);
        assert_eq!(safe.dial(), 99);

        safe.right(99);
        assert_eq!(safe.dial(), 98);

        safe.right(100);
        assert_eq!(safe.dial(), 98);

        safe.left(1000);
        assert_eq!(safe.dial(), 98);
    }
}
