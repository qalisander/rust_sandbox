use itertools::{self, Itertools};
fn path_finder(maze: &str) -> bool {
    let mut v_maze = maze
        .to_string()
        .split('\n')
        .map(|str| str.chars().collect_vec())
        .collect_vec();
    return path_finder_rec(&mut v_maze, 0, 0);
    fn path_finder_rec(maze: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let end_i = maze.len() - 1;
        let end_j = maze[0].len() - 1;

        (i <= end_i && j <= end_j)
            && (maze[i][j] == '.')
            && ((i == end_i && j == end_j) || {
                maze[i][j] = 'C';
                path_finder_rec(maze, i + 1, j)
                    || path_finder_rec(maze, i, j + 1)
                    || path_finder_rec(maze, i.saturating_sub(1), j)
                    || path_finder_rec(maze, i, j.saturating_sub(1))
            })
    }
}
// recursive functions
// https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze(
            "\
            .W.\n\
            .W.\n\
            ...\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
            false,
        );
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect,
            maze
        );
    }
}
