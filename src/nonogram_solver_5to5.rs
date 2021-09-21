use itertools::Itertools;
use std::fmt::{Display, Debug};
use std::convert::TryInto;
use std::iter::{Zip, FilterMap, Rev};
use trace::trace;
trace::init_depth_var!();

type Tile = u8;
#[derive(Debug)]
struct Clues<const T: usize>([Vec<Clue>; T]);

impl<const T: usize> Clues<T> {
    #[trace(pretty)]
    fn can_be_subtracted(&self, tiles: &[Tile; T]) -> bool {
        tiles.iter().zip(&self.0).all(|(&tile, clues_row)| {
            for clue in clues_row {
                if let ClueStatus::InUse(ref current_size) = clue.status {
                    return match tile {
                        1 => clue.init_size == *current_size,
                        0 => clue.init_size == 0,
                        _ => panic!("Invalid tile value!")
                    }
                }
            }
            tile == 0
        })
    }

    fn subtract(&mut self, tiles: &[Tile; T]){
        for clues_row in self.filtered_clues_rows(tiles){
            for clue in clues_row.iter_mut() {
                match clue.status {
                    ClueStatus::InUse(0) => {
                        clue.status = ClueStatus::Used;
                        break;
                    },
                    ClueStatus::InUse(ref mut current_size) => {
                        *current_size -= 1;
                        break;
                    },
                    _ => (),
                }
            }
        }
    }

    fn add_back(&mut self, tiles: &[Tile; T]){
        for clues_row in self.filtered_clues_rows(tiles).rev() {
            for clue in clues_row.iter_mut() {
                match clue.status {
                    ClueStatus::InUse(ref mut current_size) if *current_size != clue.init_size=> {
                        *current_size += 1;
                        break;
                    },
                    ClueStatus::Used => {
                        clue.status = ClueStatus::InUse(0);
                        break;
                    },
                    _ => (),
                }
            }
        }
    }

    fn filtered_clues_rows<'a>(&'a mut self, tiles: &'a [Tile; T]) -> impl DoubleEndedIterator<Item=&'a mut Vec<Clue>> {
        tiles.iter()
            .zip(&mut self.0)
            .filter_map(|(&tile, clues_row)| if tile == 1 { Some(clues_row) } else { None })
    }
}

impl<const T: usize> From<[&[Tile]; T]> for Clues<T> {
    fn from(from: [&[u8]; T]) -> Self {
        let slice_vec = from.iter().map(|&clues| {
            clues.iter().map(|clue| {
                Clue { init_size: *clue, status: ClueStatus::InUse(*clue) }
            }).collect_vec()
        }).collect_vec().try_into().unwrap();
        Clues(slice_vec)
    }
}

#[derive(Debug)]
struct Clue {
    init_size: u8,
    status: ClueStatus,
}

#[derive(Debug)]
enum ClueStatus {
    InUse(u8),
    Used,
}

// https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
fn solve_nonogram<const T: usize>((top_clues, left_clues): ([&'static [Tile]; T], [&'static [Tile]; T])) -> [[Tile; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<[u8; T]> = Vec::with_capacity(T);

    if solve_nongram_rec(&mut processed_top_clues, left_clues, &mut permutations_stack) {
        return permutations_stack.try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    fn solve_nongram_rec<const T: usize>(
        top_clues: &mut Clues<T>,
        left_clues: [&'static [u8]; T],
        permutations_stack: &mut Vec<[u8; T]>,
    ) -> bool {
        print(permutations_stack.clone());

        let current_clues_index = permutations_stack.len();
        for permutation in get_permutations_rec::<T>(left_clues[current_clues_index], 0)/*.inspect(|perm| {dbg!(perm);})*/ {
            if !top_clues.can_be_subtracted(&permutation) {
                continue;
            }

            top_clues.subtract(&permutation);
            permutations_stack.push(*permutation);
            if permutations_stack.len() == T || solve_nongram_rec(top_clues, left_clues, permutations_stack) {
                return true;
            }
            top_clues.add_back(&permutation);
            permutations_stack.pop();
        }
        false
    }
}

//TODO: try run Rust code from python test
fn get_permutations_rec<const N: usize>(clues: &'static [u8], init_offset: usize) -> Box<dyn Iterator<Item=Box<[u8; N]>>> {
    if clues.is_empty() {
        return Box::new(std::iter::once(Box::new([0; N])));
    }

    let current_clue = *clues.first().unwrap() as usize;
    let clues_sum = clues.iter().sum::<u8>() as usize;
    let clues_borders = clues.len() - 1;

    Box::new((0..=N.saturating_sub(init_offset + clues_sum + clues_borders))
        .flat_map(move |offset| {
            let offset = init_offset + offset;
            get_permutations_rec(&clues[1..], 1 + offset + current_clue)
                .map(move |mut slice| {
                    slice[offset..current_clue + offset].fill(1);
                    slice
                })
        }))
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
    fn get_permutations_15_test() {
        let permutations = get_permutations_rec::<15>(&[1, 2, 3, 1], 0).map(|perm| perm.to_vec()).collect_vec();
        assert_eq!(permutations.first().unwrap(), &vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0]);
        assert_eq!(permutations.last().unwrap(), &vec![0u8, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1]);
//        itertools::assert_equal(permutations.first().unwrap(), vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0].iter());
//        print(permutations);
    }

    #[test]
    fn get_permutations_single_clue_test() {
        let permutations = get_permutations_rec::<15>(&[1], 0).map(|perm| perm.to_vec()).collect_vec();
        assert_eq!(permutations.first().unwrap(), &vec![1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(permutations.last().unwrap(), &vec![0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
//        print(permutations);
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
