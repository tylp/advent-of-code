use std::iter::Cycle;

#[derive(Debug, Clone)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let (direction, distance) = value.split_at(1);

        match (direction, distance) {
            ("L", distance) => Self::Left(
                distance
                    .parse::<u16>()
                    .expect("Left input distance is invalid!"),
            ),
            ("R", distance) => Self::Right(
                distance
                    .parse::<u16>()
                    .expect("Right input distance is invalid!"),
            ),
            (_, _) => panic!("Invalid input!"),
        }
    }
}

struct Safe {
    dial: u16,
    pins: Vec<u16>,
    zero_passes: u16,
}

impl Safe {
    pub fn new(dial_start: u16, pins_size: u16) -> Self {
        Self {
            dial: dial_start,
            pins: (0..=pins_size).collect(),
            zero_passes: 0,
        }
    }

    pub fn left(&mut self, distance: u16) {
        let mut cycle = self.pins.iter().rev().cycle();

        (0..distance + self.dial).for_each(|_| {
            let _ = cycle.next();
        });

        if let Some(dial) = cycle.next() {
            self.dial = *dial;
        }
    }

    pub fn right(&mut self, distance: u16) {
        let mut cycle = self.pins.iter().cycle();

        (0..distance + self.dial).for_each(|_| {
            let _ = cycle.next();
        });

        if let Some(dial) = cycle.next() {
            self.dial = *dial;
        }
    }

    pub fn zero_passes(&self) -> u16 {
        self.zero_passes
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

    rotations.iter().for_each(|rotation| match rotation {
        Rotation::Left(distance) => safe.left(*distance),
        Rotation::Right(distance) => safe.right(*distance),
    });

    println!("{}", safe.zero_passes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut safe = Safe::new(50, 99);

        safe.right(1);
        assert_eq!(safe.dial, 51);

        safe.right(1);
        assert_eq!(safe.dial, 52);

        safe.left(10);
        assert_eq!(safe.dial, 42);
    }

    #[test]
    fn test_cycle() {
        let a = [1, 2, 3];

        let mut iter = a.into_iter().rev().cycle();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}
