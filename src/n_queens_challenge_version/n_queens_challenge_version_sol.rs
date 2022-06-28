// https://www.codewars.com/kata/5985ea20695be6079e000003/train/rust
use itertools::Itertools;
use num::{Complex, Integer, ToPrimitive};
use rand::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Step;
use std::ops::{Add, Range};
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub(super) struct Queen {
    rect: Complex<i16>,
    diag: Complex<i16>,
}

impl Queen {
    fn from_rect_tpl(n: usize, rect: (usize, usize)) -> Self {
        Queen::from_rect(n as i16, Complex::new(rect.0 as i16, rect.1 as i16))
    }

    fn from_rect(n: i16, rect: Complex<i16>) -> Self {
        // TODO: use better formula for coordinates replacement
        Self {
            rect,
            diag: Complex::new(rect.re + rect.im, n - 1 + rect.re - rect.im),
        }
    }

    fn from_diag(n: i16, diag: Complex<i16>) -> Self {
        let rect = Complex::new(
            (diag.re + diag.im + 1 - n) / 2,
            (n - 1 + diag.re - diag.im) / 2,
        );
        Self { rect, diag }
    }
}

//     |--------->(1)
//     |         |
//     |    x    |
//     |   / \   |
//     | 1/   \0 |
//  (0)V-V-----V-|
pub(super) struct DiagonalChessboard {
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
            let queen_letter = if coincident.contains(&queen) {
                "q"
            } else {
                "Q"
            };
            let is_mandatory = self.mandatory_queen == queen;
            writeln!(
                f,
                "{}{}{}",
                " .".repeat(queen.rect.im as usize),
                if is_mandatory {
                    format!("({0})", queen_letter)
                } else {
                    format!(" {0} ", queen_letter)
                },
                ". ".repeat((self.n - queen.rect.im - 1) as usize),
            )?;
        }
        writeln!(f, "Q - queen in place\nq - coincident queen\n")
    }
}

impl Display for DiagonalChessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for Queen { rect, diag } in self.axis1.iter().flatten() {
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
    pub(super) fn from_mandatory_queen(
        n: usize,
        mandatory_queen: (usize, usize),
    ) -> DiagonalChessboard {
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
                    let max_0 = initial_chessboard
                        .axis0
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
            .zip(filter_vacant(&initial_chessboard.axis1))
            .collect_vec();
        for (i, j) in vacant_queens {
            initial_chessboard.push_queen_or_coincident(Queen::from_rect_tpl(n as usize, (i, j)));
        }

        fn filter_vacant(queens: &[Option<Queen>]) -> impl Iterator<Item = usize> + '_ {
            queens.iter().positions(|opt| opt.is_none())
        }

        dbg!(&initial_chessboard);
        initial_chessboard
    }

    fn push_queen_or_coincident(&mut self, queen: Queen) {
        if self.is_diag_coincident(&queen) {
            self.coincident_queens.push_back(queen)
        } else {
            self.push_queen(queen);
        }
    }

    #[allow(clippy::or_fun_call, unused_must_use)]
    fn try_push_queen(&mut self, rnd_queen: Queen) -> Option<Queen> {
        if self.is_diag_coincident(&rnd_queen) {
            return None;
        }

        let rect_coincidence =
            self.axis0[rnd_queen.rect.re as usize].or(self.axis1[rnd_queen.rect.im as usize]);
        match rect_coincidence {
            Some(coincident_queen) => {
                if coincident_queen == self.mandatory_queen {
                    return None;
                }

                self.remove_queen(coincident_queen);
                self.push_queen(rnd_queen);
                rect_coincidence
            }
            None => None,
        }
    }

    fn remove_queen(&mut self, queen: Queen) {
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
            .sorted_by_key(|Queen { rect, diag }| rect.re)
            .cloned()
    }
}

pub(super) fn solve_n_queens(n: usize, mandatory_queen: (usize, usize)) -> Option<String> {
    match n {
        1 => return Some("Q\n".to_string()),
        2 => return None,
        _ => (),
    }

    let start = Instant::now();

    let mut chessboard = DiagonalChessboard::from_mandatory_queen(n, mandatory_queen);
    let n = n as i16;
    while let Some(queen) = chessboard.coincident_queens.pop_front() {
        let is_replaced = unique_random(0i16..n).any(|rnd_rect| {
            let rnd_im_queen = Queen::from_rect(n, Complex::new(queen.rect.re, rnd_rect));
            let rnd_re_queen = Queen::from_rect(n, Complex::new(rnd_rect, queen.rect.im));

            let mut try_rotate_queen = |rnd_queen: Queen| match chessboard.try_push_queen(rnd_queen)
            {
                None => false,
                Some(replaced_queen) => {
                    let delta_rect = queen.rect - rnd_queen.rect;
                    let new_replaced_queen = Queen::from_rect(n, replaced_queen.rect + delta_rect);
                    chessboard.push_queen_or_coincident(new_replaced_queen);
                    true
                }
            };

            (rnd_im_queen.rect - queen.rect).l1_norm() > 1 && try_rotate_queen(rnd_im_queen)
                || (rnd_re_queen.rect - queen.rect).l1_norm() > 1 && try_rotate_queen(rnd_re_queen)
        });

        if !is_replaced {
            if chessboard.coincident_queens.is_empty() {
                let first_onboard = chessboard
                    .get_all_queens_sorted()
                    .find(|q| *q != chessboard.mandatory_queen && *q != queen)
                    .unwrap();

                chessboard.remove_queen(first_onboard);
                chessboard.coincident_queens.push_back(first_onboard);
            }
            chessboard.push_queen_or_coincident(queen);
        }

        // NOTE: boards that cannot be solved
        if start.elapsed().as_millis() >= 10 && n <= 10 {
            dbg!(&chessboard);
            return None;
        }
    }

    dbg!(&chessboard);
    Some(format!("{0}", chessboard))
}

fn unique_random<T: Integer + Step>(range: Range<T>) -> impl Iterator<Item = T> {
    let mut rng = thread_rng();
    range
        .map(|index| (rng.gen::<usize>(), index))
        .sorted()
        .map(|(rng, index)| index)
}
