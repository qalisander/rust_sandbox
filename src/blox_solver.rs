// https://www.codewars.com/kata/5a2a597a8882f392020005e5/train/rust

use itertools::Itertools;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

type Grid = Vec<Vec<Option<i32>>>;// TODO: use another enum and store here previous index

// TODO: try use here reference counter
struct Tile { // TODO: turn into enum
    orientation: Orientation, //TODO: we don't need orientation here
    len: i32, //TODO: we don't need to store len here
    previous: Weak<Tile>,
}

enum Tile2{
    Visited(Weak<Tile>, char),
    Unvisited,
    Begin,
    Space,
}

struct Field {
    upright: Grid,
    vertical: Grid,
    horizontal: Grid,
    begin: (isize, isize),
    end: (isize, isize),
}

#[derive(Copy, Clone)]
struct State {
    index: (isize, isize),
    orientation: Orientation,
    len: i32, //TODO: add chars here
}

impl State {
    fn usize_index(&self) -> (usize, usize) {
        (self.index.0 as usize, self.index.1 as usize)
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    Upright,
    Vertical,
    Horizontal,
}

fn is_state_available(grid: &Grid, state: State) -> bool {
    let i_max = grid.len() as isize;
    let j_max = grid[0].len() as isize;

    let (i, j) = state.index;
    if i < 0 || j >= i_max || j < 0 || j >= j_max {
        return false;
    }

    let (i, j) = state.usize_index();
    matches!(grid[i][j], Some(-1)) // TODO: use len in state
}

pub fn blox_solver(puzzle: &[&str]) -> String {
    let mut field = create_filed(puzzle);

    //TODO:
    // - check current state
    // - try move to another state according to current state
    // - while queue is not empty

    let begin_state = State {
        index: field.begin,
        orientation: Orientation::Upright,
        len: 0,
    };
    let mut deque = VecDeque::from([begin_state]);

    while let Some(state) = deque.pop_front() {
        match state.orientation {
            Orientation::Upright if is_state_available(&field.upright, state) => {
                let (i, j) = state.usize_index();
                field.upright[i][j] = Some(state.len);

                let dirs = [
                    (Orientation::Horizontal, (0, 1)),
                    (Orientation::Horizontal, (0, -2)),
                    (Orientation::Vertical, (1, 0)),
                    (Orientation::Vertical, (-2, 0)),
                ];

                //TODO: use as a separate method
                let next_states = dirs.map(|(orientation, delta)| State {
                    orientation,
                    index: (state.index.0 + delta.0, state.index.1 + delta.1),
                    len: state.len + 1,
                });

                deque.extend(next_states);
            }
            Orientation::Vertical => {}
            Orientation::Horizontal => {}
            _ => (),
        }
    }

    // TODO: find shortest way by going back

    todo!("your task should you choose to accept it");
}

// TODO: move to state struct impl
fn as_usize(index: (isize, isize)) -> (usize, usize) {
    (index.0 as usize, index.1 as usize)
}

fn create_filed(puzzle: &[&str]) -> Field {
    let begin = index_of(puzzle, 'B');
    let end = index_of(puzzle, 'B');
    let mut upright_field = puzzle
        .iter()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '0' | 'B' | 'X' => None,
                    '1' => Some(-1),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let i_max = upright_field.len();
    let j_max = upright_field[0].len();

    let mut vertical_field = upright_field.clone();
    let mut horizontal_field = upright_field.clone();
    for i in 0..i_max {
        for j in 0..j_max {
            let next_j = j + 1;
            if next_j >= j_max || upright_field[i][next_j].is_none() {
                horizontal_field[i][j] = None;
            }

            let next_i = i + 1;
            if next_i >= i_max || upright_field[next_i][j].is_none() {
                vertical_field[i][j] = None;
            }
        }
    }

    upright_field[begin.0][begin.1] = Some(0);
    upright_field[end.0][end.1] = Some(-1);

    return Field {
        upright: upright_field,
        vertical: vertical_field,
        horizontal: horizontal_field,
        begin: (begin.0 as isize, begin.1 as isize),
        end: (end.0 as isize, end.1 as isize),
    };

    fn index_of(puzzle: &[&str], char: char) -> (usize, usize) {
        puzzle
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.chars().enumerate().map(move |(j, ch)| ((i, j), ch)))
            .find(|(_, ch)| *ch == char)
            .unwrap()
            .0
    }
}
