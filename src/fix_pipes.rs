use itertools::Itertools;

struct Tile {
    ch: char,
    x: isize,
    y: isize,
}

impl Tile {
    pub fn can_move_to(&self, tile: Tile) -> bool {
        self.can_move_to_dir(tile.x - self.x, tile.y - self.y)
    }
    
    fn can_move_to_dir(&self, dx: isize, dy: isize) -> bool {      
        match self.ch {
            '┃' | '┣' | '┫' | '╋' if dx == 0 || dy.abs() == 1 => true,       //      |      
            '━' | '┳' | '┻' | '╋' if dx.abs() == 1 || dy == 0 => true,       //      |                     
            '┛' | '┫' if (-1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true, // -----|----->    
            '┗' | '┻' if (1, 0) == (dx, dy) || (0, -1) == (dx, dy) => true,  //      |     (x)  
            '┓' | '┳' if (-1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,  //      V (y)              
            '┏' | '┣' if (1, 0) == (dx, dy) || (0, 1) == (dx, dy) => true,
            _ => false,
        }
    }
}

fn check_pipe(pipe_map: &[&str]) -> bool {
    todo!()
}

fn get_dirs(ch: char) -> Vec<(isize, isize)> {
    fn merge_dirs(ch_1: char, ch_2: char) -> Vec<(isize, isize)>{
        get_dirs(ch_1).into_iter().chain(get_dirs(ch_2)).unique().collect_vec()
    }
    
    match ch {
        '┃' => vec![(1, 0), (-1, 0)],   //      | 
        '━' => vec![(0, 1), (0, -1)],   //      |          
        '┛' => vec![(-1, 0), (0, -1)],  // -----|----->    
        '┗' => vec![(-1, 0), (0, 1)],   //      |     (j)  
        '┓' => vec![(1, 0), (0, -1)],   //      V (i)      
        '┏' => vec![(1, 0), (0, 1)],    
        '┳' => merge_dirs('┏', '━'),
        '┻' => merge_dirs('┗', '━'),
        '┫' => merge_dirs('┛', '┃'),
        '┣' => merge_dirs('┗', '┃'),
        '╋' => merge_dirs('┃', '━'),
        _ => vec![],
    }
}

#[cfg(test)]
mod sample_tests {

    #[test]
    fn small_fixed_tests() {
        for (pmap, answer) in &TEST_CASES {
            super::run_test(pmap, *answer);
        }
    }
    


    const TEST_CASES: [([&str; 3], bool); 7] = [
        (["╋━━┓", "┃..┃", "┛..┣"], true),
        (["...┏", "┃..┃", "┛..┣"], false),
        (["...┏", "...┃", "┛..┣"], false),
        (["...┏", "...┃", "┓..┣"], true),
        (["╋", "╋", "╋"], true),
        (["╋....", "┃..┛.", "┃...."], false),
        (["....", ".┛┛.", "...."], true),
    ];
}

fn run_test(pmap: &[&str], answer: bool) {
    let test_result = check_pipe(pmap);
    assert!(
        test_result == answer,
        "Output: {}; expected value: {}; for input:\n{}\n",
        test_result,
        answer,
        pmap.join("\n")
    );
}
