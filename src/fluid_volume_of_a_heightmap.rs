//https://www.codewars.com/kata/5b98dfa088d44a8b000001c1/train/rust

use itertools::Itertools;
use std::collections::{BinaryHeap, VecDeque};

// 80 <= heightmap <= 100 and -50 <= depth <= 150
fn volume(heightmap: &Vec<Vec<i32>>) -> i32 {
    // TODO: prlly create new struct-wraper of heightmap,
    let mut total_volume = 0;
    let mut current_heightmap = heightmap.clone();
    // TODO: add heap optimisation
    // let heap = BinaryHeap::new();
    loop {
        let mut floor_volume = 0;
        let i_max = heightmap.len();
        let j_max = heightmap[0].len();
        'outer: for i0 in 0_usize..i_max {
            for j0 in 0_usize..j_max {
                let mut deque = VecDeque::from([(i0, j0)]);
                let mut increase_height = vec![];
                while let Some((i, j)) = deque.pop_front() {
                    increase_height.push((i, j));
                    let in_bounds = |&(i, j): &(i32, i32)| {
                        0 <= i && i < i_max as i32 && 0 <= j && j < j_max as i32
                    };
                    let has_same_height = |&(ni, nj): &(usize, usize)| {
                        current_heightmap[ni][nj] == current_heightmap[i][j]
                    };
                    let has_lower_height = |&(ni, nj): &(usize, usize)| {
                        current_heightmap[ni][nj] < current_heightmap[i][j]
                    };
                    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    let next_points = dirs
                        .into_iter()
                        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                        .filter(in_bounds)
                        .map(|(i, j)| (i as usize, j as usize))
                        .collect_vec();
                    
                    if next_points.iter().any(has_lower_height){
                        break 'outer;
                    }
                    
                    deque.extend(next_points.into_iter().filter(has_same_height));
                }

                for (i, j) in increase_height {
                    current_heightmap[i][j] += 1;
                }
            }
        }

        if floor_volume != 0 {
            total_volume += floor_volume;
        } else {
            break total_volume;
        }
    }
}

// Add your own tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    // this just helps with the test output on failure.
    fn pretty_test(map: &Vec<Vec<i32>>, expected: i32) {
        let result = volume(&map);
        let mut printy = String::new();
        for row in map {
            printy.push_str(format!("{:?}\n", row).as_str());
        }
        assert_eq!(
            result, expected,
            "\nYour result (left) did not match expected result (right) for map:\n{}",
            printy
        );
    }

    #[test]
    fn small_maps_test() {
        let tests = [
            (vec![vec![0]], 0),
            (vec![vec![22]], 0),
            (vec![vec![2, 1, 2], vec![1, 0, 1], vec![2, 1, 2]], 1),
            (vec![vec![1, 1, 1], vec![1, 8, 1], vec![1, 1, 1]], 0),
            (
                vec![
                    vec![9, 9, 9, 9],
                    vec![9, 0, 0, 9],
                    vec![9, 0, 0, 9],
                    vec![9, 9, 9, 9],
                ],
                36,
            ),
            (
                vec![
                    vec![9, 9, 9, 9, 9],
                    vec![9, 0, 1, 2, 9],
                    vec![9, 7, 8, 3, 9],
                    vec![9, 6, 5, 4, 9],
                    vec![9, 9, 9, 9, 9],
                ],
                45,
            ),
            (
                vec![
                    vec![8, 8, 8, 8, 6, 6, 6, 6],
                    vec![8, 0, 0, 8, 6, 0, 0, 6],
                    vec![8, 0, 0, 8, 6, 0, 0, 6],
                    vec![8, 8, 8, 8, 6, 6, 6, 0],
                ],
                56,
            ),
            (
                vec![
                    vec![0, 10, 0, 20, 0],
                    vec![20, 0, 30, 0, 40],
                    vec![0, 40, 0, 50, 0],
                    vec![50, 0, 60, 0, 70],
                    vec![0, 60, 0, 70, 0],
                ],
                150,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                ],
                0,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 3, 3, 2, 3],
                    vec![3, 1, 1, 1, 3],
                    vec![3, 1, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                ],
                0,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 1, 3],
                ],
                11,
            ),
            (
                vec![
                    vec![3, 3, 3, 1, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 3, 3],
                ],
                11,
            ),
        ];

        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn negative_heights_tests() {
        let tests = [
            (vec![vec![-1]], 0),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, -2, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 1, -3],
                ],
                13,
            ),
            (
                vec![
                    vec![8192, 8192, 8192, 8192],
                    vec![8192, -8192, -8192, 8192],
                    vec![8192, -8192, -8192, 8192],
                    vec![8192, 8192, 8192, 8192],
                ],
                65536,
            ),
        ];

        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn large_map_test() {
        // 50x50 map without leaks; 100 around the border, 0 inside
        let mut map = vec![vec![100; 50]; 50];
        for y in 1..49 {
            for x in 1..49 {
                map[y][x] = 0;
            }
        }
        // volume = 100 * (48 * 48)
        pretty_test(&map, 230_400);
    }
}
