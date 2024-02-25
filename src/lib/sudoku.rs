use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::lib::exact_cover::{ExactCoverProblem, ExactCoverSolution};

#[derive(Debug, PartialEq, Clone)]
pub struct Board(Vec<Vec<u8>>);

// BoardReadError is a custom error type for errors that occur when reading a board from a file.
#[derive(Debug, PartialEq)]
pub enum BoardReadError {
    FileReadError,
    InvalidCharacter,
    InvalidSize,
}

impl Board {
    pub fn read_from_file(filepath: &str) -> Result<Self, BoardReadError> {
        let file = File::open(filepath);
        if file.is_err() {
            return Err(BoardReadError::FileReadError);
        }
        let reader = BufReader::new(file.unwrap());

        let mut vecs = vec![vec![0; 9]; 9];
        let mut i = 0;
        for result in reader.lines() {
            match result {
                Ok(s) => {
                    if s.len() == 0 {
                        continue;
                    }

                    let mut j = 0;
                    for char in s.chars() {
                        if char == ' ' {} else if char == '.' {
                            j = j + 1
                        } else if char.is_digit(10) {
                            if i >= 9 || j >= 9 {
                                return Err(BoardReadError::InvalidSize);
                            }
                            char.to_digit(10).map(|digit| {
                                vecs[i][j] = digit as u8;
                                j = j + 1
                            });
                        } else {
                            return Err(BoardReadError::InvalidCharacter);
                        }
                    }
                    if j < 9 {
                        return Err(BoardReadError::InvalidSize);
                    }

                    i = i + 1;
                }
                Err(_) => {
                    return Err(BoardReadError::FileReadError);
                }
            }
        }
        if i < 9 {
            return Err(BoardReadError::InvalidSize);
        }

        let board = Board(vecs);
        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut out = String::new();

        for row_idx in 0..self.0.len() {
            let row = &self.0[row_idx];
            for cell_idx in 0..row.len() {
                let cell = &row[cell_idx];
                if *cell == 0 {
                    out.push_str(".");
                } else {
                    out.push_str(&cell.to_string());
                }
                if cell_idx == 2 || cell_idx == 5 {
                    out.push_str(" ");
                }
            }

            out.push_str("\n");
            if row_idx == 2 || row_idx == 5 {
                out.push_str("\n");
            }
        }

        write!(f, "{}", out)
    }
}

pub fn convert_to_exact_cover_problem(board: &Board) -> ExactCoverProblem {
    let mut required_items: Vec<&str> = Vec::new();
    // One item for each cell (81) because each cell must have a digit
    for i in 0..9 {
        for j in 0..9 {
            required_items.push(Box::leak(cell_item_to_name(i as u8, j as u8).into_boxed_str()));
        }
    }
    // One item for every digit in every row (9 * 9) because each digit must appear in each row
    for i in 0..9 {
        for d in 1..10 {
            required_items.push(Box::leak(row_item_to_name(i as u8, d as u8).into_boxed_str()));
        }
    }
    // One item for every digit in every column (9 * 9) because each digit must appear in each column
    for i in 0..9 {
        for d in 1..10 {
            required_items.push(Box::leak(col_item_to_name(i as u8, d as u8).into_boxed_str()));
        }
    }
    // One item for every digit in every block (9 * 9) because each digit must appear in each block
    for i in 0..9 {
        for d in 1..10 {
            required_items.push(Box::leak(block_item_to_name(i as u8, d as u8).into_boxed_str()));
        }
    }
    // One item for initial state (1) to ensure that the initial state is preserved
    // required_items.push(initial_state_item_name);

    let mut covered_by: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut required_options: Vec<&str> = Vec::new();
    // One option for every possible digit in every cell (81 * 9) because each cell must have a digit
    for i in 0..9 {
        for j in 0..9 {
            for d in 1..10 {
                let option_name = cell_option_to_name(i as u8, j as u8, d);
                covered_by.entry(Box::leak(cell_item_to_name(i as u8, j as u8).into_boxed_str())).or_insert(Vec::new()).push(Box::leak(option_name.clone().into_boxed_str()));
                covered_by.entry(Box::leak(row_item_to_name(i as u8, d).into_boxed_str())).or_insert(Vec::new()).push(Box::leak(option_name.clone().into_boxed_str()));
                covered_by.entry(Box::leak(col_item_to_name(j as u8, d).into_boxed_str())).or_insert(Vec::new()).push(Box::leak(option_name.clone().into_boxed_str()));
                covered_by.entry(Box::leak(block_item_to_name(cell_to_block(i as u8, j as u8), d).into_boxed_str())).or_insert(Vec::new()).push(Box::leak(option_name.clone().into_boxed_str()));

                if board.0[i][j] == d {
                    required_options.push(Box::leak(option_name.into_boxed_str()));
                }
            }
        }
    }
    // One option for the initial state (1) to ensure that the initial state is preserved
    // covered_by.entry(initial_state_item_name).or_insert(Vec::new()).push(initial_state_option_name);

    return ExactCoverProblem::new(required_items.clone(), required_options, covered_by);
}

fn cell_item_to_name(row: u8, col: u8) -> String {
    return format!("r{}c{}", row, col);
}

fn row_item_to_name(row: u8, digit: u8) -> String {
    return format!("r{}d{}", row, digit);
}

fn col_item_to_name(col: u8, digit: u8) -> String {
    return format!("c{}d{}", col, digit);
}

fn cell_to_block(row: u8, col: u8) -> u8 {
    return (row / 3 * 3 + col / 3) as u8;
}

fn block_item_to_name(block: u8, digit: u8) -> String {
    return format!("b{}d{}", block, digit);
}

const initial_state_item_name: &str = "init";

fn cell_option_to_name(row: u8, col: u8, digit: u8) -> String {
    return format!("r{}c{}d{}", row, col, digit);
}

const initial_state_option_name: &str = "init";

fn name_to_cell_option(name: &str) -> (u8, u8, u8) {
    let mut chars = name.chars();
    let row = chars.nth(1).unwrap().to_digit(10).unwrap() as u8;
    let col = chars.nth(1).unwrap().to_digit(10).unwrap() as u8;
    let digit = chars.nth(1).unwrap().to_digit(10).unwrap() as u8;
    return (row, col, digit);
}

pub fn convert_to_sudoku_solution(solution: ExactCoverSolution) -> Board {
    let mut board = vec![vec![0; 9]; 9];
    for option in solution.selected_options {
        if option == initial_state_option_name {
            continue;
        }
        let (row, col, digit) = name_to_cell_option(option);
        board[row as usize][col as usize] = digit;
    }
    return Board(board);
}

/**
 * Solve Sudoku with exact cover.
 */
pub(crate) fn solve_sudoku_with_exact_cover<'a>(board: &Board) -> Option<Board> {
    let exact_cover_problem = convert_to_exact_cover_problem(board);

    let solution = exact_cover_problem.solve();

    solution.map(convert_to_sudoku_solution)
}

fn get_board1() -> Board {
    return Board(vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 7, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]);
}

fn get_board1_solved() -> Board {
    return Board(vec![
        vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
        vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
        vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
        vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
        vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
        vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
        vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
        vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
        vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    fn enable_logging() {
        std::env::set_var("RUST_LOG", "info");
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_read_from_file() {
        let file_path = "data/sudoku.txt";

        let board = Board::read_from_file(file_path);

        let expected_board = get_board1();
        assert_eq!(board.unwrap(), expected_board);
    }

    #[test]
    fn test_read_from_file_no_spaces() {
        let file_path = "data/sudoku_no_spaces.txt";

        let board = Board::read_from_file(file_path);

        let expected_board = get_board1();
        assert_eq!(board.unwrap(), expected_board);
    }

    #[test]
    fn test_read_from_file_no_newlines() {
        let file_path = "data/sudoku_no_newlines.txt";

        let board = Board::read_from_file(file_path);

        let expected_board = get_board1();
        assert_eq!(board.unwrap(), expected_board);
    }

    #[test]
    fn test_read_from_file_extra_spaces() {
        let file_path = "data/sudoku_extra_spaces.txt";

        let board = Board::read_from_file(file_path);

        let expected_board = get_board1();
        assert_eq!(board.unwrap(), expected_board);
    }

    #[test]
    fn test_read_from_file_extra_newlines() {
        let file_path = "data/sudoku_extra_newlines.txt";

        let board = Board::read_from_file(file_path);

        let expected_board = get_board1();
        assert_eq!(board.unwrap(), expected_board);
    }

    #[test]
    fn test_read_from_file_invalid_path() {
        let file_path = "data/sudoku_invalid_path.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::FileReadError));
    }

    #[test]
    fn test_read_from_file_invalid_file() {
        let file_path = "data/sudoku_invalid.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::FileReadError));
    }

    #[test]
    fn test_read_from_file_too_wide() {
        let file_path = "data/sudoku_too_wide.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::InvalidSize));
    }

    #[test]
    fn test_read_from_file_too_long() {
        let file_path = "data/sudoku_too_long.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::InvalidSize));
    }

    #[test]
    fn test_read_from_file_missing_character() {
        let file_path = "data/sudoku_missing_character.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::InvalidSize));
    }

    #[test]
    fn test_read_from_file_invalid_character() {
        let file_path = "data/sudoku_invalid_character.txt";

        let board = Board::read_from_file(file_path);

        assert_eq!(board, Err(BoardReadError::InvalidCharacter));
    }

    #[test]
    fn test_fmt() {
        let board = get_board1();

        let fmt = format!("{}", board);

        let expected_fmt = "\
53. .7. ...
6.. 195 ...
.98 ... .6.

8.. .6. ..3
4.. 8.3 ..1
7.. .2. ..6

.6. ..7 28.
... 419 ..5
... .8. .79
".to_string();
        assert_eq!(fmt, expected_fmt)
    }

    #[test]
    fn test_solve_sudoku_with_exact_cover() {
        let board = get_board1();

        let solution = solve_sudoku_with_exact_cover(&board);

        assert!(solution.is_some());
        let expected_solution = get_board1_solved();
        assert_eq!(solution.clone().unwrap(), expected_solution);
        assert_valid_sudoku_solution(solution.clone().unwrap());
    }

    #[rstest]
    #[case("sudoku_easy.txt")]
    #[case("sudoku_medium.txt")]
    #[case("sudoku_hard.txt")]
    #[case("sudoku_hardest.txt")]
    #[case("sudoku_evil.txt")]
    fn test_solve_sudoku_different_difficulties(#[case] filename: &str) {
        let board = Board::read_from_file(&format!("data/{}", filename)).unwrap();

        let solution = solve_sudoku_with_exact_cover(&board);

        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_valid_sudoku_solution(solution);
    }
}

fn assert_valid_sudoku_solution(board: Board) {
    // Check rows
    for i in 0..9 {
        let mut digits = vec![false; 9];
        for j in 0..9 {
            let digit = board.0[i][j];
            assert_ne!(digit, 0, "Row {} has a cell with no digit", i);
            assert!(!digits[(digit - 1) as usize], "Row {} has a duplicate digit {}", i, digit);
            digits[(digit - 1) as usize] = true;
        }
    }

    // Check columns
    for j in 0..9 {
        let mut digits = vec![false; 9];
        for i in 0..9 {
            let digit = board.0[i][j];
            assert_ne!(digit, 0, "Column {} has a cell with no digit", j);
            assert!(!digits[(digit - 1) as usize], "Column {} has a duplicate digit {}", j, digit);
            digits[(digit - 1) as usize] = true;
        }
    }

    // Check blocks
    for block in 0..9 {
        let mut digits = vec![false; 9];
        for i in (block / 3 * 3)..(block / 3 * 3 + 3) {
            for j in (block % 3 * 3)..(block % 3 * 3 + 3) {
                let digit = board.0[i][j];
                assert_ne!(digit, 0, "Block {} has a cell with no digit", block);
                assert!(!digits[(digit - 1) as usize], "Block {} has a duplicate digit {}", block, digit);
                digits[(digit - 1) as usize] = true;
            }
        }
    }
}

