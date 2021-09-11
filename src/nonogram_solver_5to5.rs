use itertools::Itertools;
use std::fmt::Display;

// https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
fn solve_nonogram((top_clues, left_clues): ([&[u8]; 5], [&[u8]; 5])) -> [[u8; 5]; 5] {
    //    |-------> (x)
    //    |
    //    |
    //    V (y)

    fn solve_nongram_rec(
        y_index: u32,
        (top_clues, left_clues): &([&[u8]; 5], [&[u8]; 5]),
        field: &mut [[u8; 5]; 5],
    ) -> bool {
        todo!("fn: partition by clue");
        todo!("validate recursion");
    }

    let mut field = [[0u8; 5]; 5];
    print(&field);

    // TODO: prlly create two functions: validate_field and validate_strategy (that stops further recursion)
    if solve_nongram_rec(0, &(top_clues, left_clues), &mut field) {
        field
    } else {
        panic!("Solution not found")
    }
}

//TODO: use const generics https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
//TODO: pass here arr with len: u8
//TODO: pay attention to boundaries between clues (there is at least one space between them)
//TODO: write get_permutations_test

fn get_permutations<const N: usize>(clues: &'static [u8], init_offset: usize) -> Box<dyn Iterator<Item=Box<[u8; N]>>> {
    if clues.is_empty() {
        return Box::new(std::iter::once(Box::new([0; N])));
    }

    let current_clue = *clues.first().unwrap() as usize;

    Box::new((0..=N - init_offset - clues.iter().sum::<u8>() as usize)
        .flat_map(move |offset| {
            let offset = init_offset + offset;
            get_permutations(&clues[1..], offset + current_clue)
                .map(move |mut slice| {
                    slice.iter_mut().skip(offset).take(current_clue).for_each(|mut item| *item = 1);
                    slice
                })
        }))
}

fn print(field: &[[u8; 5]; 5]) {
    println!(
        "{}",
        field.iter().map(|slice| format!("{:?}", slice)).join("\n")
    );
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn get_permutations_test() {
        let permutations: Vec<Box<[u8; 5]>> = get_permutations(&[1, 2], 0).collect_vec();
        dbg!(permutations);
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
