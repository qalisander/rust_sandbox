use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
// https://www.codewars.com/kata/59f81fe146d84322ed00001e

#[derive(Debug, Copy, Clone)]
struct Tile {
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
    fn get_all_nearest(self) -> impl Iterator<Item = Point> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |&(dx, dy)| Point(dx, dy) + self)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

struct PipeChecker {
    map: Vec<Vec<Tile>>,
}

impl Display for PipeChecker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self
            .map
            .iter()
            .flat_map(|vec| {
                vec.iter()
                    .map(|tile| {
                        if tile.is_visited {
                            "*".to_string()
                        } else {
                            tile.symbol.to_string()
                        }
                    })
                    .chain(std::iter::once("\n".to_string()))
            })
            .collect::<String>();
        write!(f, "{}", str)
    }
}

impl PipeChecker {
    fn new(pipe_map: &[&str]) -> PipeChecker {
        Self {
            map: pipe_map
                .iter()
                .map(|slice| {
                    slice
                        .chars()
                        .map(|ch| Tile {
                            symbol: ch,
                            is_visited: false,
                        })
                        .collect::<Vec<Tile>>()
                })
                .collect_vec(),
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
                self.check_rec(
                    current_point,
                    current_tile,
                    &mut is_old_pipe,
                    &mut is_leak_detected,
                );
                if is_leak_detected && !is_old_pipe {
                    return false;
                }
            }
        }
        true
    }

    fn check_rec(
        &mut self,
        current_point: Point, // NOTE: join point and tile in same struct
        current_tile: Tile,
        is_old_pipe: &mut bool,
        is_leak_detected: &mut bool,
    ) {
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
                                let nearest_tile = *nearest_tile;
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
        let in_bounds =
            x >= 0 && x < self.map[0].len() as isize && y >= 0 && y < self.map.len() as isize;
        if in_bounds {
            Some(&mut self.map[y as usize][x as usize])
        } else {
            None
        }
    }
}

pub(super) fn check_pipe(pipe_map: &[&str]) -> bool {
    let mut checker = PipeChecker::new(pipe_map);
    println!("Before:\n{}", checker);
    let ans = checker.check();
    println!("After:\n{}", checker);
    println!("{}\n", ['-'; 20].iter().collect::<String>());
    ans
}

// NOTE smb's attractive solution
//pub fn check_pipe(pipe_map: &[&str]) -> bool {
//    let mut checker = PipeChecker::new(pipe_map);
//    println!("Before:\n{}", checker);
//    let ans = checker.check();
//    println!("After:\n{}", checker);
//    println!("{}\n", ['-'; 20].iter().collect::<String>());
//    ans
//}
//
//struct Cell {
//    visited: u8,
//    pipes: u8,
//    x: usize,
//    y: usize,
//}
//
//const TOP: u8 = 1 << 0;
//const RIGHT: u8 = 1 << 1;
//const BOTTOM: u8 = 1 << 2;
//const LEFT: u8 = 1 << 3;
//
//fn to_mask(c: char) -> u8 {
//    match c {
//        '┗' => TOP | RIGHT,
//        '┓' => BOTTOM | LEFT,
//        '┏' => BOTTOM | RIGHT,
//        '┛' => TOP | LEFT,
//        '━' => LEFT | RIGHT,
//        '┃' => BOTTOM | TOP,
//        '┣' => BOTTOM | TOP | RIGHT,
//        '┫' => BOTTOM | TOP | LEFT,
//        '┳' => LEFT | RIGHT | BOTTOM,
//        '┻' => LEFT | RIGHT | TOP,
//        '╋' => LEFT | RIGHT | TOP | BOTTOM,
//        _ => 0
//    }
//}
//
//fn to_cells(pipe_map: &[&str]) -> Vec<Vec<Cell>> {
//    return pipe_map.iter().enumerate().map(|(y, str)| {
//        str.chars().enumerate().map(|(x, c)| { Cell { visited: 0, pipes: to_mask(c), x, y } }).collect()
//    }).collect();
//}
//
//
//fn is_leaking(x: usize, y: usize, origin: u8, map: &mut Vec<Vec<Cell>>) -> bool {
//    {
//        let mut cell = &mut map[y][x];
//        if (cell.visited & origin) != 0 {
//            return false;
//        }
//        cell.visited |= origin;
//    }
//
//    let pipes = map[y][x].pipes;
//
//    return (pipes & origin) == 0 ||
//        (origin != TOP && (pipes & TOP) != 0 && y > 0 && is_leaking(x, y - 1, BOTTOM, map)) ||
//        (origin != BOTTOM && (pipes & BOTTOM) != 0 && y < (map.len() - 1) && is_leaking(x, y + 1, TOP, map)) ||
//        (origin != LEFT && (pipes & LEFT) != 0 && x > 0 && is_leaking(x - 1, y, RIGHT, map)) ||
//        (origin != RIGHT && (pipes & RIGHT) != 0 && x < (map[0].len() - 1) && is_leaking(x + 1, y, LEFT, map));
//}
//
//pub fn check_pipe(pipe_map: &[&str]) -> bool {
//    let mut cells = to_cells(pipe_map);
//    let map_width = cells[0].len();
//    let map_height = cells.len();
//    !((0..map_height).any(|y| {
//        (cells[y][0].pipes & LEFT) != 0 && is_leaking(0, y, LEFT, &mut cells) ||
//            (cells[y].last().unwrap().pipes & RIGHT) != 0 && is_leaking(map_width - 1, y, RIGHT, &mut cells)
//    }) || (0..map_width).any(|x| {
//        (cells[0][x].pipes & TOP) != 0 && is_leaking(x, 0, TOP, &mut cells) ||
//            (cells.last().unwrap()[x].pipes & BOTTOM) != 0 && is_leaking(x, map_height - 1, BOTTOM, &mut cells)
//    }))
//}