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

        for i in 0..self.sqrt_len() {
            for j in 0..self.sqrt_len() {
                if !seq_is_valid(self.square_seq(i * self.sqrt_len(), j * self.sqrt_len())) {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_len_invalid(&self) -> bool {
        self.data
            .iter()
            .map(|arr| arr.len())
            .any(|len| len != self.data.len())
    }

    pub fn horizontal_seq(&self, i: usize) -> Vec<u32> {
        self.data[i].to_vec()
    }

    pub fn vertical_seq(&self, j: usize) -> Vec<u32> {
        (0..self.data.len()).map(|i| self.data[i][j]).collect()
    }

    pub fn square_seq(&self, i_origin: usize, j_origin: usize) -> Vec<u32> {
        let mut vec: Vec<u32> = vec![];
        for i in i_origin..i_origin + self.sqrt_len() {
            for j in j_origin..j_origin + self.sqrt_len() {
                vec.push(self.data[i][j]);
            }
        }
        return vec;
    }

    pub fn sqrt_len(&self) -> usize {
        (self.data.len() as f32).sqrt() as usize
    }
}

pub fn seq_is_valid(seq: Vec<u32>) -> bool {
    let mut temp = seq.to_vec();
    temp.sort();

    for i in 0..seq.len() {
        if temp[i] != (i + 1) as u32 {
            return false;
        }
    }

    return true;
}
