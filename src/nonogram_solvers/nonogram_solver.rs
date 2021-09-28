use itertools::Itertools;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::iter::{FilterMap, Rev, Zip};
use trace::trace;
trace::init_depth_var!();

//https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
//TODO: try run Rust code from python test

type Bit = u8;
const FILLED: Bit = 1;
const EMPTY: Bit = 0;

#[derive(Debug)]
struct Clues<const T: usize>([FlatClues; T]);

impl<const T: usize> Clues<T> {
    fn from(from: [&[u8]; T]) -> Self {
        let slice_vec = from.iter().map(|&clues|
            FlatClues {
                stack: clues.iter()
                    .flat_map(|&clue| std::iter::repeat(1).take(clue.into()).chain([0]))
                    .collect_vec(),
                index: 0,
            }).collect_vec().try_into().unwrap();
        Clues(slice_vec)
    }

    fn can_be_applied(&self, permutation: &[Bit; T]) -> bool {
        permutation.iter().zip(&self.0)
            .all(|(permutation_tile, clues)| match clues.current() {
                Some(clue_current) => match (*clue_current, *permutation_tile) {
                    (FILLED, FILLED) | (EMPTY, EMPTY) => true,
                    (EMPTY, FILLED) => false,
                    (FILLED, EMPTY) => match clues.previous() {
                        Some(&prev_clue) => prev_clue == EMPTY,
                        None => true,
                    },
                    _ => unreachable!(),
                },
                None => *permutation_tile == 0,
            })
    }

    fn apply_permutation(&mut self, permutation: &[Bit; T]) -> Vec<bool> {
        self.0.iter_mut().zip(permutation).map(|(clues, permutation_tile)|
            match (clues.current(), *permutation_tile) {
                (Some(&EMPTY), EMPTY) | (_, FILLED) if clues.stack.len() > 1 => {
                    clues.index += 1;
                    true
                }
                _ => false,
            }).collect_vec()
    }

    fn undo_permutation(&mut self, altered_tiles: Vec<bool>) {
        for (clues, altered_tile) in self.0.iter_mut().zip(altered_tiles) {
            if altered_tile {
                clues.index -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct FlatClues {
    stack: Vec<Bit>,
    index: usize,
}

impl FlatClues {
    fn current(&self) -> Option<&Bit> {
        self.stack.get(self.index)
    }
    fn previous(&self) -> Option<&Bit> {
        self.stack.get(self.index.checked_sub(1)?)
    }
}

pub fn solve_nonogram<const T: usize>(
    (top_clues, left_clues): ([&[Bit]; T], [&[Bit]; T]),
) -> [[Bit; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<[u8; T]> = Vec::with_capacity(T);

    let is_solved = solve_nongram_rec(
        &mut processed_top_clues,
        &left_clues,
        &mut permutations_stack);

    if is_solved {
        return permutations_stack.try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    fn solve_nongram_rec<const T: usize>(
        top_clues: &mut Clues<T>,
        left_clues: &[&[u8]; T],
        permutations_stack: &mut Vec<[u8; T]>,
    ) -> bool {
        let current_clues_index = permutations_stack.len();
        for permutation in get_permutations::<T>(left_clues[current_clues_index]) {
            if !top_clues.can_be_applied(&permutation) {
                continue;
            }

            let altered_tiles = top_clues.apply_permutation(&permutation);
            permutations_stack.push(*permutation);
            if permutations_stack.len() == T
                || solve_nongram_rec(top_clues, left_clues, permutations_stack)
            {
                return true;
            }
            top_clues.undo_permutation(altered_tiles);
            permutations_stack.pop();
        }
        false
    }
}

fn get_permutations<const N: usize>(clues: &[u8]) -> Box<dyn Iterator<Item = Box<[u8; N]>> + '_> {
    return get_permutations_rec(clues, 0);

    fn get_permutations_rec<const N: usize>(
        clues: &[u8],
        init_offset: usize,
    ) -> Box<dyn Iterator<Item = Box<[u8; N]>> + '_> {
        if clues.is_empty() {
            return Box::new(std::iter::once(Box::new([0; N])));
        }

        let current_clue = *clues.first().unwrap() as usize;
        let clues_sum = clues.iter().sum::<u8>() as usize;
        let clues_borders = clues.len() - 1;

        Box::new(
            (0..=N.saturating_sub(init_offset + clues_sum + clues_borders)).flat_map(move |offset| {
                let offset = init_offset + offset;
                get_permutations_rec(&clues[1..], 1 + offset + current_clue).map(move |mut slice| {
                    slice[offset..current_clue + offset].fill(1);
                    slice
                })
            }),
        )
    }
}

#[cfg(test)]
pub mod basic_tests {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    #[test]
    fn transpose_test() {
        let vec_vec = transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
            .map(|iter| iter.collect_vec())
            .collect_vec();
        println!("{:?}", vec_vec);
    }

    #[test]
    fn get_permutations_15_test() {
        let permutations = get_permutations::<15>(&[1, 2, 3, 1])
            .map(|perm| perm.to_vec())
            .collect_vec();
        assert_eq!(
            permutations.first().unwrap(),
            &vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            permutations.last().unwrap(),
            &vec![0u8, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1]
        );
        // itertools::assert_equal(permutations.first().unwrap(), vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0].iter());
    }

    #[test]
    fn get_permutations_5_test() {
        let permutations = get_permutations::<5>(&[2, 2])
            .map(|perm| perm.to_vec())
            .collect_vec();

        print(permutations);
    }

    #[test]
    fn get_permutations_single_clue_test() {
        let permutations = get_permutations::<15>(&[1])
            .map(|perm| perm.to_vec())
            .collect_vec();
        assert_eq!(
            permutations.first().unwrap(),
            &vec![1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            permutations.last().unwrap(),
            &vec![0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test1() {
        assert_eq!(solve_nonogram(CLUES_1), ANS_1);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_nonogram(CLUES_2), ANS_2);
    }

    #[test]
    fn test3() {
        assert_eq!(solve_nonogram(CLUES_3), ANS_3);
    }

    #[test]
    fn test15() {
        assert_eq!(solve_nonogram(CLUES_15), ANS_15);
    }

    #[bench]
    #[ignore]
    fn bench15(bencher: &mut Bencher) {
        bencher.iter(|| solve_nonogram(CLUES_15));
    }

    #[test]
    fn test25() {
        print(solve_nonogram(CLUES_25));
    }

    #[test]
    #[ignore]
    fn test50() {
        const COUNT: usize = 50;
        let arr = (1..6).collect_vec();
        let arr = (1..6).rev().collect_vec();
        let keys = (0..COUNT).map(|_| &arr[..]).collect_vec();
        print(solve_nonogram::<COUNT>((keys.clone().try_into().unwrap(), keys.try_into().unwrap())));
    }

    const CLUES_1: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[4], &[1, 1, 1], &[3], &[1]],
        [&[1], &[2], &[3], &[2, 1], &[4]],
    );

    const ANS_1: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 0, 1, 0],
        [0, 1, 1, 1, 1],
    ];

    const CLUES_2: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1], &[3], &[1], &[3, 1], &[3, 1]],
        [&[3], &[2], &[2, 2], &[1], &[1, 2]],
    );

    const ANS_2: [[u8; 5]; 5] = [
        [0, 0, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 1, 1],
    ];

    const CLUES_3: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[1, 1], &[4], &[3], &[1, 1]],
        [&[3], &[1, 1], &[2], &[3], &[2, 1]],
    );

    const ANS_3: [[u8; 5]; 5] = [
        [1, 1, 1, 0, 0],
        [0, 0, 1, 0, 1],
        [0, 0, 1, 1, 0],
        [0, 0, 1, 1, 1],
        [1, 1, 0, 1, 0],
    ];
}

pub const CLUES_15: ([&[u8]; 15], [&[u8]; 15]) = (
    [
        &[4, 3],
        &[1, 6, 2],
        &[1, 2, 2, 1, 1],
        &[1, 2, 2, 1, 2],
        &[3, 2, 3],
        &[2, 1, 3],
        &[1, 1, 1],
        &[2, 1, 4, 1],
        &[1, 1, 1, 1, 2],
        &[1, 4, 2],
        &[1, 1, 2, 1],
        &[2, 7, 1],
        &[2, 1, 1, 2],
        &[1, 2, 1],
        &[3, 3],
    ],
    [
        &[3, 2],
        &[1, 1, 1, 1],
        &[1, 2, 1, 2],
        &[1, 2, 1, 1, 3],
        &[1, 1, 2, 1],
        &[2, 3, 1, 2],
        &[9, 3],
        &[2, 3],
        &[1, 2],
        &[1, 1, 1, 1],
        &[1, 4, 1],
        &[1, 2, 2, 2],
        &[1, 1, 1, 1, 1, 1, 2],
        &[2, 1, 1, 2, 1, 1],
        &[3, 4, 3, 1],
    ],
);

pub const ANS_15: [[u8; 15]; 15] = [
    [0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
    [1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0],
    [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1],
    [0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
];

pub const CLUES_25: ([&[u8]; 25], [&[u8]; 25]) = (
    [
        &[1, 1, 4],
        &[2, 1, 1, 2],
        &[2, 1, 4, 3, 2],
        &[11, 7],
        &[7, 1, 6],
        &[3, 4, 7, 2],
        &[2, 2, 7, 1, 2],
        &[4, 3, 1, 2],
        &[7, 3, 1, 3],
        &[6, 1, 3],
        &[3, 2, 3],
        &[1, 4, 2],
        &[1, 1, 8, 1],
        &[3, 9, 3],
        &[3, 8, 1, 5],
        &[3, 4, 2, 3, 4],
        &[10, 3, 1, 1, 5],
        &[3, 4, 3, 3],
        &[8, 1, 1, 8],
        &[6, 9],
        &[7, 1, 9],
        &[3, 3, 9],
        &[1, 2, 6],
        &[1, 5],
        &[1, 4, 1],
    ],
    [
        &[5, 3],
        &[5, 5],
        &[3, 7, 1],
        &[3, 2, 2, 6],
        &[2, 2, 2, 6],
        &[1, 3, 3, 1, 3],
        &[8, 3, 3],
        &[1, 7, 3, 3, 2],
        &[3, 1, 3, 6, 1],
        &[2, 2, 7, 3],
        &[3, 3, 1, 2],
        &[1, 3, 3, 2],
        &[1, 7],
        &[11, 1],
        &[3, 3, 3],
        &[1, 4, 3, 4, 1],
        &[1, 3, 10],
        &[1, 7, 1, 6, 1],
        &[7, 11],
        &[5, 1, 7],
        &[3, 13],
        &[4, 11],
        &[1, 1, 3, 12],
        &[3, 7, 2],
        &[1, 7, 1],
    ],
);

fn print<T: Debug>(field: impl IntoIterator<Item = impl IntoIterator<Item = T>>) {
    println!(
        "{}",
        field
            .into_iter()
            .map(|collection| format!("{:?}", collection.into_iter().collect_vec()))
            .join("\n")
    );
}

fn transpose<T: Clone>(
    matrix: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    let mut iters = matrix
        .into_iter()
        .map(|iter| iter.into_iter())
        .collect_vec(); // TODO: inte_iter() type is asent. Bug

    (0..iters.len()).map(move |_| {
        iters
            .iter_mut()
            .filter_map(|iter| iter.next())
            .collect_vec()
            .into_iter()
    })
}
