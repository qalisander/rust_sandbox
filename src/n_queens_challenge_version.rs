// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::iter;

//type Queen = (usize, usize); // TODO: made struct, and implement From trait
#[derive(Clone, Copy)]
struct Queen(usize, usize); // TODO: rename to rect_queens

impl From<(usize, DiagonalQueen)> for Queen {
    fn from((size, diag_queen): (usize, DiagonalQueen)) -> Self {
        Self(
            (diag_queen.0 + diag_queen.1 + 1 - size) / 2,
            (size - 1 + diag_queen.0 - diag_queen.1) / 2,
        )
    }
}

impl From<(usize, usize)> for Queen {
    fn from(tpl: (usize, usize)) -> Self {
        Self(tpl.0, tpl.1)
    }
}

struct Chessboard(Vec<Queen>, usize); // TODO: prlly turn into slice with lifetime specifier

#[derive(Clone, Copy, Debug, PartialEq)]
struct DiagonalQueen(usize, usize);

impl From<(usize, Queen)> for DiagonalQueen {
    fn from((size, queen): (usize, Queen)) -> Self {
        // TODO: replace size - 1 with simple size
        Self(queen.0 + queen.1, size - 1 + queen.0 - queen.1)
    }
}

// TODO: maybe use randomized algo
// TODO: implement display and debug for DiagonalChessboard
// TODO: display of initial DiagonalChessboard and in any moment of time
// TODO: main algo

//     |--------->(1)
//     |         |
//     |    x    |
//     |   / \   |
//     | 1/   \0 |
//  (0)V-V-----V-|
struct DiagonalChessboard {
    diag0_to1: Vec<Option<usize>>,
    diag1_to0: Vec<Option<usize>>,
    coincident_queens: VecDeque<DiagonalQueen>,
    mandatory_queen: DiagonalQueen,
    diag_size: usize,
    size: usize,
}

impl Debug for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let queens = self.get_queens_in_rectangular_coordinates();
        writeln!(f, "\nQueens left: {0}", self.coincident_queens.len())?;
        for (Queen(i, j), is_coincident, is_mandatory) in queens{
            let queen_letter = if is_coincident {"q"} else {"Q"};
            writeln!(
                f,
                "{}{}{}",
                " .".repeat(j),
                if is_mandatory {format!("({0})", queen_letter)} else {format!(" {0} ", queen_letter)},
                ". ".repeat(self.size - j - 1),
            )?;
        }
        writeln!(f, "Q - queen in place\nq - coincident queen\n")
    }
}

impl Display for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let queens = self.get_queens_in_rectangular_coordinates();
        for (Queen(i, j), _, _) in queens{
            writeln!(
                f,
                "{}{}{}",
                ".".repeat(j),
                "Q",
                ".".repeat(self.size - j - 1)
            )?;
        }
        writeln!(f)
    }
}

impl DiagonalChessboard {
    fn from_mandatory_queen<T: Into<Queen>>(n: usize, mandatory_queen: T) -> DiagonalChessboard {
        let mandatory_queen = mandatory_queen.into();
        let diag_size = 2 * n - 1;
        let mut initial_chessboard = DiagonalChessboard {
            diag0_to1: vec![Option::None; diag_size],
            diag1_to0: vec![Option::None; diag_size],
            coincident_queens: VecDeque::new(),
            mandatory_queen: (n, mandatory_queen).into(),
            diag_size,
            size: n,
        };

        let mut vacant_queens_0 = vec![true; n];
        let mut vacant_queens_1 = vec![false; n];

        // TODO: extract function fill king pattern
        let mut queen = mandatory_queen;
        loop {
            if initial_chessboard.try_add_queen((n, queen)).is_ok() {
                vacant_queens_0[queen.0] = false;

                queen.0 = if queen.0 >= 2 {
                    queen.0 - 2
                } else {
                    let max_0 = vacant_queens_0.iter().rposition(|&is_vacant| !is_vacant).unwrap_or(n - 1);
                    if (n - max_0) % 2 == 0 {
                        n - 1
                    } else {
                        n.saturating_sub(2)
                    }
                };
            } else {
                vacant_queens_1[queen.1] = true;
            }
            queen.1 = (queen.1 + 1) % n;

            if queen.1 == mandatory_queen.1 {
                break;
            }
        }

        filter_vacant(vacant_queens_0)
            .zip(filter_vacant(vacant_queens_1))
            .for_each(|(diag_0, diag_1)| {
                initial_chessboard.add_queen((n, Queen(diag_0, diag_1)));
            });

        fn filter_vacant(vacant_queens: Vec<bool>) -> impl Iterator<Item = usize> {
            vacant_queens
                .into_iter()
                .enumerate()//TODO: use iter.positions()
                .filter_map(|(index, is_vacant)| if is_vacant { Some(index) } else { None })
        }

        dbg!(&initial_chessboard);
        initial_chessboard
    }

    fn add_queen<T: Into<DiagonalQueen>>(&mut self, queen: T){
        if let Err(diag_queen) = self.try_add_queen(queen) {
            self.add_coincident_queen(diag_queen);
        }
    }

    fn try_add_queen<T: Into<DiagonalQueen>>(
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

    fn get_queens_in_rectangular_coordinates(&self) -> impl Iterator<Item = (Queen, bool, bool)> + '_ {
        self.diag0_to1
            .iter()
            .enumerate()//TODO: use iter.positions()
            .filter_map(
                |(index, opt)| {
                    // TODO: to explore option map https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
                    opt.as_ref().map(|diag_1| (DiagonalQueen(index, *diag_1), false))
                }
            )
            .chain(
                self.coincident_queens
                    .iter()
                    .map(|diag_queen| (*diag_queen, true)),
            )
            .map(|(diag_queen, is_coincident)| {
                let is_mandatory = diag_queen == self.mandatory_queen;
                ((self.size, diag_queen).into(), is_coincident, is_mandatory)
            })
            .sorted_by_key(|(Queen(i, j), _, _)| *i)
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
    let chessboard = DiagonalChessboard::from_mandatory_queen(n, mandatory_queen);
    todo!("Create queens");
}

//....
//.Q..
//....
//Q...

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{Chessboard, DiagonalChessboard, Queen};

    #[test]
    fn format_test() {
        let queens = vec![Queen(0, 0), Queen(1, 1), Queen(2, 2), Queen(3, 3)];
        println!("{}", Chessboard(queens, 4));
    }

    #[test]
    fn initial_chessboard_test() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0))];
        for (n, mandatory_queen) in basic_tests {
            DiagonalChessboard::from_mandatory_queen(n, mandatory_queen);
        }
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
