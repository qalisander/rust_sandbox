#[cfg(test)]
mod example_tests {
    use super::*;
    use crate::transforming_maze_solver::transforming_maze_solver_sol::maze_solver;
    use itertools::Itertools;

    #[test]
    fn example_tests() {
        let example_tests = vec![
                (
                    vec![
                        vec![4, 2, 5, 4],
                        vec![4, 15, 11, 1],
                        vec![-1, 9, 6, 8],
                        vec![12, 7, 7, -2],
                    ],
                    Some(vec!["NNE", "EE", "S", "SS"]),
                ),
                (
                    vec![
                        vec![6, 3, 10, 4, 11],
                        vec![8, 10, 4, 8, 5],
                        vec![-1, 14, 11, 3, -2],
                        vec![15, 3, 4, 14, 15],
                        vec![14, 7, 15, 5, 5],
                    ],
                    Some(vec!["", "", "E", "", "E", "NESE"]),
                ),
                (
                    vec![
                        vec![9, 1, 9, 0, 13, 0],
                        vec![14, 1, 11, 2, 11, 4],
                        vec![-1, 2, 11, 0, 0, 15],
                        vec![4, 3, 9, 6, 3, -2],
                    ],
                    Some(vec!["E", "SE", "", "E", "E", "E"]),
                ),
                (
                    vec![
                        vec![-1, 6, 12, 15, 11],
                        vec![8, 7, 15, 7, 10],
                        vec![13, 7, 13, 15, -2],
                        vec![11, 10, 8, 1, 3],
                        vec![12, 6, 9, 14, 7],
                    ],
                    None,
                ),
                (
                    vec![
                        vec![4, 14, 14, 11, 4, -2],
                        vec![13, 1, 12, 1, 12, 13],
                        vec![14, 13, 1, 1, 0, 10],
                        vec![-1, 13, 2, 5, 13, 12],
                        vec![5, 2, 6, 1, 11, 10],
                    ],
                    Some(vec!["", "E", "", "ENEE", "N", "", "N", "E"]),
                ),
            ];

        example_tests.iter().for_each(|(maze, sol)| {
            let refsol = sol
                .as_ref()
                .map(|r| r.iter().map(|&s| String::from(s)).collect());
            test_helper::run_test(maze, maze_solver(maze), refsol)
        });
    }
}

#[cfg(test)]
mod test_helper {
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn run_test(r: &Vec<Vec<i8>>, _user: Option<Vec<String>>, _refsol: Option<Vec<String>>) {
        if let Some(user) = _user {
            let refsol = _refsol.unwrap();

            if user.join("").chars().any(|ch| !"WENS".contains(ch)) {
                return assert!(
                    false,
                    "Solution elements must only consist of the following characters: \"NWSE\""
                );
            }
            let ref_str = format!("Here is a valid solution:\n{}", sol_str(&refsol));
            let user_str = format!("Here is your solution:\n{}", sol_str(&user));
            if user.len() > refsol.len() {
                return assert!(false,"Your solution completes the task in {} iterations.\nThis test can be completed in {} iterations.\n{}\n{}",user.len(),refsol.len(),ref_str,user_str);
            }
            let dir_map: HashMap<u8, (i8, i8)> =
                vec![(78, (-1, 0)), (87, (0, -1)), (83, (1, 0)), (69, (0, 1))]
                    .into_iter()
                    .collect();
            let dnum: HashMap<u8, usize> = vec![(78, 0), (87, 1), (83, 2), (69, 3)]
                .into_iter()
                .collect();
            let dword = ["north", "west", "south", "east"];
            let grid: Vec<Vec<u8>> = r
                .iter()
                .map(|row| row.iter().map(|&n| n.max(0) as u8).collect())
                .collect();
            let xl = r.len();
            let yl = r[0].len();
            let (mut px, mut py): (usize, usize) = (0, 0);
            let mut dst: (usize, usize) = (0, 0);
            for (i, row) in r.iter().enumerate() {
                for (j, cel) in row.iter().enumerate() {
                    if *cel < 0 {
                        if *cel == -1 {
                            px = i;
                            py = j;
                        } else {
                            dst = (i, j);
                        }
                    }
                }
            }

            let bad_move = |s: String| assert!(false, "Invalid move: {}\n{}", s, user_str);

            for (i, s) in user.iter().enumerate() {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                for (j, b) in s.bytes().enumerate() {
                    let dq: usize = dnum[&b];
                    let (nx, ny): (i8, i8) = dir_map[&b];
                    let pos_str = format!(
                        "during move {} at iteration {}.\nLast valid position was [{}, {}].",
                        j, i, px, py
                    );
                    let _qx = nx + px as i8;
                    let _qy = ny + py as i8;
                    if _qx < 0 || _qx >= xl as i8 || _qy < 0 || _qy >= yl as i8 {
                        return bad_move(format!("Out of bounds {}", pos_str));
                    }
                    let qx = _qx as usize;
                    let qy = _qy as usize;
                    let obstructs: Vec<(usize, u8)> =
                        wall_check(grid[px][py], grid[qx][qy], dq, i % 4);
                    if !obstructs.is_empty() {
                        let (celln, d) = obstructs[0];
                        let (xx, yy) = if d == 0 { (px, py) } else { (qx, qy) };
                        return bad_move(format!(
                            "Path obstructed by a wall on the {} side of [{}, {}] {}",
                            dword[celln], xx, yy, pos_str
                        ));
                    }
                    if visited.contains(&(qx, qy)) {
                        return bad_move(format!("Entered cell [{}, {}] a second time", qx, qy));
                    }
                    px = qx;
                    py = qy;
                    visited.insert((qx, qy));
                }
            }

            if dst != (px, py) {
                return assert!(
                    false,
                    "The ball did not reach the destination. Its last position was [{}, {}]\n{}",
                    px, py, user_str
                );
            }

            return assert!(true);
        } else {
            return assert!(_refsol.is_none(), "This puzzle has no solution");
        }
    }

    fn sol_str(r: &Vec<String>) -> String {
        format!("[ \"{}\" ]", r.join("\", \""))
    }
    fn celrot(n: u8) -> u8 {
        let x = n << 1;
        x / 16 + x % 16
    }
    fn get_celval(n: u8, c: usize) -> usize {
        (0..c).into_iter().fold(n, |z, _| celrot(z)) as usize
    }

    fn wall_check(fro: u8, too: u8, d: usize, c: usize) -> Vec<(usize, u8)> {
        let wall_fro = get_celval(fro, c) & (8 >> d) == 0;
        let wall_too = get_celval(too, c) & (8 >> (d + 2) % 4) == 0;
        if wall_fro && wall_too {
            vec![]
        } else if !wall_fro {
            vec![(d, 0)]
        } else {
            vec![((d + 2) % 4, 1)]
        }
    }
}
