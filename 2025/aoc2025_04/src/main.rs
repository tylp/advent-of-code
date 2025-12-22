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
    let matrix = Matrix::from(lines);

    let accessible_rolls_4 = count_accessible_rolls(4, &matrix);

    println!("[Part 1] There are {accessible_rolls_4} accessible rolls. ");
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

// Check wether the cell is accessible
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

    // Example with
    // [R S]
    // [S R]
    //
    // We first test at 0,0
    // Then we want to check
    // 		(0, 0), 	// Top-left
    //      (0, 0), 			// Top-center
    //      (0, 1), 		// Top-right
    //      (0, 0),			// Middle-left
    //      // Ignore self cell
    //      // (row, col),					// Middle-center
    //      (0, 1),					// Middle-right
    //      (1, 0),		// Bottom-left
    //      (1, 0),					// Bottom-center
    //      (1, 1),				// Bottom-right
    //
    // So in reality, we check (0,0) 3 times, (0,1) two times, (1,0) two times and (1,1) once.
    // With this we get 3 hit in (0,0) + 1 hit in (1,1) = 4 hit instead of 2.
    // We actually have to remove duplicates.
    let mut uniques: Vec<(usize, usize)> = Vec::new();
    cells_to_check.iter().for_each(|cell| {
        if !uniques.contains(cell) {
            uniques.push(*cell);
        }
    });

    // Once we reduced it to (0,0), (0,1), (1,0), (1,1)
    // Retreive only the cells that are within the matrix bounds and counts the items
    // that are Rolls.
    let rolls_in_range = cells_to_check
        .iter()
        .filter(|cell| cell.0 != row && cell.1 != col) // Filter out self cell
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
    use crate::{Input, Matrix, count_accessible_rolls};

    #[test]
    fn count_accessible_rolls_2b2() {
        let matrix = Matrix::from(Input(vec!["@.".into(), ".@".into()]));

        assert_eq!(count_accessible_rolls(4, &matrix), 2);
    }

    /// Test the part 1 example.
    ///
    /// ..@@.@@@@.
    /// @@@.@.@.@@
    /// @@@@@.@.@@
    /// @.@@@@..@.
    /// @@.@@@@.@@
    /// .@@@@@@@.@
    /// .@.@.@.@@@
    /// @.@@@.@@@@
    /// .@@@@@@@@.
    /// @.@.@@@.@
    ///
    /// ..xx.xx@x.
    /// x@@.@.@.@@
    /// @@@@@.x.@@
    /// @.@@@@..@.
    /// x@.@@@@.@x
    /// .@@@@@@@.@
    /// .@.@.@.@@@
    /// x.@@@.@@@@
    /// .@@@@@@@@.
    /// x.x.@@@.x.
    #[test]
    fn count_accessible_rolls_with_threshold_of_4_example() {
        let threshold = 4;
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

        assert_eq!(count_accessible_rolls(threshold, &matrix), 13);
    }
}
