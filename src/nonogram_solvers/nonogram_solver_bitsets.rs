use itertools::Itertools;
use std::iter;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use bit_set::BitSet;
use bit_vec::BitVec;
//use trace::trace;
//trace::init_depth_var!();

//https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
//TODO: try run Rust code from python test

type Bit = bool;
const FILLED: Bit = true;
const EMPTY: Bit = false;

#[derive(Debug)]
struct Clues<const T: usize>([FlatClues; T]);

impl<const T: usize> From<[&[u8]; T]> for Clues<T> {
    fn from(from: [&[u8]; T]) -> Self {
        let slice_vec = from.iter().map(|&clues|
            FlatClues {
                stack: clues.iter()
                    .flat_map(|&clue| std::iter::repeat(true).take(clue.into()).chain([false]))
                    .collect(),
                index: 0,
            }).collect_vec().try_into().unwrap();
        Clues(slice_vec)
    }
}

impl<const T: usize> Clues<T> {
    fn get_next_mandatory_bits(&self) -> BitVec {
        self.0.iter().map(|clues| {
            matches!((clues.current_bit(), clues.next_bit()), (Some(FILLED), Some(FILLED)))
        }).collect()
    }

    fn get_next_banned_bits(&self) -> BitVec {
        self.0.iter().map(|clues| {
            matches!(clues.next_bit(), Some(EMPTY) | None)
        }).collect()
    }

    fn apply_permutation(&mut self, permutation: &BitVec) -> BitVec {
        self.0.iter_mut().zip(permutation).map(|(clues, permutation_bit)|
            match (clues.next_bit(), permutation_bit) {
                (Some(EMPTY), EMPTY) | (_, FILLED) if clues.stack.len() > 1 => {
                    clues.index += 1;
                    true
                }
                _ => false,
            }).collect()
    }

    fn undo_permutation(&mut self, altered_bits: BitVec) {
        for (clues, altered_bit) in self.0.iter_mut().zip(altered_bits) {
            if altered_bit {
                clues.index -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct FlatClues {
    stack: BitVec,
    index: usize,
}

impl FlatClues {
    fn next_bit(&self) -> Option<bool> {
        self.stack.get(self.index)
    }
    fn current_bit(&self) -> Option<bool> {
        self.stack.get(self.index.checked_sub(1)?)
    }
}

pub fn solve_nonogram<const T: usize>(
    (top_clues, left_clues): ([&[u8]; T], [&[u8]; T]),
) -> [[u8; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<BitVec> = Vec::with_capacity(T);

    let is_solved = solve_nongram_rec::<T>(
        &mut processed_top_clues,
        &left_clues,
        &mut permutations_stack);

    if is_solved {
        return permutations_stack.iter().map(|bit_vec| {
            bit_vec.iter().map(|bit| bit as u8).collect_vec().try_into().unwrap()
        }).collect_vec().try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    fn solve_nongram_rec<const T: usize>(
        top_clues: &mut Clues<T>,
        left_clues: &[&[u8]; T],
        permutations_stack: &mut Vec<BitVec>,
    ) -> bool {
        let current_permutation_index = permutations_stack.len();
        let next_mandatory_bits = BitSet::from_bit_vec(top_clues.get_next_mandatory_bits());
        let next_banned_bits = BitSet::from_bit_vec(top_clues.get_next_banned_bits());

//        dbg!(&next_mandatory_bits);
//        dbg!(&next_banned_bits);
//        dbg!(&permutations_stack);

        for permutation in get_permutations::<T>(left_clues[current_permutation_index], next_mandatory_bits, next_banned_bits) {
            let altered_bits = top_clues.apply_permutation(&permutation);
            permutations_stack.push(permutation);

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

pub(super) fn get_permutations<'a, const T: usize >(clues: &'a [u8], next_mandatory_bits: BitSet, next_banned_bits: BitSet) -> Box<dyn Iterator<Item=BitVec> + 'a> {
    let permutation = BitSet::with_capacity(T);
    return get_permutations_rec::<T>(permutation, clues, 0, next_mandatory_bits, next_banned_bits);

    fn get_permutations_rec<'a, const T: usize>(
        permutation: BitSet,
        clues: &'a [u8],
        init_offset: usize,
        next_mandatory_bits: BitSet,
        next_banned_bits: BitSet,
    ) -> Box<dyn Iterator<Item = BitVec> + 'a> {
        let current_clue = match clues.first() {
            None => return Box::new(iter::once(permutation.into_bit_vec())),
            Some(&clue) => clue as usize,
        };
        let clues_sum = clues.iter().sum::<u8>() as usize;
        let clues_borders = clues.len() - 1;

        let iter = (0..T + 1 - (init_offset + clues_sum + clues_borders)).filter_map(move |offset| {
            let offset = init_offset + offset;
            let ones_range = offset..current_clue + offset;
            let mut permutation = permutation.clone();
            for index in ones_range {
                permutation.insert(index);
            }

            let has_zeroes_valid = next_banned_bits
                .is_disjoint(&permutation);
            let has_ones_valid = next_mandatory_bits
                .difference(&permutation)
                .take_while(|index| *index < current_clue + offset + 1)
                .count() == 0;

            if has_ones_valid && has_zeroes_valid{
                Some(get_permutations_rec::<T>(
                    permutation, &clues[1..], 1 + offset + current_clue, next_mandatory_bits.clone(), next_banned_bits.clone()))
            } else {
                None
            }
        }).flatten();
        Box::new(iter)
    }
}