use core::iter;
use std::ops::Index;
use bit_set::{BitSet};
use bit_vec::BitVec;
use itertools::{Itertools, PeekingNext};
use rust_lab::nonogram_solvers::nonogram_solver_slice::solve_nonogram;
use rust_lab::nonogram_solvers::test::{CLUES_15, print};

fn main() {

//    dbg!(1 + 1);

//    use itertools::Itertools;
//
//    let mut hexadecimals = "0123456789abcdef".chars();
//    let mut hexadecimals = hexadecimals.peekable();
//    let peeked = hexadecimals.peeking_next(|ch| *ch == '0');
//    assert_eq!(peeked, Some('0'));
//
//    let decimals = hexadecimals.take_while_ref(|c| c.is_numeric())
//        .collect::<String>();
//    assert_eq!(decimals, "0123456789");
//    assert_eq!(hexadecimals.next(), Some('a'));

}

// struct Solution;

// use std::i32::{self, MAX};

// impl Sulution {
//     fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
//         arr.sort_unstable();
//         let min = arr
//         .windows(2)
//         .fold(i32:MAX, |x, v| i32::min(x, v[1] - v[0]));
//         let mut res: Vec<Vec<i32>> = vec![];
//         for v in arr.windows(2) {
//             res.push(v.to_vec())
//         }
//         res
//     }
// }

// #[text]
// fn test() {
//     let arr = vec![4, 2, 1, 3];
//     let res: Vec<Vec<i32>> = vec_vec_i32![[1,2], [2,3], [2, 4]];
//     assert_eq!(Solution::minimum_abs_difference(arr), res);
//     let arr = vec![1, 2, 6, 10, 15];
//     let res: Vec<Vec<i32>> = vec_vec_i32![[1, 3]];
//     assert_eq!(Solution::minimum_abs_difference(arr), res);
//     let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
//     let res: Vec<Vec<i32>> = vec_vec_i32![[-14, -10], [19, 23], [23, 27]];
//     assert_eq!(Solution::mimimum_abs_difference(arr), res);
// }