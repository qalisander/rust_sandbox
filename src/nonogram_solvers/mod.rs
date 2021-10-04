use itertools::Itertools;
use std::iter;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use trace::trace;
trace::init_depth_var!();

//https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
//TODO: try run Rust code from python test

type Bit = u8;
const FILLED: Bit = 1;
const EMPTY: Bit = 0;

enum Bit1 {
    EMPTY = 0,
    FILLED = 1,
}

#[derive(Debug, Copy, Clone)]
enum Shift {
    Available,
    Mandatory,
    Banned,
}

#[derive(Debug)]
struct Clues<const T: usize>([FlatClues; T]);

impl<const T: usize> From<[&[u8]; T]> for Clues<T> {
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
}

// TODO: prrlly remove const T, store vec only and check len as invariant
// TODO: write some benchmarks with criterioon


impl<const T: usize> Clues<T> {
    fn get_next_possible_bits(&self) -> Box<[Shift; T]>{
        let slice = self.0.iter().map(|clues| {
            match clues.next() {
                Some(&FILLED) => {
                    match clues.current() {
                        None | Some(&EMPTY) => { Shift::Available} // TODO: add remaining capacity
                        Some(&FILLED) => { Shift::Mandatory },
                        _ => unreachable!() // TODO: use EMPTY FILLED enums
                    }},
                Some(&EMPTY) | None => Shift::Banned,
                _ => unreachable!(),
            }
        }).collect_vec().try_into().unwrap();
        Box::new(slice)
    }

    fn apply_permutation(&mut self, permutation: &[Bit; T]) -> Vec<bool> {
        for i in 0..T {
            let (clues, permutation_bit) = (&mut self.0[i], permutation[i]);

        }

        self.0.iter_mut().zip(permutation).map(|(clues, permutation_bit)|
            match (clues.next(), *permutation_bit) {
                (Some(&EMPTY), EMPTY) | (_, FILLED) if clues.stack.len() > 1 => {
                    clues.index += 1;
                    true
                }
                _ => false,
            }).collect_vec()
    }

    fn undo_permutation(&mut self, altered_bits: Vec<bool>) {
        for (clues, altered_bit) in self.0.iter_mut().zip(altered_bits) {
            if altered_bit {
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
    //TODO: rename to next possible
    fn next(&self) -> Option<&Bit> {
        self.stack.get(self.index)
    }
    fn current(&self) -> Option<&Bit> {
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
        let current_permutation_index = permutations_stack.len();
        let next_possible_bits = top_clues.get_next_possible_bits();

        for permutation in get_permutations::<T>(&next_possible_bits,left_clues[current_permutation_index]) {
            let altered_bits = top_clues.apply_permutation(&permutation);
            permutations_stack.push(*permutation);

            if permutations_stack.len() == T
                || solve_nongram_rec(top_clues, left_clues, permutations_stack)
            {
                return true;
            }
            top_clues.undo_permutation(altered_bits);
            permutations_stack.pop();
        }
        false
    }
}

fn get_permutations<'a, const T: usize >(next_possible_bits: &'a [Shift; T], clues: &'a [u8]) -> Box<dyn Iterator<Item=Box<[u8; T]>> + 'a> {
    let permutation = Box::new([0u8; T]);
    return get_permutations_rec(permutation, next_possible_bits, clues, 0);

    fn get_permutations_rec<'a, const T: usize>(
        permutation: Box<[u8; T]>,
        next_possible_bits: &'a [Shift; T],
        clues: &'a [u8],
        init_offset: usize,
    ) -> Box<dyn Iterator<Item = Box<[u8; T]>> + 'a> { // TODO: return bitsets not array https://docs.rs/bit-set/0.5.0/bit_set/struct.BitSet.html
        let current_clue = match clues.first() {
            None => return Box::new(iter::once(permutation)),
            Some(&clue) => clue as usize,
        };
        let clues_sum = clues.iter().sum::<u8>() as usize;
        let clues_borders = clues.len() - 1;

        let iter = (0..T + 1 - (init_offset + clues_sum + clues_borders)).filter_map(move |offset| {
            let offset = init_offset + offset;

            // TODO: check ranges in pattern, generic true and false
            let zeroes_range = init_offset.saturating_sub(1)..offset; // TODO: predict with one at the end
            let has_zeroes_valid = next_possible_bits[zeroes_range].iter().all(|shift|{
                if let Shift::Available | Shift::Banned = shift { true } else { false }
            });

            let ones_range = offset..current_clue + offset;
            let has_ones_valid = next_possible_bits[ones_range.clone()].iter().all(|shift| {
                if let Shift::Available | Shift::Mandatory = shift { true } else { false }
            });

            if has_ones_valid && has_zeroes_valid {
                let mut permutation = permutation.clone();
                permutation[ones_range].fill(1);
                Some(get_permutations_rec(permutation.clone(), next_possible_bits, &clues[1..], 1 + offset + current_clue))
            } else {
                None
            }
        }).flatten();
        Box::new(iter)
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
        let permutations = get_permutations::<15>(&[Shift::Available; 15], &[1, 2, 3, 1])
            .map(|perm| perm.to_vec())
            .collect_vec();
//        print(&permutations);
        assert_eq!(
            permutations.first().unwrap(),
            &[1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
            "first permutation failed"
        );
        assert_eq!(
            permutations.last().unwrap(),
            &[0u8, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
            "last permutation failed"
        );
        // itertools::assert_equal(permutations.first().unwrap(), vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0].iter());
    }

    #[test]
    fn get_permutations_5_test() {
        let permutations = get_permutations::<5>(&[Shift::Available; 5], &[2, 2])
            .map(|perm| perm.to_vec())
            .collect_vec();

        assert_eq!(permutations.first().unwrap(), &[1u8, 1, 0, 1, 1]);
    }

    #[test]
    fn get_permutations_single_clue_test() {
        let permutations = get_permutations::<15>(&[Shift::Available; 15], &[1])
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
        const COUNT: usize = 35;
        let arr_top = (1..6).collect_vec();
        let arr_left = (1..6).rev().collect_vec();
        let keys_top = (0..COUNT).map(|_| &arr_top[..]).collect_vec();
        let keys_left = (0..COUNT).map(|_| &arr_left[..]).collect_vec();
        print(solve_nonogram::<COUNT>((keys_top.try_into().unwrap(), keys_left.try_into().unwrap())));
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

