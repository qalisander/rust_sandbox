use itertools::Itertools;

fn main() {
// Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
// 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");
// 2. Move a value out of a given index in the vector,
// and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
// 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
// Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
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