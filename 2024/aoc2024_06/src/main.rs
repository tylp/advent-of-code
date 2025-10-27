use clap::Parser;
use std::collections::HashSet;
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

    let (part1, part2) = resolve(&input);
    println!(
        "#### Solutions ####\n Part 1: {:?}\n Part 2: {:?}",
        part1, part2
    );
}

fn resolve(lines: &str) -> (i32, i32) {
    let mut matrix = parse_input(lines);
    let mut guard = initialize_guards(&matrix);

    while guard.visible {
        guard.patroll(&mut matrix);
    }

    let part1 = guard.visits_by_position();

    println!("Part 1: {:?} #########################", part1);

    // Add an obstacle to each tile one by one.
    // If the guard visits the same tile in the same direction, then it's a loop.
    // If the guards is out, it is not
    let mut obstacles = 0;
    let mut matrix = parse_input(lines);
    let mut guard = initialize_guards(&matrix);

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let original = matrix[y][x];

            if original == '^' {
                continue;
            }

            // println!("Testing with obstacle at: ({:?}, {:?})", x, y);
            matrix[y][x] = OBSTACLE;
            print!("\rTesting with obstacle at: ({:?}, {:?})", x, y);

            // for row in matrix.iter() {
            //     println!("{:?}", row);
            // }

            while guard.visible {
                guard.patroll(&mut matrix);

                if guard.has_visited_twice() {
                    break;
                }
            }

            if !guard.visible {
                //println!("Guard is out");
            }

            if guard.has_visited_twice() {
                //println!("Guard is looping");
                obstacles += 1;
            }

            matrix = parse_input(lines);
            guard = initialize_guards(&matrix);
        }
    }

    (part1, obstacles)
}

type Matrix = Vec<Vec<char>>;
const OBSTACLE: char = '#';
const EMPTY: char = '.';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GuardTile {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    visited: HashSet<GuardTile>,
    obstacles: HashSet<Position>,
    obstacles_loop: HashSet<Position>,
    visible: bool,
}

impl Guard {
    fn has_visited_twice(&self) -> bool {
        !self.obstacles_loop.is_empty()
    }

    fn visits_by_position(&self) -> i32 {
        self.visited
            .iter()
            .map(|pos| pos.position.clone())
            .collect::<HashSet<_>>()
            .len() as i32
    }

    // Updates the matrix and guards by one tick
    fn patroll(&mut self, matrix: &mut Matrix) {
        let guard_direction = &self.direction;

        match guard_direction {
            Direction::Up => self.move_up(matrix),
            Direction::Down => self.move_down(matrix),
            Direction::Left => self.move_left(matrix),
            Direction::Right => self.move_right(matrix),
        }
    }

    pub fn move_forward(&mut self, matrix: &mut Matrix, new_position: Position) {
        let current_position = &self.position;
        let guard_tile = GuardTile {
            position: new_position.clone(),
            direction: self.direction.clone(),
        };

        // If the new position is out of the matrix, then the gard is out of the area
        if !position_is_in_matrix(&new_position, matrix) {
            matrix[current_position.y as usize][current_position.x as usize] = EMPTY;
            self.visible = false;
            return;
        }

        // If the new position is an obstacle, then the guard should rotatate to the left
        if position_is_obstacle(&new_position, matrix) {
            self.obstacles.insert(new_position.clone());
            self.rotate();
            return;
        }

        // Otherwise, move forward
        matrix[current_position.y as usize][current_position.x as usize] = EMPTY;
        matrix[new_position.y as usize][new_position.x as usize] =
            char::from(self.direction.clone());
        self.position = new_position.clone();

        // If we visited this tile already in the same direction, assume that an obstacle here will make it loop
        if self.visited.contains(&guard_tile) {
            //println!("Already visited: {:?}", guard_tile);
            self.obstacles_loop.insert(new_position);
            return;
        }

        //println!("[{:?}] - Visited: {:?}", self.visited.len() + 1, guard_tile);
        self.visited.insert(guard_tile);
    }

    pub fn move_up(&mut self, matrix: &mut Matrix) {
        let new_position = Position {
            x: self.position.x,
            y: self.position.y - 1,
        };

        self.move_forward(matrix, new_position);
    }

    pub fn move_left(&mut self, matrix: &mut Matrix) {
        let new_position = Position {
            x: self.position.x - 1,
            y: self.position.y,
        };

        self.move_forward(matrix, new_position);
    }

    pub fn move_down(&mut self, matrix: &mut Matrix) {
        let new_position = Position {
            x: self.position.x,
            y: self.position.y + 1,
        };

        self.move_forward(matrix, new_position);
    }

    pub fn move_right(&mut self, matrix: &mut Matrix) {
        let new_position = Position {
            x: self.position.x + 1,
            y: self.position.y,
        };

        self.move_forward(matrix, new_position);
    }

    pub fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }
}

fn position_is_obstacle(position: &Position, matrix: &Matrix) -> bool {
    let y = position.y as usize;
    let x = position.x as usize;

    if matrix[y][x] == OBSTACLE {
        return true;
    }

    false
}

fn position_is_in_matrix(position: &Position, matrix: &Matrix) -> bool {
    let x = position.x;
    let y = position.y;

    if x < 0 || y < 0 {
        return false;
    }

    if x >= matrix[0].len() as i16 || y >= matrix.len() as i16 {
        return false;
    }

    true
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn initialize_guards(matrix: &Matrix) -> Guard {
    for (row, cols) in matrix.iter().enumerate() {
        for (col, c) in cols.iter().enumerate() {
            if *c == '^' || *c == 'v' || *c == '<' || *c == '>' {
                let position = Position {
                    x: col as i16,
                    y: row as i16,
                };

                let mut visited = HashSet::new();
                let guard_tile = GuardTile {
                    position: position.clone(),
                    direction: Direction::from(*c),
                };

                visited.insert(guard_tile);

                return Guard {
                    position,
                    direction: Direction::from(*c),
                    visited,
                    visible: true,
                    obstacles: HashSet::new(),
                    obstacles_loop: HashSet::new(),
                };
            }
        }
    }

    panic!("No guard found in the matrix");
}

fn parse_input(input: &str) -> Matrix {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

        let (part1, part2) = resolve(input);
        assert_eq!(part1, 41);
        assert_eq!(part2, 6);
    }
}
