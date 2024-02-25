extern crate core;

use crate::lib::sudoku::{Board, convert_to_exact_cover_problem, convert_to_sudoku_solution};

mod lib;

fn main() {
    let filename = "data/sudoku.txt";
    let result = Board::read_from_file(filename);
    match(result) {
        Ok(board) => {
            println!("Board:");
            println!("{}", board);
            let exact_cover_problem = convert_to_exact_cover_problem(&board);
            let solution = exact_cover_problem.solve();
            let solution = solution.map(convert_to_sudoku_solution);

            match solution {
                Some(solution) => {
                    println!("Solution:");
                    println!("{}", solution);
                }
                None => {
                    println!("No solution found");
                }
            }
        }
        Err(e) => {
            // println!("Error reading file: {}", e.);
        }
    }
}
