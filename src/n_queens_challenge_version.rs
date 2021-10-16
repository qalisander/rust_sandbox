// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

//type Queen = (usize, usize); // TODO: made struct, and implement From trait
#[derive(Clone, Copy)]
struct Queen(usize, usize); // TODO: rename to rect_queens

impl From<(usize, DiagonalQueen)> for Queen {
    fn from((size, diag_queen): (usize, DiagonalQueen)) -> Self {
        Self(
            (diag_queen.0 - diag_queen.1 + size - 1) / 2,
            (diag_queen.0 + diag_queen.1 - size + 1) / 2,
        )
    }
}

impl From<(usize, usize)> for Queen {
    fn from(tpl: (usize, usize)) -> Self {
        Self(tpl.0, tpl.1)
    }
}

struct Chessboard(Vec<Queen>, usize); // TODO: prlly turn into slice with lifetime specifier

#[derive(Clone, Copy)]
struct DiagonalQueen(usize, usize);

impl From<(usize, Queen)> for DiagonalQueen {
    fn from((size, queen): (usize, Queen)) -> Self {
        // TODO: replace size - 1 with simple size
        Self(queen.0 + queen.1, size - 1 - queen.0 + queen.1)
    }
}

// TODO: maybe use randomized algo
// TODO: implement display and debug for DiagonalChessboard
// TODO: display of initial DiagonalChessboard and in any moment of time
// TODO: main algo

// |---------|
// |         |
// |    x    |
// |   / \   |
// |  /   \  |
// |-V-----V-|
struct DiagonalChessboard {
    diag0_to1: Vec<Option<usize>>,
    diag1_to0: Vec<Option<usize>>,
    coincident_queens: VecDeque<DiagonalQueen>,
    diag_size: usize,
    size: usize,
}

impl Debug for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let queens = self.diag0_to1.iter().enumerate().filter_map(
            (|(index, opt)| {
                if let Some(diag_1) = opt {
                    let diag_queen = DiagonalQueen(index, *diag_1);
                    Some((self.size, diag_queen).into())
                } else {
                    None
                }
            }),
        );
        writeln!(f, "Chessboard is:")?;
        for Queen(x, y) in queens.sorted_by_key(|Queen(x, y)| *y) {
            writeln!(
                f,
                "{}{}{}",
                ".".repeat(x),
                "Q",
                ".".repeat(self.size - x - 1)
            )?;
        }
        writeln!(f)
    }
}

impl DiagonalChessboard {
    fn from_mandatory_queen(n: usize, mandatory_queen: Queen) -> DiagonalChessboard {
        let diag_size = 2 * n - 1;
        let mut diag_chessboard = DiagonalChessboard {
            diag0_to1: vec![Option::None; diag_size],
            diag1_to0: vec![Option::None; diag_size],
            coincident_queens: VecDeque::new(),
            diag_size,
            size: n,
        };

        let mut vacant_queens_0 = vec![true; n];
        let mut vacant_queens_1 = vec![false; n];

        let mut queen = mandatory_queen;
        loop {
            if diag_chessboard.add_queen((n, queen)).is_ok() {
                vacant_queens_0[queen.0] = false;
                queen.0 = if queen.0 >= 2 {
                    queen.0 - 2
                } else {
                    if (n - mandatory_queen.0) % 2 == 0 {
                        n - 1
                    } else {
                        n - 2
                    }
                };
                queen.1 = (n + queen.1 + 1) % n;
            } else {
                vacant_queens_1[queen.1] = true;
                queen.1 += 1;
            }

            if queen.1 == mandatory_queen.1 {
                break;
            }
        }

        filter_vacant(vacant_queens_0)
            .zip(filter_vacant(vacant_queens_1))
            .for_each(|(diag_0, diag_1)| {
                diag_chessboard.add_coincident_queen((n, Queen(diag_0, diag_1)));
            });

        fn filter_vacant(vacant_queens: Vec<bool>) -> impl Iterator<Item = usize> {
            vacant_queens
                .into_iter()
                .enumerate()
                .filter_map(|(index, is_vacant)| if is_vacant { Some(index) } else { None })
        }

        diag_chessboard
    }

    fn add_queen<T: Into<DiagonalQueen>>(
        &mut self,
        queen: T,
    ) -> Result<DiagonalQueen, DiagonalQueen> {
        let queen = queen.into();
        if self.diag0_to1[queen.0].is_some() || self.diag1_to0[queen.1].is_some() {
            Err(queen)
        } else {
            self.diag0_to1[queen.0] = Some(queen.1);
            self.diag1_to0[queen.1] = Some(queen.0);
            Ok(queen)
        }
    }

    fn add_coincident_queen<T: Into<DiagonalQueen>>(&mut self, queen: T) {
        self.coincident_queens.push_back(queen.into());
    }

    fn get_queens_in_rectangular_coordinates(&self) -> impl Iterator<Item = (Queen, bool)> + '_ {
        self.diag0_to1
            .iter()
            .enumerate()
            .filter_map(
                |(index, opt)| {
                    // TODO: read about option map https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
                    opt.as_ref().map(|diag_1| (DiagonalQueen(index, *diag_1), false))
                }
            )
            .chain(
                self.coincident_queens
                    .iter()
                    .map(|diag_queen| (*diag_queen, true)),
            )
            .map(|(diag_quen, is_vacant)| ((self.size, diag_quen).into(), is_vacant))
    }
}

impl Display for Chessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Chessboard(queens, size) = self;
        writeln!(f, "Chessboard is:")?;
        for Queen(x, y) in queens.iter().sorted_by_key(|Queen(x, y)| y) {
            writeln!(f, "{}{}{}", ".".repeat(*x), "Q", ".".repeat(*size - x - 1))?;
        }
        writeln!(f)
    }
}

pub fn solve_n_queens(n: usize, mandatory_queen: (usize, usize)) -> Option<String> {
    let chessboard = DiagonalChessboard::from_mandatory_queen(n, mandatory_queen.into());
    todo!("Create queens");
}

//....
//.Q..
//....
//Q...

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{Chessboard, Queen};

    #[test]
    fn format_test() {
        let queens = vec![Queen(0, 0), Queen(1, 1), Queen(2, 2), Queen(3, 3)];
        println!("{}", Chessboard(queens, 4));
    }

    #[test]
    fn initial_chessboard_test() {
        todo!("Initial chessboard")
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

        assert_eq!(
            num_queens, n,
            "The number of queens should be equal to size"
        );

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
