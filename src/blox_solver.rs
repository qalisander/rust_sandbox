// https://www.codewars.com/kata/5a2a597a8882f392020005e5/train/rust

use itertools::Itertools;
use log::set_logger_racy;
use std::collections::{HashMap, VecDeque};
use std::iter::from_fn;
use std::ops::Deref;
use std::rc::{Rc, Weak};

type Grid = Vec<Vec<Rc<Tile>>>;

#[derive(Clone)]
enum Tile {
    Visited { from: Weak<Tile>, ch: char },
    Unvisited,
    Begin,
    Space,
    End,
}

struct Field {
    grid: HashMap<Orientation, Grid>,
    begin: (usize, usize),
    end: (usize, usize),
}

impl Field {
    fn grid(&self, orientation: Orientation) -> &Grid {
        self.grid.get(&orientation).unwrap()
    }

    fn grid_mut(&mut self, orientation: Orientation) -> &mut Grid {
        self.grid.get_mut(&orientation).unwrap()
    }

    fn is_available(&self, state: State) -> bool {
        let grid = self.grid(state.orientation);

        let i_max = grid.len() as isize;
        let j_max = grid[0].len() as isize;

        let (i, j) = state.index;
        if i < 0 || j >= i_max || j < 0 || j >= j_max {
            return false;
        }

        let (i, j) = state.usize_index();
        matches!(*grid[i][j], Tile::Unvisited | Tile::End) // TODO: use len in state
    }
}

#[derive(Copy, Clone)]
struct State {
    index: (isize, isize),
    orientation: Orientation,
    ch: char,
}

impl State {
    fn usize_index(&self) -> (usize, usize) {
        (self.index.0 as usize, self.index.1 as usize)
    }

    fn get_dirs(&self) -> impl Iterator<Item = (Orientation, (isize, isize), char)> {
        match &self.orientation {
            Orientation::Upright => [
                (Orientation::Horizontal, (0, 1), 'R'),
                (Orientation::Horizontal, (0, -2), 'L'),
                (Orientation::Vertical, (1, 0), 'D'),
                (Orientation::Vertical, (-2, 0), 'U'),
            ],
            Orientation::Vertical => [
                (Orientation::Vertical, (0, 1), 'R'),
                (Orientation::Vertical, (0, -1), 'L'),
                (Orientation::Upright, (2, 0), 'D'),
                (Orientation::Upright, (-1, 0), 'U'),
            ],
            Orientation::Horizontal => [
                (Orientation::Upright, (0, 2), 'R'),
                (Orientation::Upright, (0, -1), 'L'),
                (Orientation::Horizontal, (1, 0), 'D'),
                (Orientation::Horizontal, (-1, 0), 'U'),
            ],
        }
        .into_iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Orientation {
    Upright,
    Vertical,
    Horizontal,
}

pub fn blox_solver(puzzle: &[&str]) -> String {
    let mut field = create_filed(puzzle);
    let begin_state = State {
        index: (field.begin.0 as isize, field.begin.1 as isize),
        orientation: Orientation::Upright,
        ch: '*',
    };

    let mut deque = VecDeque::from([begin_state]);

    while let Some(popped_state) = deque.pop_front() {
        let new_states = popped_state
            .get_dirs()
            .map(|(orientation, delta, ch)| State {
                orientation,
                index: (popped_state.index.0 + delta.0, popped_state.index.1 + delta.1),
                ch,
            })
            .filter(|&state| field.is_available(state))
            .collect_vec();

            for new_state in &new_states {
                let (i, j) = popped_state.usize_index();
                let tile = &field.grid(popped_state.orientation)[i][j];

                let (visited_i, visited_j) = new_state.usize_index();
                let visited_tile = Tile::Visited {
                    ch: new_state.ch,
                    from: Rc::downgrade(tile)
                };
                field.grid_mut(new_state.orientation)[visited_i][visited_j] = Rc::new(visited_tile);
            }

        deque.extend(new_states);
    }

    let mut ans = String::new();
    let mut tile = field.grid(Orientation::Upright)[field.end.0][field.end.1].clone();

    while let Tile::Visited {ch, from} = tile.deref() {
        ans.push(*ch);
        tile = from.upgrade().unwrap();
    }

    ans.chars().rev().collect()
}

fn create_filed(puzzle: &[&str]) -> Field {
    let begin = index_of(puzzle, 'B');
    let end = index_of(puzzle, 'B');
    let mut upright_field = create_field(puzzle);

    let i_max = upright_field.len();
    let j_max = upright_field[0].len();

    let mut vertical_field = create_field(puzzle);
    let mut horizontal_field = create_field(puzzle);
    for i in 0..i_max {
        for j in 0..j_max {
            let next_j = j + 1;
            if next_j >= j_max || matches!(*upright_field[i][next_j], Tile::Space) {
                horizontal_field[i][j] = Rc::new(Tile::Space);
            }

            let next_i = i + 1;
            if next_i >= i_max || matches!(*upright_field[next_i][j], Tile::Space) {
                vertical_field[i][j] = Rc::new(Tile::Space);
            }
        }
    }

    upright_field[begin.0][begin.1] = Rc::new(Tile::Begin);
    upright_field[end.0][end.1] = Rc::new(Tile::End);

    let grid = HashMap::from([
        (Orientation::Upright, upright_field),
        (Orientation::Horizontal, horizontal_field),
        (Orientation::Vertical, vertical_field),
    ]);

    return Field {
        grid,
        begin: (begin.0, begin.1),
        end: (end.0, end.1),
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

    fn create_field(puzzle: &[&str]) -> Vec<Vec<Rc<Tile>>> {
        puzzle
            .iter()
            .map(|row| {
                row.chars()
                    .map(|ch| match ch {
                        '1' | 'B' | 'X' => Tile::Unvisited,
                        '0' => Tile::Space,
                        _ => unreachable!(),
                    })
                    .map(|tile| Rc::new(tile))
                    .collect_vec()
            })
            .collect_vec()
    }
}
