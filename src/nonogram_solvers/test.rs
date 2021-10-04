#[cfg(test)]
use crate::nonogram_solvers::*;
extern crate test;
use self::test::Bencher;
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
    let permutations = get_permutations::<15>(
        &[1, 2, 3, 1],
        BitVec::from_elem(15, false),
        BitVec::from_elem(15, false),
    )
    .map(|perm| perm.iter().map(|bit| bit as u8).collect_vec())
    .collect_vec();
    //        print(&permutations);
    assert_eq!(
        permutations.first().unwrap(),
        &[1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
        "first permutation failed"
    );
    assert_eq!(
        permutations.last().unwrap(),
        &[0u8, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
        "last permutation failed"
    );
    // itertools::assert_equal(permutations.first().unwrap(), vec![1u8, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0].iter());
}

#[test]
fn get_permutations_5_test() {
    let permutations = get_permutations::<5>(
        &[2, 2],
        BitVec::from_elem(5, false),
        BitVec::from_elem(5, false),
    )
    .map(|perm| perm.iter().map(|bit| bit as u8).collect_vec())
    .collect_vec();

    assert_eq!(permutations.first().unwrap(), &[1u8, 1, 0, 1, 1]);

    let permutations = get_permutations::<5>(
        &[2, 1],
        BitVec::from_elem(5, false),
        BitVec::from_elem(5, false),
    )
        .map(|perm| perm.iter().map(|bit| bit as u8).collect_vec())
        .collect_vec();

    assert_eq!(permutations.first().unwrap(), &[1u8, 1, 0, 1, 0]);
}

#[test]
fn get_permutations_single_clue_test() {
    let permutations = get_permutations::<15>(
        &[1],
        BitVec::from_elem(15, false),
        BitVec::from_elem(15, false),
    )
    .map(|perm| perm.iter().map(|bit| bit as u8).collect_vec())
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

#[test]
fn test15() {
    assert_eq!(solve_nonogram(CLUES_15), ANS_15);
}

#[bench]
//#[ignore]
fn bench15(bencher: &mut Bencher) {
    bencher.iter(|| solve_nonogram(CLUES_15));
}

#[test]
fn test25() {
    print(solve_nonogram(CLUES_25));
}

#[test]
#[ignore]
fn test50() {
    const COUNT: usize = 35;
    let arr_top = (1..6).collect_vec();
    let arr_left = (1..6).rev().collect_vec();
    let keys_top = (0..COUNT).map(|_| &arr_top[..]).collect_vec();
    let keys_left = (0..COUNT).map(|_| &arr_left[..]).collect_vec();
    print(solve_nonogram::<COUNT>((
        keys_top.try_into().unwrap(),
        keys_left.try_into().unwrap(),
    )));
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

const CLUES_3: ([&[u8]; 5], [&[u8]; 5]) = (
    [&[1, 1], &[1, 1], &[4], &[3], &[1, 1]],
    [&[3], &[1, 1], &[2], &[3], &[2, 1]],
);

const ANS_3: [[u8; 5]; 5] = [
    [1, 1, 1, 0, 0],
    [0, 0, 1, 0, 1],
    [0, 0, 1, 1, 0],
    [0, 0, 1, 1, 1],
    [1, 1, 0, 1, 0],
];

pub const CLUES_15: ([&[u8]; 15], [&[u8]; 15]) = (
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

pub const ANS_15: [[u8; 15]; 15] = [
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

pub const CLUES_25: ([&[u8]; 25], [&[u8]; 25]) = (
    [
        &[1, 1, 4],
        &[2, 1, 1, 2],
        &[2, 1, 4, 3, 2],
        &[11, 7],
        &[7, 1, 6],
        &[3, 4, 7, 2],
        &[2, 2, 7, 1, 2],
        &[4, 3, 1, 2],
        &[7, 3, 1, 3],
        &[6, 1, 3],
        &[3, 2, 3],
        &[1, 4, 2],
        &[1, 1, 8, 1],
        &[3, 9, 3],
        &[3, 8, 1, 5],
        &[3, 4, 2, 3, 4],
        &[10, 3, 1, 1, 5],
        &[3, 4, 3, 3],
        &[8, 1, 1, 8],
        &[6, 9],
        &[7, 1, 9],
        &[3, 3, 9],
        &[1, 2, 6],
        &[1, 5],
        &[1, 4, 1],
    ],
    [
        &[5, 3],
        &[5, 5],
        &[3, 7, 1],
        &[3, 2, 2, 6],
        &[2, 2, 2, 6],
        &[1, 3, 3, 1, 3],
        &[8, 3, 3],
        &[1, 7, 3, 3, 2],
        &[3, 1, 3, 6, 1],
        &[2, 2, 7, 3],
        &[3, 3, 1, 2],
        &[1, 3, 3, 2],
        &[1, 7],
        &[11, 1],
        &[3, 3, 3],
        &[1, 4, 3, 4, 1],
        &[1, 3, 10],
        &[1, 7, 1, 6, 1],
        &[7, 11],
        &[5, 1, 7],
        &[3, 13],
        &[4, 11],
        &[1, 1, 3, 12],
        &[3, 7, 2],
        &[1, 7, 1],
    ],
);
