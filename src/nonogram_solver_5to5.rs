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
fn get_permutations(clues: &'static [u8], len: u8) -> Vec<Vec<u8>> {
    if clues.is_empty() {
        return vec![vec![]];
    }

    let current_clue = clues.first().expect("Clues are not empty");
    let sum_of_others: u8 = clues.iter().skip(1).sum();

    //TODO: anyway here will get empty vector, prlly remove check on line 25
    (0_u8..(len - current_clue - sum_of_others))
        .map(move |offset| {
            get_permutations(&clues[1..], len - current_clue - offset)
                .into_iter()
                .flat_map(move |perm| {
                    let zeroes = std::iter::repeat(0_u8).take(offset as usize);
                    let ones = std::iter::repeat(1_u8).take(*current_clue as usize);
                    let current_clue_position = zeroes.chain(ones);
                    current_clue_position.chain(perm.into_iter())
                })
                .collect_vec()
        })
        .collect_vec()
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
    fn get_permutations_test(){
        let permutations = get_permutations(&[1, 2], 5);
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
