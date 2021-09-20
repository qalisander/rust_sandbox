use itertools::Itertools;
use std::fmt::{Display, Debug};
use std::convert::TryInto;
use std::ops::Deref;

struct Clues(Vec<Vec<Clue>>);

impl Clues {
//    fn can_be_add(&self, perm: &[u8]) -> bool{
//        perm.iter().zip(&self.0).all(|(&u8, clues_raw)|{
//            
//        })
//    }
    fn try_add(&mut self, perm: &[u8]) -> bool {
        perm.iter().zip(&self.0).all(|(&u8, clues_raw)|{
            let x = clues_raw.iter().find(|&clue| match clue.status {
                ClueStatus::InUse(_) => {
                    
                }
                ClueStatus::Used => false,
            }).unwrap();
            true
        })
    }
}

impl<const T: usize> From<[&[u8]; T]> for Clues {
    fn from(from: [&[u8]; T]) -> Self {
        let vec_vec = from.iter().map(|&clues| {
            clues.iter().map(|clue| Clue { init_size: *clue, status: ClueStatus::InUse(*clue) }).collect_vec()
        }).collect_vec();
        Clues(vec_vec)
    }
}

struct Clue {
    init_size: u8,
    status: ClueStatus,
}

enum ClueStatus {
    InUse(u8),
    Used,
}

// https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
fn solve_nonogram<const T: usize>((top_clues, left_clues): ([&'static [u8]; T], [&'static [u8]; T])) -> [[u8; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<[u8; T]> = Vec::with_capacity(T);

    if solve_nongram_rec(&mut processed_top_clues, left_clues, &mut permutations_stack) {
        return permutations_stack.try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    // TODO: prlly create two functions: validate_field and validate_strategy (that stops further recursion)
    fn solve_nongram_rec<const T: usize>(
        top_clues: &mut Clues,
        left_clues: [&'static [u8]; T],
        permutations_stack: &mut Vec<[u8; T]>,
    ) -> bool {
        let current_depth = permutations_stack.len();
        if current_depth == T {
            return true;
        }

        for permutation in get_permutations_rec::<T>(left_clues[current_depth], 0) {
//            permutation.to_vec()

            // is_valid(permutation)

            permutations_stack.push(*permutation);
            if solve_nongram_rec(top_clues, left_clues, permutations_stack) {
                return true;
            }
        }
        false
    }

    let mut field = [[0u8; T]; T];
    print(&field);
    field
}

//TODO: use const generics https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
//TODO: pass here arr with len: u8
//TODO: pay attention to boundaries between clues (there is at least one space between them)
//TODO: write get_permutations_test
//TODO: try run Rust code from python test
//TODO: distance between permutations

fn get_permutations_rec<const N: usize>(clues: &'static [u8], init_offset: usize) -> Box<dyn Iterator<Item=Box<[u8; N]>>> {
    if clues.is_empty() {
        return Box::new(std::iter::once(Box::new([0; N])));
    }

    let current_clue = *clues.first().unwrap() as usize;
    let clues_sum = clues.iter().sum::<u8>() as usize;

    Box::new((0..=N.saturating_sub(init_offset + clues_sum))
        .flat_map(move |offset| {
            let offset = init_offset + offset;
            get_permutations_rec(&clues[1..], 1 + offset + current_clue)
                .map(move |mut slice| {
                    slice[offset..current_clue + offset].fill(1);
                    slice
                })
        }))
}

fn print<T: Debug>(field: impl IntoIterator<Item=impl IntoIterator<Item=T>>) {
    println!(
        "{}",
        field.into_iter().map(|collection|
            format!("{:?}", collection.into_iter().collect_vec())).join("\n")
    );
}

fn transpose<T: Clone>(matrix: impl IntoIterator<Item=impl IntoIterator<Item=T>>)
                       -> impl Iterator<Item=impl Iterator<Item=T>>
{
    let mut iters = matrix.into_iter()
        .map(|iter| iter.into_iter()).collect_vec(); // TODO: inte_iter() type is asent. Bug

    (0..iters.len()).map(move |_| {
        iters.iter_mut().filter_map(|iter| iter.next()).collect_vec().into_iter()
    })
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let vec_vec = transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
            .map(|iter| iter.collect_vec()).collect_vec();
        println!("{:?}", vec_vec);
    }

    #[test]
    fn get_permutations_test() {
        let permutations: Vec<Box<[u8; 15]>> = get_permutations_rec(&[1, 2, 3, 1], 0).collect_vec();
        dbg!(permutations); // TODO: implement transpose method with generics T
    }

    #[test]
    fn test1() {
        assert_eq!(solve_nonogram(CLUES_1), ANS_1);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_nonogram(CLUES_2), ANS_2);
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
}
