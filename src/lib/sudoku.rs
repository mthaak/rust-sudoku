use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

