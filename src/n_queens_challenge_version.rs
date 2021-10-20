// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Step;
use std::ops::{Add, Range};
use std::time::Instant;
use num::{Complex, ToPrimitive};
use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Queen{
    rect: Complex<i16>,
    diag: Complex<i16>,
}

impl Queen {
    fn from_rect_tpl(n: usize, rect: (usize, usize)) -> Self{
        Queen::from_rect(n as i16, Complex::new(rect.0 as i16, rect.1 as i16))
    }

    fn from_rect(n: i16, rect: Complex<i16>) -> Self{
        Self{ rect, diag: Complex::new(rect.re + rect.im, n - 1 + rect.re - rect.im) } // TODO: use better formula for coordinates replacement
    }

    fn from_diag(n: i16, diag: Complex<i16>) -> Self{
        let rect = Complex::new(
            (diag.re + diag.im + 1 - n) / 2,
            (n - 1 + diag.re - diag.im) / 2,
        );
        Self{ rect, diag }
    }
}

//     |--------->(1)
//     |         |
//     |    x    |
//     |   / \   |
//     | 1/   \0 |
//  (0)V-V-----V-|
struct DiagonalChessboard {
    // TODO: maybe use hashset?
    // TODO: use tuples here? diag.re diag.im
    diag0: Vec<Option<Queen>>,
    diag1: Vec<Option<Queen>>,
    axis0: Vec<Option<Queen>>,
    axis1: Vec<Option<Queen>>,
    coincident_queens: VecDeque<Queen>,
    mandatory_queen: Queen,
    diag_size: i16,
    n: i16,
}

impl Debug for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coincident = self.coincident_queens.iter().collect::<HashSet<_>>();
        writeln!(f, "\nQueens left: {0}", self.coincident_queens.len())?;
        for queen in self.get_all_queens_sorted() {
            let queen_letter = if coincident.contains(&queen) {"q"} else {"Q"};
            let is_mandatory = self.mandatory_queen == queen;
            writeln!(
                f,
                "{}{}{}",
                " .".repeat(queen.rect.im as usize),
                if is_mandatory {format!("({0})", queen_letter)} else {format!(" {0} ", queen_letter)},
                ". ".repeat((self.n - queen.rect.im - 1) as usize),
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
                ".".repeat(rect.re as usize),
                "Q",
                ".".repeat((self.n - rect.re - 1) as usize)
            )?;
        }
        writeln!(f)
    }
}

impl DiagonalChessboard {
    fn from_mandatory_queen(n: usize, mandatory_queen: (usize, usize)) -> DiagonalChessboard {
        let mandatory_queen = Queen::from_rect_tpl(n, mandatory_queen);
        let diag_size = 2 * n - 1;
        let mut initial_chessboard = DiagonalChessboard {
            diag0: vec![Option::None; diag_size],
            diag1: vec![Option::None; diag_size],
            axis0: vec![Option::None; n],
            axis1: vec![Option::None; n],
            coincident_queens: VecDeque::new(),
            mandatory_queen,
            diag_size: diag_size as i16,
            n: n as i16,
        };

        // TODO: extract function fill knight pattern
        let n = n as i16;
        let mut queen_rect = mandatory_queen.rect;
        loop {
            let queen = Queen::from_rect(n, queen_rect);
            if !initial_chessboard.is_diag_coincident(&queen) {
                initial_chessboard.push_queen(queen);

                queen_rect.re = if queen_rect.re >= 2 {
                    queen_rect.re - 2
                } else {
                    let max_0 = initial_chessboard.axis0
                        .iter()
                        .rposition(|opt| opt.is_some())
                        .unwrap_or(n as usize - 1) as i16;
                    if (n - max_0) % 2 == 0 {
                        n - 1
                    } else {
                        n.saturating_sub(2)
                    }
                };
            }

            queen_rect.im = (queen_rect.im + 1) % n;

            if queen_rect.im == mandatory_queen.rect.im {
                break;
            }
        }

        let vacant_queens = filter_vacant(&initial_chessboard.axis0)
            .zip(filter_vacant(&initial_chessboard.axis1)).collect_vec();
        for (i, j) in vacant_queens {
            initial_chessboard.push_queen_or_coincident(Queen::from_rect_tpl(n as usize, (i, j)));
        }

        fn filter_vacant(queens: &[Option<Queen>]) -> impl Iterator<Item = usize> + '_ {
            queens
                .iter()
                .positions(|opt| opt.is_none())
        }

//        dbg!(&initial_chessboard);
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
    fn try_push_queen(&mut self, rnd_queen: Queen) -> Option<Queen>{
        if self.is_diag_coincident(&rnd_queen){
            return None;
        }

        let rect_coincidence = self.axis0[rnd_queen.rect.re as usize].or(self.axis1[rnd_queen.rect.im as usize]);
        match rect_coincidence {
            Some(coincident_queen) => {
                if coincident_queen == self.mandatory_queen {
                    return None;
                }

                self.remove_queen(coincident_queen);
                self.push_queen(rnd_queen);
                rect_coincidence
            }
            None => None
        }
    }

    fn remove_queen(&mut self, queen: Queen){
        self.axis0[queen.rect.re as usize].take();
        self.axis1[queen.rect.im as usize].take();
        self.diag0[queen.diag.re as usize].take();
        self.diag1[queen.diag.im as usize].take();
    }

    fn push_queen(&mut self, queen: Queen) {
        self.diag0[queen.diag.re as usize].insert(queen);
        self.diag1[queen.diag.im as usize].insert(queen);
        self.axis0[queen.rect.re as usize].insert(queen);
        self.axis1[queen.rect.im as usize].insert(queen);
    }

    fn is_diag_coincident(&self, queen: &Queen) -> bool {
        self.diag0[queen.diag.re as usize].is_some() || self.diag1[queen.diag.im as usize].is_some()
    }

    fn get_all_queens_sorted(&self) -> impl Iterator<Item = Queen> + '_ {
        self.diag0
            .iter()
            .flatten()
            .chain(&self.coincident_queens)
            .sorted_by_key(|Queen{rect, diag}| rect.re)
            .cloned()
    }
}

pub fn solve_n_queens(n: usize, mandatory_queen: (usize, usize)) -> Option<String> {
    match n {
        1 => return Some("Q\n".to_string()),
        2 => return None,
        _ => (),
    }

    let start = Instant::now();

    let mut chessboard = DiagonalChessboard::from_mandatory_queen(n, mandatory_queen);
    let n = n as i16;
    while let Some(queen) = chessboard.coincident_queens.pop_front() {
        let is_replaced = unique_random(0i16..n).any(|rnd_rect|{
            let rnd_im_queen = Queen::from_rect(n, Complex::new(queen.rect.re, rnd_rect));
            let rnd_re_queen = Queen::from_rect(n, Complex::new(rnd_rect, queen.rect.im));

            let mut try_rotate_queen = |rnd_queen: Queen| {
                match chessboard.try_push_queen(rnd_queen) {
                    None => false,
                    Some(replaced_queen) => {
                        let delta_rect = queen.rect - rnd_queen.rect;
                        let new_replaced_queen = Queen::from_rect(n, replaced_queen.rect + delta_rect);
                        chessboard.push_queen_or_coincident(new_replaced_queen);
                        true
                    }
                }
            };

            (rnd_im_queen.rect - queen.rect).l1_norm() > 1 && try_rotate_queen(rnd_im_queen)
                || (rnd_re_queen.rect - queen.rect).l1_norm() > 1 && try_rotate_queen(rnd_re_queen)
        });

        if !is_replaced {
            chessboard.push_queen_or_coincident(queen);
        }

        // NOTE: boards that cannot be solved
        if start.elapsed().as_millis() >= 10 && n <= 10 {
            return None;
        }
    }

//    dbg!(&chessboard);
    Some(format!("{0}", chessboard))
}

fn unique_random<T: Step + Ord>(range: Range<T>) -> impl Iterator<Item=T> {
    let mut rng = thread_rng();
    range
        .map(|index| (rng.gen::<usize>(), index))
        .sorted()
        .map(|(rng, index)| index)
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;
    use crate::n_queens_challenge_version::{DiagonalChessboard, Queen};

    #[test]
    fn initial_chessboard_test() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0))];
        for (n, mandatory_queen) in basic_tests {
            DiagonalChessboard::from_mandatory_queen(n, mandatory_queen);
        }
    }

    #[test]
    fn basic_tests() {
        let basic_tests = vec![(8, (3, 0)), (4, (2, 0)), (1, (0, 0)), (8, (0, 5))];
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
    fn basic_test_15(){
        let (n, fixed) = (15, (10, 10));
        test_solution(n, fixed);
    }

    #[test]
    fn hard_test(){
        let basic_tests = vec![(500, (100, 50)), (1000, (10, 900)), (1500, (700, 555)), (5500, (700, 555))];
        for (n, fixed) in basic_tests.into_iter() {
            test_solution(n, fixed);
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
