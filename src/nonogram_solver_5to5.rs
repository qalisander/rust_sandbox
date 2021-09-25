use itertools::Itertools;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::iter::{FilterMap, Rev, Zip};
use trace::trace;
trace::init_depth_var!();

//TODO: try run Rust code from python test
type Tile = u8;
#[derive(Debug)]
struct Clues<const T: usize>([FlatClues; T]);

#[derive(Debug)]
struct FlatClues {
    stack: Vec<Tile>,
    index: usize,
}

impl FlatClues {
    fn current(&self) -> Option<&Tile> {
        self.stack.get(self.index)
    }
    fn previous(&self) -> Option<&Tile> {
        self.stack.get(self.index.checked_sub(1)?)
    }
}

impl<const T: usize> Clues<T> {
    fn from(from: [&[u8]; T]) -> Self {
        let slice_vec = from
            .iter()
            .map(|&clues| FlatClues {
                stack: clues
                    .iter()
                    .flat_map(|&clue| std::iter::repeat(1).take(clue.into()).chain([0]))
                    .collect_vec(),
                index: 0,
            })
            .collect_vec()
            .try_into()
            .unwrap();
        Clues(slice_vec)
    }

    // TODO: count rest keys
    fn can_be_used(&self, permutation: &[Tile; T]) -> bool {
        permutation
            .iter()
            .zip(&self.0)
            .all(|(permutation_tile, clues)| match clues.current() {
                Some(clue_tile) => match (clue_tile, permutation_tile) {
                    (1, 1) | (0, 0) => true,
                    (0, 1) => false,
                    (1, 0) => match clues.previous() {
                        Some(&prev_clue) => prev_clue == 0,
                        None => true,
                    },
                    _ => unreachable!(),
                },
                None => *permutation_tile == 0,
            })
    }

    // TODO: apply permutation
    fn add(&mut self, permutation: &[Tile; T]) {
        for (clues, permutation_tile) in self.0.iter_mut().zip(permutation) {
            match (clues.current(), permutation_tile) {
                (Some(0), 0) | (_, 1) if clues.stack.len() > 1 => { clues.index += 1; }
                _ => (),
            }
        }
    }

    fn remove(&mut self, permutation: &[Tile; T]) {
        // TODO: undo permutation
        for (clues, permutation_tile) in self.0.iter_mut().zip(permutation) {
            match (clues.previous(), permutation_tile) {
                (Some(0), 0) | (_, 1) => clues.index -= 1,
                _ => (),
            }
        }
    }
}

// https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
fn solve_nonogram<const T: usize>(
    (top_clues, left_clues): ([&'static [Tile]; T], [&'static [Tile]; T]),
) -> [[Tile; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<[u8; T]> = Vec::with_capacity(T);
    let mut counter = 0;

    if solve_nongram_rec(
        &mut counter,
        &mut processed_top_clues,
        left_clues,
        &mut permutations_stack,
    ) {
        return permutations_stack.try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    fn solve_nongram_rec<const T: usize>(
        counter: &mut u32,
        top_clues: &mut Clues<T>,
        left_clues: [&'static [u8]; T],
        permutations_stack: &mut Vec<[u8; T]>,
    ) -> bool {
        //        println!("counter: {:?}", &counter);
        //        *counter += 1;
        //        print(permutations_stack.clone());
        //        println!();
        //        println!("{:?}", &top_clues);
        //        println!("---------------------------------------");

        let current_clues_index = permutations_stack.len();
        for permutation in get_permutations_rec::<T>(left_clues[current_clues_index], 0) {
            if !top_clues.can_be_used(&permutation) {
                continue;
            }

            top_clues.add(&permutation);
            permutations_stack.push(*permutation);
            if permutations_stack.len() == T
                || solve_nongram_rec(counter, top_clues, left_clues, permutations_stack)
            {
                return true;
            }
            top_clues.remove(&permutation);
            permutations_stack.pop();
        }
        false
    }
}

fn get_permutations_rec<const N: usize>(
    clues: &'static [u8],
    init_offset: usize,
) -> Box<dyn Iterator<Item = Box<[u8; N]>>> {
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

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let vec_vec = transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
            .map(|iter| iter.collect_vec())
            .collect_vec();
        println!("{:?}", vec_vec);
    }

    #[test]
    fn get_permutations_15_test() {
        let permutations = get_permutations_rec::<15>(&[1, 2, 3, 1], 0)
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
        //        itertools::assert_equal(permutations.first().unwrap(), vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0].iter());
        //        print(permutations);
    }

    #[test]
    fn get_permutations_5_test() {
        let permutations = get_permutations_rec::<5>(&[2, 2], 0)
            .map(|perm| perm.to_vec())
            .collect_vec();

        print(permutations);
    }

    #[test]
    fn get_permutations_single_clue_test() {
        let permutations = get_permutations_rec::<15>(&[1], 0)
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

    const CLUES_3: ([&[u8]; 15], [&[u8]; 15]) = (
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

    pub const ANS_3: [[u8; 15]; 15] = [
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
