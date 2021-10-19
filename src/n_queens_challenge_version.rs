// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::ops::Add;
use std::time::Instant;
use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Queen{
    rect: (usize, usize),
    diag: (usize, usize),
}

//NOTE: before solving a problem think about which data will be used aka context pattern!

impl Queen {
    fn from_rect(n: usize, rect: (usize, usize)) -> Self{
        Self{ rect, diag: (rect.0 + rect.1, n - 1 + rect.0 - rect.1) }
    }

    fn from_diag(n: usize, diag: (usize, usize)) -> Self{
        let rect = (
            (diag.0 + diag.1 + 1 - n) / 2,
            (n - 1 + diag.0 - diag.1) / 2,
        );
        Self{ rect, diag }
    }

    // TODO: maybe use complex
    fn try_add_rect(&self, n: usize, delta: (isize, isize)) -> Option<Self> {
        let in_bounds = |num: isize| 0 <= num && num < n as isize;
        let new_rect = (self.rect.0 as isize + delta.0, self.rect.1 as isize + delta.1);
        if in_bounds(new_rect.0) && in_bounds(new_rect.1) {
            Some(Self::from_rect(n, (new_rect.0 as usize, new_rect.1 as usize)))
        } else {
            None
        }
    }
}

// TODO: maybe use randomized algo

//     |--------->(1)
//     |         |
//     |    x    |
//     |   / \   |
//     | 1/   \0 |
//  (0)V-V-----V-|
struct DiagonalChessboard {
    // TODO: maybe use hashset?
    // TODO: use tuples here? diag.0 diag.1
    diag0: Vec<Option<Queen>>,
    diag1: Vec<Option<Queen>>,
    axis0: Vec<Option<Queen>>,
    axis1: Vec<Option<Queen>>,
    coincident_queens: VecDeque<Queen>,
    mandatory_queen: Queen,
    diag_size: usize,
    n: usize,
}

impl Debug for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut coincident = &self.coincident_queens.iter().collect::<HashSet<_>>();
        writeln!(f, "\nQueens left: {0}", self.coincident_queens.len())?;
        for queen in self.get_all_queens_sorted() {
            let queen_letter = if coincident.contains(&queen) {"q"} else {"Q"};
            let is_mandatory = self.mandatory_queen == queen;
            writeln!(
                f,
                "{}{}{}",
                " .".repeat(queen.rect.1),
                if is_mandatory {format!("({0})", queen_letter)} else {format!(" {0} ", queen_letter)},
                ". ".repeat(self.n - queen.rect.1 - 1),
            )?;
        }
        writeln!(f, "Q - queen in place\nq - coincident queen\n")
    }
}

impl Display for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for Queen{ rect, diag } in self.axis1.iter().flatten() {
            writeln!(
                f,
                "{}{}{}",
                ".".repeat(rect.0),
                "Q",
                ".".repeat(self.n - rect.0 - 1)
            )?;
        }
        writeln!(f)
    }
}

impl DiagonalChessboard {
    fn from_mandatory_queen(n: usize, mandatory_queen: Queen) -> DiagonalChessboard {
        let diag_size = 2 * n - 1;
        let mut initial_chessboard = DiagonalChessboard {
            diag0: vec![Option::None; diag_size],
            diag1: vec![Option::None; diag_size],
            axis0: vec![Option::None; n],
            axis1: vec![Option::None; n],
            coincident_queens: VecDeque::new(),
            mandatory_queen,
            diag_size,
            n,
        };

        // TODO: extract function fill knight pattern
        let mut queen_rect = mandatory_queen.rect;
        loop {
            let queen = Queen::from_rect(n, queen_rect);
            if !initial_chessboard.is_diag_coincident(&queen) {
                initial_chessboard.push_queen(queen);

                queen_rect.0 = if queen_rect.0 >= 2 {
                    queen_rect.0 - 2
                } else {
                    let max_0 = initial_chessboard.axis0
                        .iter()
                        .rposition(|opt| opt.is_some())
                        .unwrap_or(n - 1);
                    if (n - max_0) % 2 == 0 {
                        n - 1
                    } else {
                        n.saturating_sub(2)
                    }
                };
            }
            queen_rect.1 = (queen_rect.1 + 1) % n;

            if queen_rect.1 == mandatory_queen.rect.1 {
                break;
            }
        }

        let vacant_queens = filter_vacant(&initial_chessboard.axis0)
            .zip(filter_vacant(&initial_chessboard.axis1)).collect_vec();
        for (i, j) in vacant_queens {
            initial_chessboard.push_queen_or_coincident(Queen::from_rect(n, (i, j)));
        }

        fn filter_vacant(queens: &Vec<Option<Queen>>) -> impl Iterator<Item = usize> + '_ {
            queens
                .iter()
                .positions(|opt| opt.is_none())
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

        let rect_coincidence = self.axis0[queen.rect.0].take()
            .or(self.axis1[queen.rect.1].take());
        if let Some(coincident_queen) = rect_coincidence {
            if coincident_queen == self.mandatory_queen {
                return None;
            }

            self.axis0[coincident_queen.rect.0].take();
            self.axis1[coincident_queen.rect.1].take();
            self.diag0[coincident_queen.diag.0].take();
            self.diag1[coincident_queen.diag.1].take();
        }

        self.diag0[queen.diag.0].insert(queen);
        self.diag1[queen.diag.1].insert(queen);
        self.axis0[queen.rect.0].insert(queen);
        self.axis1[queen.rect.1].insert(queen);
        rect_coincidence
    }

    fn is_diag_coincident(&self, queen: &Queen) -> bool{
        self.diag0[queen.diag.0].is_some() || self.diag1[queen.diag.1].is_some()
    }

    fn get_all_queens_sorted(&self) -> impl Iterator<Item = Queen> + '_ {
        self.diag0
            .iter()
            .flatten()
            .chain(&self.coincident_queens)
            .sorted_by_key(|Queen{rect, diag}| rect.0)
            .cloned()
    }
}

pub fn solve_n_queens(n: usize, mandatory_queen: (usize, usize)) -> Option<String> {
    match n {
        1 => return Some("Q".to_string()),
        2 => return None,
        _ => (),
    }

    let start = Instant::now();
    let mut now = Instant::now();
    let mut counter: u64 = 0;

    let mut chessboard = DiagonalChessboard::from_mandatory_queen(n, Queen::from_rect(n, mandatory_queen));
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(queen) = chessboard.coincident_queens.pop_front() {
        counter += 1;
        'outer: for dir in dirs {
            for f in 1.. {
                let delta = (dir.0 * f, dir.1 * f);
                if let Some(queen) = queen.try_add_rect(n, delta) {
                    if let Some(replaced_queen) = chessboard.push_queen(queen) {
                        let new_queen = replaced_queen.try_add_rect(n, (-delta.0, -delta.1)).unwrap();
                        chessboard.push_queen_or_coincident(new_queen);
                        break 'outer;
                    }
                } else {
                    break
                }
            }
        }

        if now.elapsed().as_millis() >= 500 {
            dbg!(counter);
            dbg!(&chessboard);
            now = Instant::now();
        } else if start.elapsed().as_millis() >= 5000 {
            return None;
        }
    }
    // TODO: return None by timer and size lower than 8
    Some(format!("{0}", chessboard))
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{DiagonalChessboard, Queen};

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

    #[test]
    fn hard_test(){
        let (n, fixed) = (100, (10, 10));
        test_solution(n, fixed);
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
