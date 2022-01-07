// https://www.codewars.com/kata/5b86a6d7a4dcc13cd900000b/train/rust

use crate::transforming_maze_solver::transforming_maze_solver_sol::Tile::Unvisited;
use itertools::Itertools;

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
        prev_tile_index: (i8, i8),
        prev_grid_index: i8,
    },
    Unvisited {
        walls: DIR,
    },
    Begin,
    End,
}

type Grid = Vec<Vec<Tile>>;
struct Field {
    grid: [Grid; 4],
    begin: (i8, i8),
    end: (i8, i8),
}

impl Field {
    fn new(maze: &Vec<Vec<DIR>>) -> Self {
        let mut begin = None;
        let mut end = None;

        let grid = (0..4)
            .map(|shift| {
                maze.iter()
                    .enumerate()
                    .map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .map(|(j, dir)| match *dir {
                                BEGIN => {
                                    begin = Some((i as i8, j as i8));
                                    Tile::Begin
                                }
                                END => {
                                    end = Some((i as i8, j as i8));
                                    Tile::End
                                }
                                dir if dir & DIR_MASK == dir => Tile::Unvisited { walls: dir },
                                dir => panic!("Invalid cell value: {}", dir),
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Field{
            grid,
            begin: begin.expect("Begin cell not found!"),
            end: end.expect("End cell not found!"),
        }
    }
}

pub fn maze_solver(maze: &Vec<Vec<DIR>>) -> Option<Vec<String>> {
    //TODO:
    // create data structure that contains tiles and references on previous directions
    // move to every possible state -> do next turn
    // move back from end tile to beginning

    None
}

pub fn circular_bit_shift(dir: DIR, shift: i8) -> DIR {
    let shifted = dir << shift;
    if shifted & DIR_MASK == shifted {
        shifted
    } else {
        (shifted & DIR_MASK) | 0b00000001
    }
}
