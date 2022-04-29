// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::iter;

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
        prev_tile: (usize, usize),
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

            let (i, j) = (i as usize, j as usize);
            // TODO: it's possible to give reference to tile
            let tile = self.grid[i][j];
            match tile.t_type {
                TType::Unvisited => {
                    if cur_walls & dir == 0 && shift_dir(tile.walls, 2) & dir == 0 {
                        Some((i, j))
                    } else {
                        None
                    }
                }
                TType::End => Some((i, j)),
                _ => None,
            }
        });
        Box::new(iter)
    }

    //TODO: maybe create method get tile by i, j
    fn has_valid_size(&self, (i, j): (i32, i32)) -> bool {
        let i_max = self.grid.len() as i32;
        let j_max = self.grid[0].len() as i32;

        0 <= i && i < i_max && 0 <= j && j < j_max
    }

    fn tile(&mut self, tile_id: (usize, usize)) -> Tile {
        self.grid[tile_id.0][tile_id.1]
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
                            prev_tile: from_tile,
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

pub fn shift_dir(dir: DIR, shift: i8) -> DIR {
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
    dbg!(&field);

    // TODO: create method in field struct
    let mut stop_counter = 4;
    let mut points_to_visit = vec![field.begin];
    'outer: for interval_id in 0.. {
        let prev_points_len = points_to_visit.len();
        for point_id in 0.. {
            if let Some(&(i, j)) = points_to_visit.get(point_id) {
                if (i, j) == field.end {
                    // t_type of end tile will be Visited, not End
                    break 'outer;
                }

                let next_points = field.get_next_points((i, j)).collect_vec();
                for (next_i, next_j) in next_points {
                    points_to_visit.push((next_i, next_j));
                    field.grid[next_i][next_j].t_type = TType::Visited {
                        interval_id,
                        prev_tile: (i, j),
                    };
                }
            } else {
                break;
            }
        }

        if prev_points_len == points_to_visit.len() {
            stop_counter -= 1;
            if stop_counter == 0 {
                return None;
            }
        }

        field.rotate_walls();
    }

    // TODO: create method in field struct move back from end tile to beginning
    let mut tile_id = field.end;
    let mut reversed_ans: Vec<String> = vec![];
    let mut next_interval_id: Option<usize> = None;
    while let TType::Visited {
        prev_tile: prev_tile_id,
        interval_id,
    } = field.tile(tile_id).t_type
    {
        let char = get_dir_char(prev_tile_id, tile_id);
        match next_interval_id {
            None | Some(1) => reversed_ans.push(char.to_string()),
            Some(next_interval_id) => match next_interval_id - interval_id {
                0 => reversed_ans.last_mut().unwrap().push(char),
                delta => reversed_ans.extend(
                    iter::repeat("".to_string())
                        .take(delta - 1)
                        .chain([char.to_string()].into_iter()),
                ),
            },
        };
        
        next_interval_id.replace(interval_id);
        tile_id = prev_tile_id;
    }

    Some(reverse_ans(reversed_ans))
}

fn reverse_ans(reversed_ans: Vec<String>) -> Vec<String> {
    reversed_ans
        .iter()
        .map(|string| string.chars().rev().collect::<String>())
        .rev()
        .collect_vec()
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
        ];

        for ((i, j), next_points) in data.into_iter() {
            field.grid[i][j].t_type = TType::Visited {
                prev_tile: (0, 0),
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
