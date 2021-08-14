use itertools::Itertools;
use std::ops::{Add, Neg, Sub};
// https://www.codewars.com/kata/59f81fe146d84322ed00001e

type Tile = char;

#[derive(Debug, Copy, Clone)]
struct Point(isize, isize);

impl Sub for Point {
    type Output = Self;

    // TODO: saturating subtraction don't use isize
    fn sub(self, other: Self) -> Self::Output { Point(self.0 - other.0, self.1 - other.1) }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output { Point(self.0 + other.0, self.1 + other.1) }
}

//struct Tile {
//    ch: char,
//    x: isize,
//    y: isize,
//}

//impl Tile {
//        pub fn can_move_to(&self, tile: Tile) -> bool {
//            self.can_move_to_dir(tile.x - self.x, tile.y - self.y)
//        }
//
//    #[rustfmt::skip]
//    fn can_move_to_dir(&self, Point(dx, dy): Point) -> bool {
//        match self.ch {
//            '┃' | '┣' | '┫' | '╋' if dx == 0 || dy.abs() == 1 => true,       //      |
//            '━' | '┳' | '┻' | '╋' if dx.abs() == 1 || dy == 0 => true,       //      |
//            '┛' | '┫' if (-1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true, // -----|----->
//            '┗' | '┻' if (1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true,  //      |     (x)
//            '┓' | '┳' if (-1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,  //      V (y)
//            '┏' | '┣' if (1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,
//            _ => false,
//        }
//    }
//}

fn check_pipe(pipe_map: &[&str]) -> bool {
    let map = pipe_map.iter().map(|slice| {
        slice.chars().collect::<Vec<Tile>>()
    }).collect_vec();

    let in_bounds = |&Point(x, y): &Point| -> bool {
        x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize
    };

    // TODO: difference between [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map and [(-1, 0), (1, 0), (0, -1), (0, 1)].map
    fn get_nearest_of(point: Point) -> impl Iterator<Item=Point>{
        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(move |(dx, dy)| Point(*dx, *dy) + point)
//        (x - 1..=x + 1).flat_map(move |x| (y - 1..=y + 1).filter_map(move |y| {
//            if (1, 1) == (x.abs(), y.abs()) || (0, 0) == (x, y) {
//                None
//            } else {
//                Some(Point(x, y))
//            }
//        }))
    }

    fn get_iter_impl<'a>() -> impl Iterator<Item=char> + 'a {
        "abcdefg".chars().enumerate()
            .map(|(i, x)| if i % 2 == 0 { Some(x) } else { None })
            .flat_map(|x| x)
    }

    #[rustfmt::skip]
    fn can_move_to_dir(ch: char, Point(dx, dy): Point) -> bool {
        match ch {
            '┃' | '┣' | '┫' | '╋' if dx == 0 || dy.abs() == 1 => true,       //      |
            '━' | '┳' | '┻' | '╋' if dx.abs() == 1 || dy == 0 => true,       //      |
            '┛' | '┫' if (-1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true, // -----|----->
            '┗' | '┻' if (1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true,  //      |     (x)
            '┓' | '┳' if (-1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,  //      V (y)
            '┏' | '┣' if (1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,
            _ => false,
        }
    }

    for (y, inner_map) in map.iter().enumerate() {
        for (x, &current_ch) in inner_map.into_iter().enumerate() {
            if current_ch == '.'{
                continue;
            }

            let nearest = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
            
            let current = Point(x as isize, y as isize);
            let leak_detected = get_nearest_of(current).filter(in_bounds).any(|nearest| {
                let nearest_ch = map[nearest.1 as usize][nearest.0 as usize];
                can_move_to_dir(current_ch, nearest - current) && !can_move_to_dir(nearest_ch, current - nearest)
            });

            if leak_detected {
                return false;
            };
        }
    }
    return true;
}
//
//fn get_dirs(ch: char) -> Vec<(isize, isize)> {
//    fn merge_dirs(ch_1: char, ch_2: char) -> Vec<(isize, isize)> {
//        get_dirs(ch_1)
//            .into_iter()
//            .chain(get_dirs(ch_2))
//            .unique()
//            .collect_vec()
//    }
//
//    match ch {
//        '┃' => vec![(1, 0), (-1, 0)],  //      |
//        '━' => vec![(0, 1), (0, -1)],  //      |
//        '┛' => vec![(-1, 0), (0, -1)], // -----|----->
//        '┗' => vec![(-1, 0), (0, 1)],  //      |     (j)
//        '┓' => vec![(1, 0), (0, -1)],  //      V (i)
//        '┏' => vec![(1, 0), (0, 1)],
//        '┳' => merge_dirs('┏', '━'),
//        '┻' => merge_dirs('┗', '━'),
//        '┫' => merge_dirs('┛', '┃'),
//        '┣' => merge_dirs('┗', '┃'),
//        '╋' => merge_dirs('┃', '━'),
//        _ => vec![],
//    }
//}

#[cfg(test)]
mod sample_tests {
    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in &TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }

    const TEST_CASES: [([&str; 3], bool); 7] = [
        (["╋━━┓", "┃..┃", "┛..┣"], true),
        (["...┏", "┃..┃", "┛..┣"], false),
        (["...┏", "...┃", "┛..┣"], false),
        (["...┏", "...┃", "┓..┣"], true),
        (["╋", "╋", "╋"], true),
        (["╋....", "┃..┛.", "┃...."], false),
        (["....", ".┛┛.", "...."], true),
    ];
}

fn run_test(pmap: &[&str], answer: bool) {
    let test_result = check_pipe(pmap);
    assert!(
        test_result == answer,
        "Output: {}; expected value: {}; for input:\n{}\n",
        test_result,
        answer,
        pmap.join("\n")
    );
}
