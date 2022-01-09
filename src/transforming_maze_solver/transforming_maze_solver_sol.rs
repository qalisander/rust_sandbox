// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use crate::transforming_maze_solver::transforming_maze_solver_sol::Tile::Unvisited;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

type DIR = i8;

const DIR_MASK: DIR = 0b00001111;
const E_DIR: DIR = 0b_0001;
const S_DIR: DIR = 0b_0010;
const W_DIR: DIR = 0b_0100;
const N_DIR: DIR = 0b_1000;
const BEGIN: DIR = -1;
const END: DIR = -2;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Visited {
        prev_tile_delta: (i8, i8),
        prev_grid_index: i8,
        walls: DIR,
    },
    Unvisited {
        walls: DIR,
    },
    Begin,
    End,
}

type Grid = Vec<Vec<Tile>>;
struct Field {
    grids: [Grid; 4],
    begin: (i8, i8),
    end: (i8, i8),
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (index, grid) in self.grids.iter().enumerate() {
            writeln!(f, "Grid: {}", index)?;
            let i_max = grid.len();
            let j_max = grid[0].len();

            let grid = grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match *tile {
                            Tile::Visited {
                                walls,
                                prev_tile_delta,
                                ..
                            } => format_walls(walls, get_dir_char(prev_tile_delta)),
                            Tile::Unvisited { walls } => format_walls(walls, '·'),
                            Tile::Begin => format_walls(0, 'B'),
                            Tile::End => format_walls(0, 'X'),
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

fn get_dir_char(delta: (i8, i8)) -> char {
    match delta {
        (i, _) if i < 0 => 'N',
        (i, _) if i > 0 => 'S',
        (_, j) if j < 0 => 'E',
        (_, j) if j < 0 => 'W',
        _ => panic!("Invalid delta: {:?}", delta),
    }
}

impl Field {
    fn new(maze: &Vec<Vec<DIR>>) -> Self {
        let mut begin = None;
        let mut end = None;
        let mut process_dir = |dir, (i, j)| match dir {
            BEGIN => {
                begin = Some((i as i8, j as i8));
                Tile::Begin
            }
            END => {
                end = Some((i as i8, j as i8));
                Tile::End
            }
            dir if dir & DIR_MASK == dir => Tile::Unvisited { walls: dir },
            dir => panic!("Invalid cell! value: {}; index: {:?}", dir, (i, j)),
        };

        let grid = (0..4)
            .map(|shift| {
                maze.iter()
                    .enumerate()
                    .map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .map(|(j, &dir)| {
                                let dir = if matches!(dir, END | BEGIN) {
                                    dir
                                } else {
                                    shift_dir(dir, shift)
                                };
                                process_dir(dir, (i, j))
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Field {
            grids: grid,
            begin: begin.expect("Begin cell not found!"),
            end: end.expect("End cell not found!"),
        }
    }
}

pub fn maze_solver(maze: &Vec<Vec<DIR>>) -> Option<Vec<String>> {
    let field = Field::new(maze);
    dbg!(&field);
    //TODO:
    // create data structure that contains tiles and references on previous directions
    // move to every possible state -> do next turn
    // move back from end tile to beginning

    None
}

pub fn shift_dir(dir: DIR, shift: i8) -> DIR {
    let shifted = dir << (shift % 4);
    (shifted & DIR_MASK) | (shifted >> 4)
}
