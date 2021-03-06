//https://www.codewars.com/kata/5b98dfa088d44a8b000001c1/train/rust

use itertools::Itertools;
use std::collections::{BinaryHeap, HashSet, VecDeque};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Cell {
    ord: i32,
    index: (usize, usize),
}

impl Cell {
    pub fn new(height: i32, index: (usize, usize)) -> Self {
        Self {
            ord: -height,
            index,
        }
    }

    pub fn height(&self) -> i32 {
        -self.ord
    }
}

// 80 <= heightmap <= 100 and -50 <= depth <= 150
fn volume(heightmap: &Vec<Vec<i32>>) -> i32 {
    let i_max = heightmap.len();
    let j_max = heightmap[0].len();
    let mut total_volume = 0;
    let mut heightmap = heightmap.clone();
    let cells = heightmap.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .enumerate()
            .map(move |(j, heigh)| Cell::new(*heigh, (i, j)))
    });
    let mut cells_heap = BinaryHeap::from_iter(cells);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut prev_height = cells_heap.peek().unwrap().height();
    while let Some(cell) = cells_heap.pop() {
        if prev_height != cell.height() {
            visited.clear()
        }
        if visited.contains(&cell.index) {
            continue;
        }

        let mut has_increase_height = true;
        let mut increase_height_candidates = vec![];
        let mut deque = VecDeque::from([cell.index]);
        while let Some((i, j)) = deque.pop_front() {
            if visited.contains(&(i, j)) {
                continue;
            }
            increase_height_candidates.push((i, j));
            visited.insert((i, j));

            let in_bounds =
                |&(i, j): &(i32, i32)| 0 <= i && i < i_max as i32 && 0 <= j && j < j_max as i32;
            let has_same_height = |&(ni, nj): &(usize, usize)| heightmap[ni][nj] == heightmap[i][j];
            let has_lower_height = |&(ni, nj): &(usize, usize)| heightmap[ni][nj] < heightmap[i][j];
            let to_usize = |(i, j): (i32, i32)| (i as usize, j as usize);

            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let next_points = dirs
                .into_iter()
                .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                .collect_vec();

            if has_increase_height {
                has_increase_height = next_points.iter().all(in_bounds)
                    && !next_points
                        .iter()
                        .cloned()
                        .filter(in_bounds)
                        .map(to_usize)
                        .any(|p| has_lower_height(&p));
            }

            let to_visit = next_points
                .into_iter()
                .filter(in_bounds)
                .map(to_usize)
                .filter(has_same_height);
            deque.extend(to_visit);
        }

        prev_height = cell.height();
        if has_increase_height {
            for (i, j) in increase_height_candidates {
                heightmap[i][j] += 1;
                total_volume += 1;
                cells_heap.push(Cell::new(cell.height() + 1, (i, j)))
            }
        }
    }

    total_volume
}

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
