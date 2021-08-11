use itertools::Itertools;

struct Tile {
    ch: char,
    x: usize,
    y: usize,
}

impl Tile {
    fn can_move_to(&self, tile: Tile) -> bool {
        unimplemented!()
    }
    
    fn can_move_to_dir(ch: char, (dx, dy) : (isize, isize)) -> bool {
        //      | 
        //      |          
        // -----|----->    
        //      |     (x)  
        //      V (y)              
        match ch {
            '┃' | '┣' => dx == 0 || dy.abs() == 1,             
            '━' | '┳' | '┻' => dx.abs() == 1 || dy == 0,                   
            '┛' => (-1, 0) == (dx, dy) || (0, -1) == (dx, dy), 
            '┗' => (1, 0) == (dx, dy) || (0, -1) == (dx, dy),  
            '┓' => (-1, 0) == (dx, dy) || (0, 1) == (dx, dy),  
            '┏' => (1, 0) == (dx, dy) || (0, 1) == (dx, dy),
            // '┳' => can_move_to('━', (dx, dy)),
            // '┻' => ,
            // '┫' => ,
            // '┣' => ,
            // '╋' => ,
            _ => unimplemented!(),
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
