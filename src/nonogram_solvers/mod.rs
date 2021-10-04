pub mod test;
use itertools::Itertools;
use std::iter;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use bit_vec::BitVec;
//use trace::trace;
//trace::init_depth_var!();

//https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
//TODO: try run Rust code from python test

type Bit = bool;
const FILLED: Bit = true;
const EMPTY: Bit = false;

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
                    .flat_map(|&clue| std::iter::repeat(true).take(clue.into()).chain([false]))
                    .collect(),
                index: 0,
            }).collect_vec().try_into().unwrap();
        Clues(slice_vec)
    }
}

// TODO: prrlly remove const T, store vec only and check len as invariant

impl<const T: usize> Clues<T> {
    fn get_next_possible_bits(&self) -> Box<[Shift; T]>{
        let slice = self.0.iter().map(|clues| {
            match clues.next() {
                Some(FILLED) => {
                    match clues.current() {
                        None | Some(EMPTY) => { Shift::Available} // TODO: add remaining capacity
                        Some(FILLED) => { Shift::Mandatory },
                        _ => unreachable!() // TODO: use EMPTY FILLED enums
                    }},
                Some(EMPTY) | None => Shift::Banned,
                _ => unreachable!(),
            }
        }).collect_vec().try_into().unwrap();
        Box::new(slice)
    }

    fn apply_permutation(&mut self, permutation: &BitVec) -> BitVec {
        self.0.iter_mut().zip(permutation).map(|(clues, permutation_bit)|
            match (clues.next(), permutation_bit) {
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
    fn next(&self) -> Option<bool> {
        self.stack.get(self.index)
    }
    fn current(&self) -> Option<bool> {
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
        let next_possible_bits = top_clues.get_next_possible_bits();

        for permutation in get_permutations(&next_possible_bits,left_clues[current_permutation_index]) {
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

fn get_permutations<'a, const T: usize >(next_possible_bits: &'a [Shift; T], clues: &'a [u8]) -> Box<dyn Iterator<Item=BitVec> + 'a> {
    let permutation = BitVec::from_elem(T, EMPTY);
    return get_permutations_rec(permutation, next_possible_bits, clues, 0);

    fn get_permutations_rec<'a, const T: usize>(
        permutation: BitVec,
        next_possible_bits: &'a [Shift; T], // TODO: use bitvec
        clues: &'a [u8],
        init_offset: usize,
    ) -> Box<dyn Iterator<Item = BitVec> + 'a> { // TODO: return bitsets not array https://docs.rs/bit-set/0.5.0/bit_set/struct.BitSet.html
        let current_clue = match clues.first() {
            None => return Box::new(iter::once(permutation)),
            Some(&clue) => clue as usize,
        };
        let clues_sum = clues.iter().sum::<u8>() as usize;
        let clues_borders = clues.len() - 1;

        let iter = (0..T + 1 - (init_offset + clues_sum + clues_borders)).filter_map(move |offset| {
            let offset = init_offset + offset;

            let zeroes_range = init_offset..offset; // TODO: predict with one at the end
            let has_zeroes_valid = next_possible_bits[zeroes_range].iter().all(|shift|{
                matches!(shift, Shift::Available | Shift::Banned)
            });

            let ones_range = offset..current_clue + offset;
            let has_ones_valid = next_possible_bits[ones_range.clone()].iter().all(|shift| {
                matches!(shift, Shift::Available | Shift::Mandatory)
            });

            let has_last_zero_valid = match next_possible_bits.get(current_clue + offset) {
                None => true,
                Some(&shift) => matches!(shift, Shift::Available | Shift::Banned)
            };

            if has_ones_valid && has_zeroes_valid && has_last_zero_valid{
                let mut permutation = permutation.clone();
                for index in ones_range {
                    permutation.set(index, true);
                }
                Some(get_permutations_rec(permutation, next_possible_bits, &clues[1..], 1 + offset + current_clue))
            } else {
                None
            }
        }).flatten();
        Box::new(iter)
    }
}

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

