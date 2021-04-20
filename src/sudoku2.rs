struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        use std::collections::HashSet;
        let s = self.data.len(); // size
        let bs = (s as f32).sqrt() as usize; // block size
        let tgt: HashSet<u32> = (1..=s as u32).collect(); // comparison target

        // check rows
        self.data
            .iter()
            .all(|r| r.iter().cloned().collect::<HashSet<_>>() == tgt)
            // check columns
            && (0..s).all(|c| (0..s).map(|r| self.data[r][c]).collect::<HashSet<_>>() == tgt)
            // check blocks
            && (0..bs).all(|r| {
                (0..bs).all(|c| {
                    (0..bs)
                        .flat_map(|br| (0..bs).map(move |bc| self.data[r * bs + br][c * bs + bc]))
                        .collect::<HashSet<_>>()
                        == tgt
                })
            })
    }
}
