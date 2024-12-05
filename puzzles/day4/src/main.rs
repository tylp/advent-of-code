use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let file = File::open(&args.input_file).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    reader.lines().for_each(|line| {
        lines.push(line.unwrap());
    });

    println!("Solution: {:?}", resolve(&lines));
}

fn resolve(lines: &[String]) -> i32 {
    let mut matrix = matrix(lines);
    let mut sum = horizontal_count(&matrix);

    sum += diagonal(&matrix);

    rotate(&mut matrix);
    sum += horizontal_count(&matrix);

    sum
}

type Matrix = Vec<Vec<char>>;

fn matrix(lines: &[String]) -> Matrix {
    let mut matrix: Matrix = Vec::new();

    lines.iter().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    });

    matrix
}

/// Rotate a matrix 90 degrees clockwise
fn rotate(matrix: &mut Matrix) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }

    let cols = matrix[0].len();
    let mut rotated = vec![vec![' '; rows]; cols];

    (0..rows).for_each(|i| {
        (0..cols).for_each(|j| {
            rotated[j][rows - i - 1] = matrix[i][j];
        });
    });

    *matrix = rotated;
}

fn horizontal_count(matrix: &Matrix) -> i32 {
    let acc = matrix.iter().fold(0, |acc, row| {
        acc + row.iter().collect::<String>().matches("XMAS").count() as i32
            + row.iter().collect::<String>().matches("SAMX").count() as i32
    });

    acc
}

fn diagonal(matrix: &Matrix) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::diagonal;
    use crate::horizontal_count;
    use crate::matrix;
    use crate::resolve;
    use crate::rotate;

    #[test]
    fn test_diagonal() {
        let lines = vec![
            "X...X".to_string(),
            ".M.M.".to_string(),
            "..A..".to_string(),
            ".S.S.".to_string(),
        ];

        let matrix = matrix(&lines);
        assert_eq!(diagonal(&matrix), 4);
    }

    #[test]
    fn test_rotate() {
        let lines = vec![
            "XXMAS".to_string(),
            "MAAAA".to_string(),
            "AAAAA".to_string(),
            "SAAAA".to_string(),
        ];

        let mut matrix = matrix(&lines);
        rotate(&mut matrix);

        assert_eq!(matrix[0], vec!['S', 'A', 'M', 'X']);
        assert_eq!(matrix[1], vec!['A', 'A', 'A', 'X']);
        assert_eq!(matrix[2], vec!['A', 'A', 'A', 'M']);
        assert_eq!(matrix[3], vec!['A', 'A', 'A', 'A']);
        assert_eq!(matrix[4], vec!['A', 'A', 'A', 'S']);
    }

    #[test]
    fn test_horizontal_count() {
        let lines = vec!["MSAMXMSMSAXMASAMX".to_string()];
        let matrix = matrix(&lines);
        assert_eq!(horizontal_count(&matrix), 3);
    }

    #[test]
    fn test_resolve_1() {
        let lines = vec![
            "X...SA".to_string(),
            "M.XMAS".to_string(),
            "A...MA".to_string(),
            "SSAMXM".to_string(),
            "S.A.XX".to_string(),
        ];

        assert_eq!(resolve(&lines), 5);
    }

    #[test]
    fn test_resolve_2() {
        let lines = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        assert_eq!(resolve(&lines), 18);
    }
}
