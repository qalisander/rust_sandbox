// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust

use std::fmt::{Display, Formatter};
use itertools::Itertools;

struct Queen(usize, usize);

struct Chessboard(Vec<Queen>, usize); // TODO: prlly turn into slice with lifetime specifier

impl Display for Chessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Chessboard(queens, size) = self;
        let str = queens.iter()
            .sorted_by_key(|Queen(x, y)| y)
            .flat_map(|Queen(x, y)| {
                let mut row = vec!['.'; *size];
                row[*x] = 'Q';
                row.into_iter().chain(std::iter::once('\n'))
            }).collect::<String>();
        write!(f, "Chessboard is:\n{}", str)
    }
}

pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
    let initial_queens_pos = get_initial_pos(n, mandatory_coords);
    
    todo!();
}

fn get_initial_pos(n: usize, mandatory_queen: (usize, usize)) -> Vec<Queen> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{Queen, Chessboard};

    #[test]
    fn smoke_test() {
        let queens = vec![Queen(0, 0), Queen(1,1), Queen(2, 2), Queen(3, 3)];
        println!("{}", Chessboard(queens, 4));
    }
    
    #[test]
    fn basic_tests() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0))];
        for (n, fixed) in basic_tests.into_iter() {
            test_solution(n, fixed);
        }
    }

    #[test]
    fn no_solution_tests() {
        let no_solutions = vec![(2, (0, 0)), (3, (2, 0)), (6, (0, 0))];
        for (n, fixed) in no_solutions.into_iter() {
            test_no_solution(n, fixed);
        }
    }

    fn check_board(board: &[u8], n: usize, fixed: (usize, usize)) {
        let mut offset = 0;
        let mut num_queens = 0;
        let mut queens: Vec<Option<usize>> = vec![None; n];
        #[allow(clippy::needless_range_loop)] // should be more clear to keep the `y` indexing
        for y in 0..n {
            for x in 0..n {
                match board[offset] {
                    b'Q' => {
                        assert!(
                            queens[y].is_none(),
                            "The board should not have horizontal attacks between Queens"
                        );
                        num_queens += 1;
                        queens[y] = Some(x);
                    }
                    b'.' => {}
                    _ => panic!("The board has invalid character"),
                }
                offset += 1;
            }

            assert_eq!(
                board[offset], b'\n',
                "The board has missing/incorrect characters"
            );
            offset += 1;
        }

        assert_eq!(num_queens, n, "The number of queens should be equal to size");

        let queens = queens.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        assert!(
            queens[fixed.1] == fixed.0,
            "The mandatory queen is not in the required position"
        );

        // Check no attacks
        let mut taken_cols = vec![false; n];
        let mut taken_diag1 = vec![false; 2 * n];
        let mut taken_diag2 = vec![false; 2 * n];
        for row in 0..n {
            let col = queens[row];
            assert!(
                !taken_cols[col],
                "The board has vertical attacks between Queens"
            );
            assert!(
                !taken_diag1[col + row],
                "The board has diag1 attacks between Queens"
            );
            assert!(
                !taken_diag2[n + col - row - 1],
                "The board has diag2 attacks between Queens"
            );
            taken_cols[col] = true;
            taken_diag1[col + row] = true;
            taken_diag2[n + col - row - 1] = true;
        }
    }

    fn test_solution(n: usize, fixed: (usize, usize)) {
        if let Some(board) = solve_n_queens(n, fixed) {
            check_board(&board.as_bytes(), n, fixed);
        } else {
            panic!("Returned None when there's a solution");
        }
    }

    fn test_no_solution(n: usize, fixed: (usize, usize)) {
        assert_eq!(
            solve_n_queens(n, fixed),
            None,
            "Expected None when no solution is possible"
        );
    }
}