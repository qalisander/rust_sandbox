// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use itertools::Itertools;
use num::cast::AsPrimitive;
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
        prev_tile_id: (i32, i32),
        interval_id: u32,
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
    begin: (i32, i32),
    end: (i32, i32),
}

impl Field {
    fn new(maze: &Vec<Vec<DIR>>) -> Self {
        let mut begin = None;
        let mut end = None;
        let mut process_dir = |dir, (i, j)| match dir {
            BEGIN => {
                begin = Some((i as i32, j as i32));
                Tile {
                    t_type: TType::Begin,
                    walls: 0,
                }
            }
            END => {
                end = Some((i as i32, j as i32));
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
        (i, j): (i32, i32),
    ) -> Box<impl Iterator<Item = (i32, i32)> + '_> {
        let cur_walls = self.tile((i, j)).expect("Invalid size!").walls;

        let dirs = &[
            ((1, 0), S_DIR),
            ((-1, 0), N_DIR),
            ((0, 1), E_DIR),
            ((0, -1), W_DIR),
        ];

        let iter = dirs.iter().filter_map(move |&((di, dj), dir)| {
            let tile_id = (i + di, j + dj);
            let tile = self.tile(tile_id)?;
            match tile.t_type {
                _ if cur_walls & dir != 0 || shift_dir(tile.walls, 2) & dir != 0 => None,
                TType::Unvisited | TType::End => Some(tile_id),
                _ => None,
            }
        });
        Box::new(iter)
    }

    fn tile<T>(&self, (i, j): (T, T)) -> Option<&Tile>
        where
            T: PartialOrd + AsPrimitive<usize> + Default {
        if self.has_valid_size((i, j)) {
            Some(&self.grid[i.as_()][j.as_()])
        } else {
            None
        }
    }

    fn tile_mut<T>(&mut self, (i, j): (T, T)) -> Option<&mut Tile>
        where
            T: PartialOrd + AsPrimitive<usize> + Default {
        if self.has_valid_size((i, j)) {
            Some(&mut self.grid[i.as_()][j.as_()])
        } else {
            None
        }
    }

    fn has_valid_size<T>(&self, (i, j): (T, T)) -> bool
    where
        T: PartialOrd + AsPrimitive<usize> + Default 
    {
        !(i < T::default()
            || j < T::default()
            || self.grid.get(i.as_()).is_none()
            || self.grid[0].get(j.as_()).is_none())
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
                        self.tile_mut((next_i, next_j)).unwrap().t_type = TType::Visited {
                            interval_id,
                            prev_tile_id: (i, j),
                        };
                    }
                } else {
                    break;
                }
            }

            if instant.elapsed().as_millis() >= 5 {
                return false;
            }

            self.rotate_walls();
        }
        true
    }

    fn get_ans(&self) -> Vec<String> {
        let mut tile_id = self.end;
        let mut next_interval_id: Option<u32> = None;
        let mut reversed_ans: Vec<String> = vec![];
        loop {
            match self.tile(tile_id).unwrap().t_type {
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

            fn add_spaces(vec: &mut Vec<String>, count: u32) {
                vec.extend(iter::repeat("".to_string()).take(count as usize));
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
                    .map(|(j, tile)| {
                        match tile.t_type {
                            TType::Visited {
                                prev_tile_id: from_tile,
                                ..
                            } => {
                                let (i, j) = (i as i32, j as i32);
                                let dir_char = get_dir_char(from_tile, (i, j));
                                format_walls(tile.walls, dir_char)
                            }
                            TType::Unvisited => format_walls(tile.walls, '·'),
                            TType::Begin => format_walls(0, 'B'),
                            TType::End => format_walls(0, 'X'),
                        }
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

fn get_dir_char(from: (i32, i32), to: (i32, i32)) -> char {
    match (to.0 - from.0, to.1 - from.1) {
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
            ((2, 1), vec![(3, 1)]),
            ((0, 3), vec![(1, 3)]),
            (field.begin, vec![(1, 0), (2, 1)]),
            ((2, 2), vec![(2, 3)]),
        ];

        for (point, next_points) in data.into_iter() {
            field.tile_mut(point).unwrap().t_type = TType::Visited {
                prev_tile_id: (0, 0),
                interval_id: 0,
            };

            assert_eq!(field.get_next_points(point).collect_vec(), next_points);
            field.tile_mut(point).unwrap().t_type = TType::Unvisited;
        }
    }

    #[test]
    fn shift_dir_test() {
        assert_eq!(shift_dir(N_DIR, 2), S_DIR);
    }
}
