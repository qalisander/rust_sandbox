// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use crate::sudoku::sudoku_my::seq_is_valid;
use itertools::Itertools;
use std::collections::vec_deque::VecDeque;
use std::fmt::{Debug, Formatter};

type DIR = i8;

const DIR_MASK: DIR = 0b00001111;
const E_DIR: DIR = 0b_0001;
const S_DIR: DIR = 0b_0010;
const W_DIR: DIR = 0b_0100;
const N_DIR: DIR = 0b_1000;
const BEGIN: DIR = -1;
const END: DIR = -2;

//TODO: better make tile as a struct and tile type
#[derive(Debug, Copy, Clone)]
enum TType {
    Visited {
        prev_tile_delta: (i8, i8),
        is_prev_same_interval: bool,
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
        if !self.has_valid_size((cur_i as i8, cur_j as i8)) {
            panic!("Invalid size!");
        }

        let cur_walls = self.grid[cur_i as usize][cur_j as usize].walls;

        let dirs = &[
            ((1_i8, 0_i8), S_DIR),
            ((-1, 0), N_DIR),
            ((0, 1), E_DIR),
            ((0, -1), W_DIR),
        ];

        let iter = dirs.iter().filter_map(move |&((i, j), dir)| {
            let (i, j) = (cur_i as i8 + i, cur_j as i8 + j);
            if !self.has_valid_size((i, j)) {
                return None;
            }

            let (i, j) = (i as usize, j as usize);
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

    fn has_valid_size(&self, (i, j): (i8, i8)) -> bool {
        let i_max = self.grid.len() as i8;
        let j_max = self.grid[0].len() as i8;

        0 <= i && i < i_max && 0 <= j && j < j_max
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let i_max = self.grid.len();

        let grid = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile.t_type {
                        TType::Visited {
                            prev_tile_delta, ..
                        } => format_walls(tile.walls, get_dir_char(prev_tile_delta)),
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

// TODO: use this fn when go back
fn get_dir_char(delta: (i8, i8)) -> char {
    match delta {
        (i, _) if i < 0 => 'N',
        (i, _) if i > 0 => 'S',
        (_, j) if j < 0 => 'E',
        (_, j) if j < 0 => 'W',
        _ => panic!("Invalid delta: {:?}", delta),
    }
}

struct Move {
    to: (usize, usize),
    from: Option<(usize, usize)>,
}

pub fn maze_solver(maze: &Vec<Vec<DIR>>) -> Option<Vec<String>> {
    let field = Field::new(maze);
    dbg!(&field);

    let init_move = Move {
        to: field.begin,
        from: None,
    };
    let mut moves_queue = VecDeque::from([init_move]);
    while let Some(state) = moves_queue.pop_front() {}
    // TODO: move back from end tile to beginning
    None
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
            (field.begin, vec![(1, 0), (2, 1)])
        ];

        for ((i, j), next_points) in data.into_iter() {
            field.grid[i][j].t_type = TType::Visited {
                prev_tile_delta: (0, 0),
                is_prev_same_interval: false,
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
