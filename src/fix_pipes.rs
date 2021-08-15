use itertools::Itertools;
use std::ops::{Add, Neg, Sub};
// https://www.codewars.com/kata/59f81fe146d84322ed00001e

#[derive(Debug, Copy, Clone)]
struct  Tile {
    symbol: char,
    is_visited: bool,
}

impl Tile {
    #[rustfmt::skip]
    fn can_move_to_dir(&self, Point(dx, dy): Point) -> bool {
        match self.symbol {
            '┃' | '┣' | '┫' | '╋' if dx == 0 || dy.abs() == 1 => true,       //      |
            '━' | '┳' | '┻' | '╋' if dx.abs() == 1 || dy == 0 => true,       //      |
            '┛' | '┫' if (-1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true, // -----|----->
            '┗' | '┻' if (1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true,  //      |     (x)
            '┓' | '┳' if (-1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,  //      V (y)
            '┏' | '┣' if (1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,
            _ => false,
        }
    }
}

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

struct PipeChecker{
    map: Vec<Vec<Tile>>
}

impl PipeChecker {
    fn new(pipe_map: &[&str]) -> PipeChecker {
        Self {
            map: pipe_map.iter().map(|slice| {
                slice.chars().map(|ch| Tile {
                    symbol: ch,
                    is_visited: false,
                }).collect::<Vec<Tile>>()
            }).collect_vec()
        }
    }
    
    fn in_bounds(&self, &Point(x, y): &Point) -> bool {
        x >= 0 && x < self.map[0].len() as isize && y >= 0 && y < self.map.len() as isize
    }

    // TODO: difference between [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map and [(-1, 0), (1, 0), (0, -1), (0, 1)].map
    
    fn check(&self) -> bool {

        for (y, inner_map) in self.map.iter().enumerate() {
            for (x, &current_tile) in inner_map.into_iter().enumerate() {
                if current_tile.symbol == '.'{
                    continue;
                }

                let nearest = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];

                let current = Point(x as isize, y as isize);
                let leak_detected = get_nearest_of(current).filter(|pnt| self.in_bounds(pnt)).any(|nearest| {
                    let nearest_tile = self.map[nearest.1 as usize][nearest.0 as usize];
                    current_tile.can_move_to_dir(nearest - current) && !nearest_tile.can_move_to_dir(current - nearest)
                });

                if leak_detected {
                    return false;
                };
            }
        }
        return true;
        
        fn check_rec(current: Point) -> bool {
            unimplemented!();
        }
        
        fn get_nearest_of(point: Point) -> impl Iterator<Item=Point>{
            [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(move |(dx, dy)| Point(*dx, *dy) + point)
        }
    }
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    let pipe_checker = PipeChecker::new(pipe_map);
    pipe_checker.check()
}

#[cfg(test)]
mod sample_tests {
    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in &TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }

    #[rustfmt::skip]
    const TEST_CASES: [([&str; 3], bool); 7] = [
        (["╋━━┓", 
          "┃..┃", 
          "┛..┣"], true),
        (["...┏",
          "┃..┃",
          "┛..┣"], false),
        (["...┏",
          "...┃", 
          "┛..┣"], false),
        (["...┏",
          "...┃", 
          "┓..┣"], true),
        (["╋",
          "╋",
          "╋"], true),
        (["╋....", 
          "┃..┛.",
          "┃...."], false),
        (["....", 
          ".┛┛.",
          "...."], true),
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
