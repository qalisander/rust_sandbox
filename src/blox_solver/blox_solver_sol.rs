// https://www.codewars.com/kata/5a2a597a8882f392020005e5/train/rust

use crate::blox_solver::blox_solver_sol::Orientation::{Horizontal, Upright, Vertical};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

type Grid = Vec<Vec<Rc<Tile>>>;

//NOTE: Solution with "from" index instead of weak reference would be prlly simpler and rust idiomatic
#[derive(Clone, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Orientation {
    Upright,
    Vertical,
    Horizontal,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let orientations = [Upright, Horizontal, Vertical]
            .iter()
            .map(|orientation| (orientation, self.grid.get(orientation).unwrap()));
        for (orientation, grid) in orientations {
            writeln!(f, "{:?}", orientation)?;
            let formatted_row = grid
                .iter()
                .flat_map(|row| row.iter().map(|tile| tile_to_char(tile)).chain(['\n']))
                .collect::<String>();
            writeln!(f, "{}", formatted_row)?;
        }
        return writeln!(f);

        fn tile_to_char(tile: &Tile) -> char {
            match tile {
                Tile::Visited { ch, .. } => *ch,
                Tile::Unvisited => '.',
                Tile::Begin => 'B',
                Tile::Space => '0',
                Tile::End => 'X',
            }
        }
    }
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
        if i < 0 || i >= i_max || j < 0 || j >= j_max {
            return false;
        }

        let (i, j) = state.usize_index();
        matches!(*grid[i][j], Tile::Unvisited | Tile::End)
    }

    fn new(puzzle: &[&str]) -> Self {
        let begin = index_of(puzzle, 'B');
        let end = index_of(puzzle, 'X');
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
            (Upright, upright_field),
            (Horizontal, horizontal_field),
            (Vertical, vertical_field),
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
                        .map(Rc::new)
                        .collect_vec()
                })
                .collect_vec()
        }
    }
}

#[derive(Copy, Clone, Debug)]
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
            Upright => [
                (Horizontal, (0, 1), 'R'),
                (Horizontal, (0, -2), 'L'),
                (Vertical, (1, 0), 'D'),
                (Vertical, (-2, 0), 'U'),
            ],
            Vertical => [
                (Vertical, (0, 1), 'R'),
                (Vertical, (0, -1), 'L'),
                (Upright, (2, 0), 'D'),
                (Upright, (-1, 0), 'U'),
            ],
            Horizontal => [
                (Upright, (0, 2), 'R'),
                (Upright, (0, -1), 'L'),
                (Horizontal, (1, 0), 'D'),
                (Horizontal, (-1, 0), 'U'),
            ],
        }
        .into_iter()
    }
}

pub(super) fn blox_solver(puzzle: &[&str]) -> String {
    let mut field = Field::new(puzzle);
    let begin_state = State {
        index: (field.begin.0 as isize, field.begin.1 as isize),
        orientation: Upright,
        ch: '*',
    };

    let mut deque = VecDeque::from([begin_state]);

    while let Some(popped_state) = deque.pop_front() {
        //        dbg!(&field);
        //        dbg!(&deque);

        let new_states = popped_state
            .get_dirs()
            .map(|(orientation, delta, ch)| State {
                orientation,
                index: (
                    popped_state.index.0 + delta.0,
                    popped_state.index.1 + delta.1,
                ),
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
                from: Rc::downgrade(tile),
            };
            field.grid_mut(new_state.orientation)[visited_i][visited_j] = Rc::new(visited_tile);
        }

        deque.extend(new_states);
    }

    let mut ans = String::new();
    let mut tile = field.grid(Upright)[field.end.0][field.end.1].clone();

    while let Tile::Visited { ch, from } = tile.deref() {
        ans.push(*ch);
        tile = from.upgrade().unwrap();
    }

    ans.chars().rev().collect()
}

//NOTE: smb's short solution
//mod blox {
//    use std::collections::VecDeque;
//    use bit_set::BitSet;
//
//    pub fn blox_solver(puzzle: &[&str]) -> String {
//        let sizey = puzzle.len() as i32;
//        let sizex = puzzle[0].len() as i32;
//        let mut visited = BitSet::with_capacity((sizey * sizex * 3) as usize);
//        let get = |x: i32, y: i32| puzzle[y as usize].as_bytes()[x as usize];
//        let mut next = VecDeque::new();
//
//        for y in 0..sizey {
//            for x in 0..sizex {
//                if get(x, y) == b'B' {
//                    next.push_back((x, y, 0, "".to_string()));
//                }
//            }
//        }
//
//        // 0: #          2: #
//        //       1: ##      #
//        while let Some((x, y, rot, path)) = next.pop_front() {
//            // Check if not ouside
//            if x < 0 || y < 0 || x + (rot == 1) as i32 >= sizex ||
//                y + (rot == 2) as i32 >= sizey {
//                continue
//            }
//
//            // Check if not visited
//            let visited_pos = ((y * sizex + x) * 3 + rot) as usize;
//            if visited.contains(visited_pos) {
//                continue
//            }
//
//            // Check if found the exit
//            if rot == 0 && get(x, y) == b'X' {
//                return path
//            }
//
//            // Check if not above air
//            if get(x, y) == b'0' ||
//                (rot == 1 && get(x + 1, y) == b'0') ||
//                (rot == 2 && get(x, y + 1) == b'0') {
//                continue
//            }
//
//            visited.insert(visited_pos);
//
//            match rot {
//                0 => {
//                    next.push_back((x-2, y, 1, format!("{}L", path)));
//                    next.push_back((x+1, y, 1, format!("{}R", path)));
//                    next.push_back((x, y-2, 2, format!("{}U", path)));
//                    next.push_back((x, y+1, 2, format!("{}D", path)));
//                }
//                1 => {
//                    next.push_back((x-1, y, 0, format!("{}L", path)));
//                    next.push_back((x+2, y, 0, format!("{}R", path)));
//                    next.push_back((x, y-1, 1, format!("{}U", path)));
//                    next.push_back((x, y+1, 1, format!("{}D", path)));
//                }
//                2 => {
//                    next.push_back((x-1, y, 2, format!("{}L", path)));
//                    next.push_back((x+1, y, 2, format!("{}R", path)));
//                    next.push_back((x, y-1, 0, format!("{}U", path)));
//                    next.push_back((x, y+2, 0, format!("{}D", path)));
//                }
//                _ => unreachable!(),
//            }
//        }
//        unreachable!()
//    }
//}
