// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::iter;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Queen{
    rect: (usize, usize),
    diag: (usize, usize),
}

//NOTE: before solving a problem think about which data will be used aka context pattern!

impl Queen {
    fn from_rect(n: usize, rect: (usize, usize)) -> Self{
        let diag = (rect.0 + rect.1, n - 1 + rect.0 - rect.1);
        Queen{ rect, diag }
    }

    fn from_diag(n: usize, diag: (usize, usize)) -> Self{
        let rect = (
            (diag.0 + diag.1 + 1 - n) / 2,
            (n - 1 + diag.0 - diag.1) / 2,
        );
        Queen{ rect, diag }
    }
    // TODO: create try move fund
//    fn try_move_rect(&mut self, n: usize, delta: (usize, usize)) -> bool{
//
//    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct RectQueen(usize, usize); // TODO: rename to rect_queens

impl From<(usize, DiagQueen)> for RectQueen {
    fn from((size, diag_queen): (usize, DiagQueen)) -> Self {
        Self(
            (diag_queen.0 + diag_queen.1 + 1 - size) / 2,
            (size - 1 + diag_queen.0 - diag_queen.1) / 2,
        )
    }
}

impl From<(usize, usize)> for RectQueen {
    fn from(tpl: (usize, usize)) -> Self {
        Self(tpl.0, tpl.1)
    }
}

struct Chessboard(Vec<RectQueen>, usize); // TODO: prlly turn into slice with lifetime specifier

#[derive(Clone, Copy, Debug, PartialEq)]
struct DiagQueen(usize, usize);// TODO: beter has one queen with all kinds of coordinates, and methods from_rect and from_diag

impl From<(usize, RectQueen)> for DiagQueen {
    fn from((size, queen): (usize, RectQueen)) -> Self {
        // TODO: replace size - 1 with simple size
        Self(queen.0 + queen.1, size - 1 + queen.0 - queen.1)
    }
}

// TODO: maybe use randomized algo

//     |--------->(1)
//     |         |
//     |    x    |
//     |   / \   |
//     | 1/   \0 |
//  (0)V-V-----V-|
struct DiagonalChessboard { // TODO: vec option? maybe use hashset?
    diag0_to_rect: Vec<Option<Queen>>,// TODO: diag to horizontal queen
    diag1_to_rect: Vec<Option<Queen>>,// TODO: horizontal queen to diag
    axis0_to_diag: Vec<Option<Queen>>,
    axis1_to_diag: Vec<Option<Queen>>,
    coincident_queens: VecDeque<Queen>,
    mandatory_queen: Queen,
    diag_size: usize,
    n: usize,
}

impl Debug for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let queens = self.get_queens_in_rectangular_coordinates();
        writeln!(f, "\nQueens left: {0}", self.coincident_queens.len())?;
        for (Queen{rect, diag}, is_coincident, is_mandatory) in queens{
            let queen_letter = if is_coincident {"q"} else {"Q"};
            writeln!(
                f,
                "{}{}{}",
                " .".repeat(rect.1),
                if is_mandatory {format!("({0})", queen_letter)} else {format!(" {0} ", queen_letter)},
                ". ".repeat(self.n - rect.1 - 1),
            )?;
        }
        writeln!(f, "Q - queen in place\nq - coincident queen\n")
    }
}

impl Display for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let queens = self.get_queens_in_rectangular_coordinates();
        for (Queen{rect, diag}, _, _) in queens{
            writeln!(
                f,
                "{}{}{}",
                ".".repeat(rect.1),
                "Q",
                ".".repeat(self.n - rect.1 - 1)
            )?;
        }
        writeln!(f)
    }
}

impl DiagonalChessboard {
    fn from_mandatory_queen(n: usize, mandatory_queen: Queen) -> DiagonalChessboard {
        let diag_size = 2 * n - 1;
        let mut initial_chessboard = DiagonalChessboard {
            diag0_to_rect: vec![Option::None; diag_size],
            diag1_to_rect: vec![Option::None; diag_size],
            axis0_to_diag: vec![Option::None; n],
            axis1_to_diag: vec![Option::None; n],
            coincident_queens: VecDeque::new(),
            mandatory_queen,
            diag_size,
            n,
        };

        // TODO: there is no point in vacant_queens anymore, use axis0_to_..
        let mut vacant_queens_0 = vec![true; n]; // TODO: use sets or bitsets
        let mut vacant_queens_1 = vec![false; n];

        // TODO: extract function fill knight pattern
        let mut queen_rect = mandatory_queen.rect;
        loop {
            let queen = Queen::from_rect(n, queen_rect);
            if !initial_chessboard.is_diag_coincident(&queen) {
                initial_chessboard.push_queen(queen);
                vacant_queens_0[queen_rect.0] = false;

                queen_rect.0 = if queen_rect.0 >= 2 {
                    queen_rect.0 - 2
                } else {
                    let max_0 = vacant_queens_0.iter().rposition(|&is_vacant| !is_vacant).unwrap_or(n - 1);
                    if (n - max_0) % 2 == 0 {
                        n - 1
                    } else {
                        n.saturating_sub(2)
                    }
                };
            } else {
                vacant_queens_1[queen_rect.1] = true;
            }
            queen_rect.1 = (queen_rect.1 + 1) % n;

            if queen_rect.1 == mandatory_queen.rect.1 {
                break;
            }
        }

        filter_vacant(vacant_queens_0)
            .zip(filter_vacant(vacant_queens_1))
            .for_each(|(i, j)| {
                initial_chessboard.push_queen_or_coincident(Queen::from_rect(n, (i, j)));
            });

        fn filter_vacant(vacant_queens: Vec<bool>) -> impl Iterator<Item = usize> {
            vacant_queens
                .into_iter()
                .positions(|is_vacant| is_vacant)
        }

        dbg!(&initial_chessboard);
        initial_chessboard
    }

    fn push_queen_or_coincident(&mut self, queen: Queen){
        if self.is_diag_coincident(&queen) {
            self.coincident_queens.push_back(queen)
        } else {
            self.push_queen(queen);
        }
    }

    #[allow(clippy::or_fun_call, unused_must_use)]
    fn push_queen(&mut self, queen: Queen) -> Option<Queen>{
        if self.is_diag_coincident(&queen) {
            return None;
        }
        self.diag0_to_rect[queen.diag.0].insert(queen);
        self.diag1_to_rect[queen.diag.1].insert(queen);

        let rect_coincidence = self.axis0_to_diag[queen.rect.0].take()
            .or(self.axis1_to_diag[queen.rect.1].take());
        if let Some(coincident_queen) = rect_coincidence {
            self.axis0_to_diag[coincident_queen.rect.0].take()
                .or(self.axis1_to_diag[coincident_queen.rect.1].take())
        } else {
            None
        }
    }

    fn is_diag_coincident(&self, queen: &Queen) -> bool{
        self.diag0_to_rect[queen.diag.0].is_some() || self.diag1_to_rect[queen.diag.1].is_some()
    }

    fn get_queens_in_rectangular_coordinates(&self) -> impl Iterator<Item = (Queen, bool, bool)> + '_ {
        self.diag0_to_rect
            .iter()
            .enumerate()
            .filter_map(
                |(index, opt)| {
                    // TODO: to explore option map https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
                    opt.as_ref().map(|rect| (rect, false))
                }
            )
            .chain(
                self.coincident_queens
                    .iter()
                    .map(|queen| (queen, true))
            )
            .map(|(queen, is_coincident)| {
                let is_mandatory = self.mandatory_queen == *queen;
                (*queen, is_coincident, is_mandatory)
            })
            .sorted_by_key(|(Queen{rect, diag}, _, _)| rect.0)
    }
}

impl Display for Chessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Chessboard(queens, size) = self;
        writeln!(f, "Chessboard is:")?;
        for RectQueen(x, y) in queens.iter().sorted_by_key(|RectQueen(x, y)| y) {
            writeln!(f, "{}{}{}", ".".repeat(*x), "Q", ".".repeat(*size - x - 1))?;
        }
        writeln!(f)
    }
}

pub fn solve_n_queens(n: usize, mandatory_queen: (usize, usize)) -> Option<String> {
    let mut chessboard = DiagonalChessboard::from_mandatory_queen(n, Queen::from_rect(n, mandatory_queen));
    while let Some(queen) = chessboard.coincident_queens.pop_front() {
        if let Some(queen_1) = chessboard.diag0_to_rect[queen.diag.0] {

        }
    }
    // TODO: return None by timer and size lower than 8
    Some(format!("{0}", chessboard))
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{Chessboard, DiagonalChessboard, Queen, RectQueen};

    #[test]
    fn format_test() {
        let queens = vec![RectQueen(0, 0), RectQueen(1, 1), RectQueen(2, 2), RectQueen(3, 3)];
        println!("{}", Chessboard(queens, 4));
    }

    #[test]
    fn initial_chessboard_test() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0))];
        for (n, mandatory_queen) in basic_tests {
            DiagonalChessboard::from_mandatory_queen(n, Queen::from_rect(n, mandatory_queen));
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
