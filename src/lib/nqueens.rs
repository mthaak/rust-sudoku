use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::lib::exact_cover::{ExactCoverProblem, ExactCoverSolution};

pub(crate) struct NQueensProblem {
    n: u16,
}

impl NQueensProblem {
    fn new(n: u16) -> NQueensProblem {
        NQueensProblem { n }
    }
}

fn convert_to_exact_cover_problem(nqueens_problem: &NQueensProblem) -> ExactCoverProblem {
    let n = nqueens_problem.n as u8;

    let mut required_items: Vec<String> = Vec::new();
    let mut covered_by: HashMap<String, Vec<String>> = HashMap::new();
    // One item for every row (n)
    for row in 0..n {
        let row_item_name = row_to_name(row);
        required_items.push(row_item_name.clone());
        covered_by.insert(row_item_name, Vec::new());
    }
    // One item for every column (n)
    for col in 0..n {
        let col_item_name = col_to_name(col);
        required_items.push(col_item_name.clone());
        covered_by.insert(col_item_name, Vec::new());
    }
    // One optional item for every diagonal in both directions 2 * (2n - 1)
    for diag1 in (-(n as i16) + 1)..(n as i16) {
        let diag1_item_name = diag1_to_name(diag1);
        covered_by.insert(diag1_item_name, Vec::new());
    }
    for diag2 in (-(n as i16) + 1)..(n as i16) {
        let diag2_item_name = diag2_to_name(diag2);
        covered_by.insert(diag2_item_name, Vec::new());
    }

    // One option for every possible position (64)
    for row in 0..n {
        for col in 0..n {
            let option_name = format!("{}{}", col_to_name(col), row_to_name(row));
            let row_item_name = row_to_name(row);
            let col_item_name = col_to_name(col);
            let diag1_item_name = diag1_to_name(col_row_to_diag1(col, row));
            let diag2_item_name = diag2_to_name(col_row_to_diag2(col, row, n));
            covered_by.get_mut(&row_item_name).unwrap().push(option_name.clone());
            covered_by.get_mut(&col_item_name).unwrap().push(option_name.clone());
            covered_by.get_mut(&diag1_item_name).unwrap().push(option_name.clone());
            covered_by.get_mut(&diag2_item_name).unwrap().push(option_name.clone());
        }
    }
    return ExactCoverProblem::new(required_items, vec![], covered_by);
}

fn col_to_name(col: u8) -> String {
    format!("{}", (col + b'a') as char)
}

fn name_to_col(name: String) -> u8 {
    name.chars().nth(0).unwrap() as u8 - b'a'
}

fn row_to_name(row: u8) -> String {
    format!("{}", row + 1)
}

fn name_to_row(name: String) -> u8 {
    name.parse::<u8>().unwrap() - 1
}

fn col_row_to_diag1(col: u8, row: u8) -> i16 {
    return (col as i16) - (row as i16);
}

fn diag1_to_name(diag1: i16) -> String {
    return format!("/{}", diag1);
}

fn col_row_to_diag2(col: u8, row: u8, n: u8) -> i16 {
    return (col as i16) + (row as i16) - (n as i16 - 1);
}

fn diag2_to_name(diag2: i16) -> String {
    return format!("\\{}", diag2);
}

pub(crate) struct NQueensSolution {
    board: Board,
}

#[derive(Debug, PartialEq)]
pub struct Board(Vec<Vec<u8>>);

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
                    out.push_str("Q");
                }
            }

            out.push_str("\n");
        }

        write!(f, "{}", out)
    }
}

fn convert_to_nqueens_solution(solution: ExactCoverSolution) -> NQueensSolution {
    // Find largest row number
    let n = solution.selected_options.iter().map(|option| {
        return option.chars().nth(1).unwrap().to_digit(10).unwrap();
    }).max().unwrap();
    let mut board = Board(vec![vec![0; n as usize]; n as usize]);
    for option in solution.selected_options {
        let col = name_to_col(option.chars().nth(0).unwrap().to_string());
        let row = name_to_row(option.chars().nth(1).unwrap().to_string());
        board.0[row as usize][col as usize] = 1;
    }
    NQueensSolution { board }
}

/**
 * Solve n-queens problem with exact cover.
 */
pub(crate) fn solve_nqueens_problem_with_exact_cover(nqueens_problem: &NQueensProblem) -> Option<NQueensSolution> {
    let exact_cover_problem = convert_to_exact_cover_problem(nqueens_problem);

    let solution = exact_cover_problem.solve();

    solution.map(convert_to_nqueens_solution)
}

/**
 * Count all solutions to n-queens problem with exact cover.
 */
pub(crate) fn count_all_nqueens_solutions_with_exact_cover(nqueens_problem: &NQueensProblem) -> u64 {
    let exact_cover_problem = convert_to_exact_cover_problem(nqueens_problem);

    exact_cover_problem.count_all_solutions()
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_fmt() {
        let board = Board(vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0],
        ]);

        let fmt = format!("{}", board);

        let expected_fmt = "\
Q.......
......Q.
....Q...
.......Q
.Q......
...Q....
.....Q..
..Q.....
".to_string();
        assert_eq!(fmt, expected_fmt)
    }

    #[test]
    fn test_nqueens_problem() {
        let nqueens_problem = NQueensProblem::new(8);

        let solution = solve_nqueens_problem_with_exact_cover(&nqueens_problem);

        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_valid_nqueens_solution(solution);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 0)]
    #[case(3, 0)]
    #[case(4, 2)]
    #[case(5, 10)]
    #[case(6, 4)]
    #[case(7, 40)]
    #[case(8, 92)]
    #[case(9, 352)]
    #[case(10, 724)]
    fn test_nqueens_problem_count_all(#[case] input: u16, #[case] expected: u64) {
        let nqueens_problem = NQueensProblem::new(input);

        let count = count_all_nqueens_solutions_with_exact_cover(&nqueens_problem);

        assert_eq!(count, expected);
    }
}

fn assert_valid_nqueens_solution(nqueens_solution: NQueensSolution) {
    let board = nqueens_solution.board;
    let n = board.0.len();
    let mut row_counts = vec![0; n];
    let mut col_counts = vec![0; n];
    let mut diag1_counts = vec![0; 2 * n - 1];
    let mut diag2_counts = vec![0; 2 * n - 1];
    for row in 0..n {
        for col in 0..n {
            if board.0[row][col] == 1 {
                row_counts[row] += 1;
                col_counts[col] += 1;
                let diag1 = col_row_to_diag1(col as u8, row as u8);
                let diag2 = col_row_to_diag2(col as u8, row as u8, n as u8);
                diag1_counts[(diag1 + (n as i16 - 1)) as usize] += 1;
                diag2_counts[(diag2 + (n as i16 - 1)) as usize] += 1;
            }
        }
    }
    for i in 0..n {
        assert_eq!(row_counts[i], 1);
        assert_eq!(col_counts[i], 1);
    }
    for i in 0..2 * n - 1 {
        assert!(diag1_counts[i] <= 1);
        assert!(diag2_counts[i] <= 1);
    }
}
