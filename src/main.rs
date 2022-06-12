use bit_set::BitSet;
use bit_vec::BitVec;
use core::iter;
use counter::Counter;
use itertools::{Itertools, PeekingNext};
use num::Complex;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Index;

use rust_sandbox::plants_and_zombies::example_tests;

fn main() {
    example_tests::tests()
}

//fn main() {
//    let data = vec![1, 3, -2, -2, 1, 0, 1, 2];
//    // groups:     |---->|------>|--------->|
//
//    // Note: The `&` is significant here, `GroupBy` is iterable
//    // only by reference. You can also call `.into_iter()` explicitly.
//    let mut data_grouped = Vec::new();
//    for (key, group) in &data.into_iter().group_by(|elt| *elt >= 0) {
//        data_grouped.push((key, group.collect()));
//    }
//    assert_eq!(
//        data_grouped,
//        vec![
//            (true, vec![1, 3]),
//            (false, vec![-2, -2]),
//            (true, vec![1, 0, 1, 2])
//        ]
//    );
//
//}
