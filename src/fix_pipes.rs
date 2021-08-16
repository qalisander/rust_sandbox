use itertools::Itertools;
use std::ops::{Add, Neg, Sub};
use std::fmt::{Display, Formatter};
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

impl Point {
    fn get_all_nearest(self) -> impl Iterator<Item=Point> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(move |&(dx, dy)| Point(dx, dy) + self)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output { Point(self.0 - other.0, self.1 - other.1) }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output { Point(self.0 + other.0, self.1 + other.1) }
}

struct PipeChecker{
    map: Vec<Vec<Tile>>
}

impl Display for PipeChecker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.map.iter().flat_map(|vec| vec.iter().map(|tile| { 
            if tile.is_visited { "*".to_string() } else { tile.symbol.to_string() }
        }).chain(std::iter::once("\n".to_string()))).collect::<String>();
        write!(f, "{}", str)
    }
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
    
    fn check(&mut self) -> bool {
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                let current_point = Point(x as isize, y as isize);
                let current_tile = self.take_tile(current_point).unwrap().to_owned();
                if current_tile.symbol == '.' || current_tile.is_visited {
                    continue;
                }

                let mut is_old_pipe = true;
                let mut is_leak_detected = false;
                self.check_rec(current_point, current_tile, &mut is_old_pipe, &mut is_leak_detected);
                if is_leak_detected && !is_old_pipe { 
                    return false;
                }
            }
        }
        return true;
    }

    fn check_rec(&mut self, current_point: Point, current_tile: Tile, is_old_pipe: &mut bool, is_leak_detected: &mut bool) {
        for nearest in current_point.get_all_nearest() {
            if current_tile.can_move_to_dir(nearest - current_point) {
                match self.take_tile(nearest) {
                    None => {
                        *is_old_pipe = false;
                    }
                    Some(mut nearest_tile) => {
                        if nearest_tile.can_move_to_dir(current_point - nearest) {
                            if !nearest_tile.is_visited {
                                nearest_tile.is_visited = true; // NOTE: create method try_visit
                                let nearest_tile = (*nearest_tile).clone();
                                self.check_rec(nearest, nearest_tile, is_old_pipe, is_leak_detected)
                            }
                        } else {
                            *is_leak_detected = true
                        }
                    }
                }
            }
        }
    }
    
    fn take_tile(&mut self, Point(x, y): Point) -> Option<&mut Tile> {
        let in_bounds = x >= 0 && x < self.map[0].len() as isize && y >= 0 && y < self.map.len() as isize;
        if in_bounds { 
            Some(&mut self.map[y as usize][x as usize])
        } else { 
            None
        }
    }
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut checker = PipeChecker::new(pipe_map);
    println!("Before:\n{}", checker);
    let ans = checker.check();
    println!("After:\n{}", checker);
    println!("{}\n", std::iter::repeat("-").take(10).collect::<String>());
    ans
}

#[cfg(test)]
mod sample_tests {
    
    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }

    #[rustfmt::skip]
    const TEST_CASES: &'static [(&'static[& str], bool)] = &[
        (&["╋━━┓", 
           "┃..┃", 
           "┛..┣"], true),
        (&["...┏",
           "┃..┃",
           "┛..┣"], false),
        (&["...┏",
           "...┃", 
           "┛..┣"], false),
        (&["...┏",
           "...┃", 
           "┓..┣"], true),
        (&["╋",
           "╋",
           "╋"], true),
        (&["╋....", 
           "┃..┛.",
           "┃...."], false),
        (&["....", 
           ".┛┛.",
           "...."], true),
        (&["....", 
           "....",
           "┓..┏"], true),
        (&["..┃.", 
           "....",
           "┓..┏"], false),
        
        (&["┻┛...", 
           "..┓..",
           ".....",
           "....┻"], false),
        
        (&["..", 
           ".┻"], false),
        
        (&["┏┛..┏━┓┃", 
           "┃..┫┣┓┣┏",
           "┛...┗┻╋┻",
           "......┃.",
           "......┃.",
           ".╋....┗┓",
           ".......┃",
           "......┏┻",
           "......┃.",
           "......┗━",
           "........",
           "...┏━━┓.",
           "┏┓.┣┳━┻━",
           "┛┗┳╋┛..."], false),
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
