use itertools::{self, sorted};

pub struct Sudoku {
    pub data: Vec<Vec<u32>>,
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        if self.is_len_invalid() {
            return false;
        }

        for i in 0..self.data.len() {
            if !seq_is_valid(self.horizontal_seq(i)) || !seq_is_valid(self.vertical_seq(i)) {
                return false;
            }
        }

        for i in (0..self.data.len()).step_by(self.sqrt_len()) {
            for j in (0..self.data.len()).step_by(self.sqrt_len()) {
                if !seq_is_valid(self.square_seq(i, j)) {
                    return false;
                }
            }
        }

        true
    }

    fn is_len_invalid(&self) -> bool {
        let mut lens = self.data.iter().map(|arr| arr.len());
        lens.any(|len| len != self.data.len())
    }

    fn horizontal_seq(&self, i: usize) -> Vec<u32> {
        self.data[i].to_vec()
    }

    fn vertical_seq(&self, j: usize) -> Vec<u32> {
        (0..self.data.len()).map(|i| self.data[i][j]).collect()
    }
    fn square_seq(&self, i_origin: usize, j_origin: usize) -> Vec<u32> {
        (i_origin..i_origin + self.sqrt_len())
            .flat_map(|i| self.data[i][j_origin..j_origin + self.sqrt_len()].to_vec())
            .collect()
    }

    fn sqrt_len(&self) -> usize {
        (self.data.len() as f32).sqrt() as usize
    }
}

pub fn seq_is_valid(seq: Vec<u32>) -> bool {
    (1..=seq.len() as u32).eq(sorted(seq))
}


// NOTE: smb's pretty solutions
//https://www.codewars.com/kata/540afbe2dc9f615d5e000425/solutions/rust/all/best_practice

//pub struct Sudoku {
//    pub data: Vec<Vec<u32>>,
//}
//
//impl Sudoku {
//    pub fn is_valid(&self) -> bool {
//        fn check_range<'a, I: Iterator<Item = &'a u32>>(n: usize, i: I) -> bool {
//            let mut vals: Vec<u32> = i.map(|x| *x).collect();
//            vals.sort();
//            (1..n as u32 + 1).collect::<Vec<_>>() == vals
//        }
//
//        let dim = self.data.len();
//        let n = (dim as f32).sqrt() as usize;
//        // Check dimension
//        if self.data.iter().map(|row| row.len()).any(|l| l != dim) {
//            return false;
//        }
//        // Check row
//        for r in self.data.iter() {
//            if !check_range(dim, r.iter()) {
//                return false;
//            }
//        }
//        // Check column
//        for i in 0..dim {
//            if !check_range(dim, (0..dim).map(|j| &self.data[j][i])) {
//                return false;
//            }
//        }
//        // Check cubes
//        for sx in 0..n {
//            for sy in 0..n {
//                let mut cube = Vec::new();
//                for x in 0..n {
//                    for y in 0..n {
//                        cube.push(self.data[sx * n + x][sy * n + y]);
//                    }
//                }
//                if !check_range(dim, cube.iter()) {
//                    return false;
//                }
//            }
//        }
//        true
//    }
//}

//struct Sudoku {
//    data: Vec<Vec<u32>>,
//}
//
//impl Sudoku {
//    fn is_valid(&self) -> bool {
//        use std::collections::HashSet;
//        let s = self.data.len(); // size
//        let bs = (s as f32).sqrt() as usize; // block size
//        let tgt: HashSet<u32> = (1..=s as u32).collect(); // comparison target
//
//        // check rows
//        self.data
//            .iter()
//            .all(|r| r.iter().cloned().collect::<HashSet<_>>() == tgt)
//            // check columns
//            && (0..s).all(|c| (0..s).map(|r| self.data[r][c]).collect::<HashSet<_>>() == tgt)
//            // check blocks
//            && (0..bs).all(|r| {
//            (0..bs).all(|c| {
//                (0..bs)
//                    .flat_map(|br| (0..bs).map(move |bc| self.data[r * bs + br][c * bs + bc]))
//                    .collect::<HashSet<_>>()
//                    == tgt
//            })
//        })
//    }
//}

