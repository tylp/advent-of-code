#[derive(Debug, PartialEq, Eq)]
enum Item {
    Roll,
    Space,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Roll,
            '.' => Self::Space,
            _ => panic!("Unexpected char"),
        }
    }
}

impl From<&Item> for char {
    fn from(val: &Item) -> Self {
        match val {
            Item::Roll => '@',
            Item::Space => '.',
        }
    }
}

struct Input(Vec<String>);

/// Matrix is organized by rows first, then colums.
///
/// Accessing matrix[2][1] will return the item at
/// the 3rd row and 2nd column.
type Matrix = Vec<Vec<Item>>;

impl From<Input> for Matrix {
    fn from(value: Input) -> Self {
        value
            .0
            .iter()
            .map(|value| value.chars().map(Item::from).collect())
            .collect()
    }
}

fn main() {
    let lines = Input(aoc::init());
    let mut matrix = Matrix::from(lines);

    let accessible_rolls = count_accessible_rolls(4, &matrix);
    let removable_rolls = count_removable_rolls(4, &mut matrix);

    println!("[Part 1] There are {accessible_rolls} accessible rolls.");
    println!("[Part 2] There are {removable_rolls} removable rolls.");
}

/// Returns the number of accessible rolls in a grid.
///
/// A roll is accessible when there are fewer than `access_threshold`
/// rolls in the 8 surrounding cells.
///
/// For example, with the following grid (@ = roll, . is empty), and
/// threshold of 4:
///
/// | _ | 0 | 1 | 2 | 3 |
/// |---|---|---|---|---|
/// | 0 | . | . | @ | @ |
/// | 1 | @ | @ | @ | . |
/// | 2 | @ | @ | @ | @ |
/// | 3 | @ | . | @ | @ |
///
/// The accessible rolls are `[
/// (1;1), (1;2),
/// (2;0), (2;1), (2;2), (2;3)
/// (3;1), (3;2), (3;3)
/// ]`.
///
/// | _ | 0 | 1 | 2 | 3 |
/// |---|---|---|---|---|
/// | 0 | . | . | x | x |
/// | 1 | x | @ | @ | . |
/// | 2 | @ | @ | @ | @ |
/// | 3 | x | . | @ | x |
///
/// Wich counts as 5 accessible rolls (marked by `x`).
fn count_accessible_rolls(threshold: u16, matrix: &Matrix) -> usize {
    matrix.iter().enumerate().fold(0, |acc, (row, cols)| {
        acc + cols
            .iter()
            .enumerate()
            .filter(|(col, item)| {
                **item == Item::Roll && is_roll_accessible(row, *col, threshold, matrix)
            })
            .count()
    })
}

/// Count the removable rolls.
fn count_removable_rolls(threshold: u16, matrix: &mut Matrix) -> usize {
    let mut removed = 0;
    let mut rolls_to_remove: Vec<(usize, usize)> = Vec::new();

    while count_accessible_rolls(threshold, matrix) != 0 {
        // Transform the cached rolls into spaces and empty the cache
        rolls_to_space(&rolls_to_remove, matrix);
        rolls_to_remove = Vec::new();

        // Count the accessible rolls in the current matrix.
        matrix.iter().enumerate().for_each(|(row, items)| {
            items.iter().enumerate().for_each(|(col, item)| {
                // If it is accessible, we remove it for the next iteration
                if *item == Item::Roll && is_roll_accessible(row, col, threshold, matrix) {
                    rolls_to_remove.push((row, col));
                    removed += 1;
                }
            });
        });
    }

    removed
}

/// Change the given rolls into space in the Matrix.
fn rolls_to_space(rolls: &Vec<(usize, usize)>, matrix: &mut Matrix) {
    for (row, col) in rolls {
        // find the associated item and swap it into space
        if let Some(items) = matrix.get_mut(*row)
            && let Some(item) = items.get_mut(*col)
        {
            *item = Item::Space;
        }
    }
}

// Check wether the cell is accessible and returns a tuple containing wether it is accessible and its coordinates.
fn is_roll_accessible(row: usize, col: usize, threshold: u16, matrix: &Matrix) -> bool {
    // Guard the negative value on usize
    let previous_row = match row {
        0 => 0,
        _ => row - 1,
    };

    let previous_col = match col {
        0 => 0,
        _ => col - 1,
    };

    // Get all the cells to check around it
    let cells_to_check: [(usize, usize); 8] = [
        (previous_row, previous_col), // Top-left
        (previous_row, col),          // Top-center
        (previous_row, col + 1),      // Top-right
        (row, previous_col),          // Middle-left
        (row, col + 1),               // Middle-right
        (row + 1, previous_col),      // Bottom-left
        (row + 1, col),               // Bottom-center
        (row + 1, col + 1),           // Bottom-right
    ];

    // We actually have to remove duplicates.
    let mut uniques: Vec<(usize, usize)> = Vec::new();
    cells_to_check.iter().for_each(|cell| {
        if !uniques.contains(cell) {
            uniques.push(*cell);
        }
    });

    // Retreive only the cells that are within the matrix bounds and counts the items
    // that are Rolls.
    let rolls_in_range = uniques
        .iter()
        .filter(|cell| **cell != (row, col)) // Filter out self cell
        .fold(0, |acc, e| {
            if let Some(item) = extract_item_from_matrix(e.0, e.1, matrix)
                && *item == Item::Roll
            {
                return acc + 1;
            }

            acc
        });

    rolls_in_range < threshold
}

/// Returns the item at the given index if it exists. None otherwise.
fn extract_item_from_matrix(row: usize, col: usize, matrix: &Matrix) -> Option<&Item> {
    if let Some(row) = &matrix.get(row)
        && let Some(item) = row.get(col)
    {
        return Some(item);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{Input, Matrix, count_accessible_rolls, count_removable_rolls};

    #[test]
    fn count_accessible_rolls_test() {
        let matrix = Matrix::from(Input(vec![
            "..@@.@@@@.".into(),
            "@@@.@.@.@@".into(),
            "@@@@@.@.@@".into(),
            "@.@@@@..@.".into(),
            "@@.@@@@.@@".into(),
            ".@@@@@@@.@".into(),
            ".@.@.@.@@@".into(),
            "@.@@@.@@@@".into(),
            ".@@@@@@@@.".into(),
            "@.@.@@@.@".into(),
        ]));

        assert_eq!(count_accessible_rolls(4, &matrix), 13);
    }

    #[test]
    fn count_removable_rolls_test() {
        let mut matrix = Matrix::from(Input(vec![
            "..@@.@@@@.".into(),
            "@@@.@.@.@@".into(),
            "@@@@@.@.@@".into(),
            "@.@@@@..@.".into(),
            "@@.@@@@.@@".into(),
            ".@@@@@@@.@".into(),
            ".@.@.@.@@@".into(),
            "@.@@@.@@@@".into(),
            ".@@@@@@@@.".into(),
            "@.@.@@@.@".into(),
        ]));

        assert_eq!(count_removable_rolls(4, &mut matrix), 43);
    }
}
