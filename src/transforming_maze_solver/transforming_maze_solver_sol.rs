// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::iter;
use std::time::Instant;

type DIR = i8;

const DIR_MASK: DIR = 0b00001111;
const E_DIR: DIR = 0b_0001;
const S_DIR: DIR = 0b_0010;
const W_DIR: DIR = 0b_0100;
const N_DIR: DIR = 0b_1000;
const BEGIN: DIR = -1;
const END: DIR = -2;

#[derive(Debug, Copy, Clone)]
enum TType {
    Visited {
        prev_tile_id: (usize, usize),
        interval_id: usize,
    },
    Unvisited,
    Begin,
    End,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    t_type: TType,
    walls: DIR,
}

//      |
//      |
// -----|----->
//      |     (j)
//      V (i)

type Grid = Vec<Vec<Tile>>;

struct Field {
    grid: Grid,
    begin: (usize, usize),
    end: (usize, usize),
}

impl Field {
    fn new(maze: &Vec<Vec<DIR>>) -> Self {
        let mut begin = None;
        let mut end = None;
        let mut process_dir = |dir, (i, j)| match dir {
            BEGIN => {
                begin = Some((i, j));
                Tile {
                    t_type: TType::Begin,
                    walls: 0,
                }
            }
            END => {
                end = Some((i, j));
                Tile {
                    t_type: TType::End,
                    walls: 0,
                }
            }
            dir if dir & DIR_MASK == dir => Tile {
                t_type: TType::Unvisited,
                walls: dir,
            },
            dir => panic!("Invalid cell! value: {}; index: {:?}", dir, (i, j)),
        };

        let grid = maze
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &dir)| process_dir(dir, (i, j)))
                    .collect_vec()
            })
            .collect_vec();

        Field {
            grid,
            begin: begin.expect("Begin cell not found!"),
            end: end.expect("End cell not found!"),
        }
    }

    fn rotate_walls(&mut self) {
        for tile in self.grid.iter_mut().flatten() {
            tile.walls = shift_dir(tile.walls, 1);
        }
    }

    pub(crate) fn get_next_points(
        &self,
        (cur_i, cur_j): (usize, usize),
    ) -> Box<impl Iterator<Item = (usize, usize)> + '_> {
        if !self.has_valid_size((cur_i as i32, cur_j as i32)) {
            panic!("Invalid size!");
        }

        let cur_walls = self.grid[cur_i as usize][cur_j as usize].walls;

        let dirs = &[
            ((1, 0), S_DIR),
            ((-1, 0), N_DIR),
            ((0, 1), E_DIR),
            ((0, -1), W_DIR),
        ];

        let iter = dirs.iter().filter_map(move |&((i, j), dir)| {
            let (i, j) = (cur_i as i32 + i, cur_j as i32 + j);
            if !self.has_valid_size((i, j)) {
                return None;
            }

            let tile_id = (i as usize, j as usize);
            let option = match self.tile(tile_id).t_type {
                _ if cur_walls & dir != 0 || shift_dir(self.tile(tile_id).walls, 2) & dir != 0 => {
                    None
                }
                TType::Unvisited | TType::End => Some(tile_id),
                _ => None,
            };
            option
        });
        Box::new(iter)
    }

    //TODO: maybe create method get tile by i, j, and use generics as parameter
    fn has_valid_size(&self, (i, j): (i32, i32)) -> bool {
        let i_max = self.grid.len() as i32;
        let j_max = self.grid[0].len() as i32;

        0 <= i && i < i_max && 0 <= j && j < j_max
    }

    fn tile(&self, tile_id: (usize, usize)) -> Tile {
        self.grid[tile_id.0][tile_id.1]
    }

    fn tile_mut(&mut self, tile_id: (usize, usize)) -> Tile {
        self.grid[tile_id.0][tile_id.1]
    }
    
    fn try_solve(&mut self) -> bool {
        let instant = Instant::now();
        let mut points_to_visit = vec![self.begin];
        'outer: for interval_id in 0.. {
            for point_id in 0.. {
                if let Some(&(i, j)) = points_to_visit.get(point_id) {
                    if (i, j) == self.end {
                        // t_type of end tile will be Visited, not End
                        break 'outer;
                    }

                    let next_points = self.get_next_points((i, j)).collect_vec();
                    for (next_i, next_j) in next_points {
                        points_to_visit.push((next_i, next_j));
                        self.grid[next_i][next_j].t_type = TType::Visited {
                            interval_id,
                            prev_tile_id: (i, j),
                        };
                    }
                } else {
                    break;
                }
            }

            if instant.elapsed().as_millis() >= 5{
                return false;
            }

            self.rotate_walls();
        }
        true
    }
    
    fn get_ans(&self) -> Vec<String>{
        let mut tile_id = self.end;
        let mut next_interval_id: Option<usize> = None;
        let mut reversed_ans: Vec<String> = vec![];
        loop {
            match self.tile(tile_id).t_type {
                TType::Visited {
                    prev_tile_id,
                    interval_id,
                } => {
                    let char = get_dir_char(prev_tile_id, tile_id);
                    match next_interval_id {
                        None => reversed_ans.push(char.to_string()),
                        Some(next_interval_id) => match next_interval_id - interval_id {
                            0 => reversed_ans.last_mut().unwrap().push(char),
                            1 => reversed_ans.push(char.to_string()),
                            delta => {
                                add_spaces(&mut reversed_ans, delta - 1);
                                reversed_ans.push(char.to_string());
                            }
                        },
                    };

                    next_interval_id.replace(interval_id);
                    tile_id = prev_tile_id;
                }
                TType::Begin => {
                    add_spaces(&mut reversed_ans, next_interval_id.unwrap());
                    break;
                }
                _ => panic!("Invalid route!"),
            }

            fn add_spaces(vec: &mut Vec<String>, count: usize) {
                vec.extend(iter::repeat("".to_string()).take(count));
            }
        }
        return reverse_ans(reversed_ans);

        fn reverse_ans(reversed_ans: Vec<String>) -> Vec<String> {
            reversed_ans
                .iter()
                .map(|string| string.chars().rev().collect::<String>())
                .rev()
                .collect_vec()
        }
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let i_max = self.grid.len();

        let grid = self
            .grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, tile)| match tile.t_type {
                        TType::Visited {
                            prev_tile_id: from_tile,
                            ..
                        } => format_walls(tile.walls, get_dir_char(from_tile, (i, j))),
                        TType::Unvisited => format_walls(tile.walls, '·'),
                        TType::Begin => format_walls(0, 'B'),
                        TType::End => format_walls(0, 'X'),
                    })
                    .collect_vec()
            })
            .collect_vec();

        for str_row_index in 0..i_max * 3 {
            let row_index = str_row_index / 3;
            let index = str_row_index % 3;
            let str = grid[row_index]
                .iter()
                .flat_map(|arr| arr[3 * index..3 * (index + 1)].iter())
                .intersperse(&' ')
                .collect::<String>();
            writeln!(f, "{}", str)?;
        }
        return writeln!(f);

        fn format_walls(walls: DIR, ch_inside: char) -> [char; 9] {
            [
                '┌',
                if walls & N_DIR != 0 { '—' } else { ' ' },
                '┐',
                if walls & W_DIR != 0 { '|' } else { ' ' },
                ch_inside,
                if walls & E_DIR != 0 { '|' } else { ' ' },
                '└',
                if walls & S_DIR != 0 { '—' } else { ' ' },
                '┘',
            ]
        }
    }
}

fn shift_dir(dir: DIR, shift: i8) -> DIR {
    let shifted = dir << (shift % 4);
    (shifted & DIR_MASK) | (shifted >> 4)
}

fn get_dir_char(from: (usize, usize), to: (usize, usize)) -> char {
    match (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32) {
        (-1, 0) => 'N',
        (1, 0) => 'S',
        (0, 1) => 'E',
        (0, -1) => 'W',
        (i, j) => panic!("Invalid delta: {:?}", (i, j)),
    }
}

pub fn maze_solver(maze: &Vec<Vec<DIR>>) -> Option<Vec<String>> {
    let mut field = Field::new(maze);
    if !field.try_solve() {
        return None;
    }

    Some(field.get_ans())
}

mod test {
    use super::*;

    #[test]
    fn get_next_points_test() {
        let maze = vec![
            vec![4, 2, 5, 4],
            vec![4, 15, 11, 1],
            vec![-1, 9, 6, 8],
            vec![12, 7, 7, -2],
        ];

        let mut field = Field::new(&maze);
        dbg!(&field);

        let data = [
            ((2_usize, 1_usize), vec![(3, 1)]),
            ((0, 3), vec![(1, 3)]),
            (field.begin, vec![(1, 0), (2, 1)]),
            ((2, 2), vec![(2, 3)]),
        ];

        for ((i, j), next_points) in data.into_iter() {
            field.grid[i][j].t_type = TType::Visited {
                prev_tile_id: (0, 0),
                interval_id: 0,
            };

            assert_eq!(field.get_next_points((i, j)).collect_vec(), next_points);
            field.grid[i][j].t_type = TType::Unvisited;
        }
    }

    #[test]
    fn shift_dir_test() {
        assert_eq!(shift_dir(N_DIR, 2), S_DIR);
    }
}
