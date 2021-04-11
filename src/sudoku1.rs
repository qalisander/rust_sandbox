//https://www.codewars.com/kata/540afbe2dc9f615d5e000425/solutions/rust/all/best_practice

pub struct Sudoku {
    pub data: Vec<Vec<u32>>,
}

impl Sudoku {
    pub fn is_valid(&self) -> bool {
        fn check_range<'a, I: Iterator<Item=&'a u32>>(n: usize, i: I) -> bool {
            let mut vals: Vec<u32> = i.map(|x| *x).collect();
            vals.sort();
            (1..n as u32 + 1).collect::<Vec<_>>() == vals
        }

        let dim = self.data.len();
        let n = (dim as f32).sqrt() as usize;
        // Check dimension
        if self.data.iter().map(|row| row.len()).any(|l| l != dim) { return false; }
        // Check row
        for r in self.data.iter() {
            if !check_range(dim, r.iter()) { return false; }
        }
        // Check column
        for i in 0..dim {
            if !check_range(dim, (0..dim).map(|j| &self.data[j][i])) { return false; }
        }
        // Check cubes
        for sx in 0..n {
            for sy in 0..n {
                let mut cube = Vec::new();
                for x in 0..n {
                    for y in 0..n {
                        cube.push(self.data[sx * n + x][sy * n + y]);
                    }
                }
                if !check_range(dim, cube.iter()) { return false; }
            }
        }
        true
    }
}