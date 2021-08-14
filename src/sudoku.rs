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

        return true;
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
