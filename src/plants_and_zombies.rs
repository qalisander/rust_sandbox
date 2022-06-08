//https://www.codewars.com/kata/5a5db0f580eba84589000979/train/rust
mod pnz {
    use crate::plants_and_zombies::pnz::Tile::Empty;
    use itertools::Itertools;

    #[derive(Copy, Clone, Debug)]
    enum Tile {
        Shooter { power: usize },
        SShooter,
        Zombie { hp: usize },
        Empty,
    }

    #[derive(Debug)]
    struct Field {
        tiles: Vec<Vec<Tile>>,
        zombies_outside: Vec<(usize, usize, usize)>,
        turn: usize,
    }

    enum GameStatus {
        ZombiesWon,
        ZombiesLost,
        Unknown,
    }

    impl Field {
        pub fn new(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> Self {
            let tiles = lawn
                .iter()
                .map(|str| {
                    str.chars()
                        .map(|ch| match ch {
                            'S' => Tile::SShooter,
                            ' ' => Empty,
                            ch => {
                                let power = ch.to_digit(10).expect("Invalid char!") as usize;
                                Tile::Shooter { power }
                            }
                        })
                        .collect_vec()
                })
                .collect_vec();
            let zombies_outside = zombies
                .iter()
                .map(|vec| {
                    vec.iter()
                        .cloned()
                        .collect_tuple::<(_, _, _)>()
                        .expect("Invalid zombie info!")
                })
                .sorted()
                .rev()
                .collect_vec();
            Self {
                tiles,
                zombies_outside,
                turn: 0,
            }
        }

        fn zombies_make_step(&mut self) -> GameStatus {
            let mut zombies_left = 0;
            for i in 0..self.tiles.len() {
                for j in 0..self.tiles[0].len() {
                    if let Tile::Zombie { .. } = self.tiles[i][j] {
                        if j == 0 {
                            return GameStatus::ZombiesWon;
                        }
                        self.tiles[i][j - 1] = self.tiles[i][j];
                        self.tiles[i][j] = Tile::Empty;
                        zombies_left += 1;
                    }
                }
            }

            while let &Some(&(i, row, hp)) = &self.zombies_outside.last() {
                if i != self.turn {
                    break;
                }
                *self.tiles[row].last_mut().unwrap() = Tile::Zombie { hp };
                self.zombies_outside.pop();
                zombies_left += 1;
            }
            
            if zombies_left == 0 {
                GameStatus::ZombiesLost
            } else {
                GameStatus::Unknown
            }
        }
        
        fn shooters_fire(&mut self){
            todo!()
        }
    }

    // game rules
    // - zombie makes a step. And if it's position intersect with a shooter. It dies.
    // - simple shooters fire straight
    // - S shooters fire straight
    // - S shooters fire diagonally
    // - zombies dies instantly and don't absorb any additional pellets
    pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
        let mut field = Field::new(lawn, zombies);
        loop {
            match field.zombies_make_step() {
                GameStatus::ZombiesWon => break field.turn + 1,
                GameStatus::ZombiesLost => break 0,
                GameStatus::Unknown => {},
            }
            field.shooters_fire();
            field.turn += 1;
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod example_tests {
    use super::*;

    #[test]
    fn example_tests() {
        let example_tests: Vec<(Vec<&str>,Vec<Vec<usize>>,usize)> = vec![
            (
                vec![
                    "2       ",
                    "  S     ",
                    "21  S   ",
                    "13      ",
                    "2 3     "],
                vec![
                    vec![0,4,28],
                    vec![1,1,6],
                    vec![2,0,10],
                    vec![2,4,15],
                    vec![3,2,16],
                    vec![3,3,13]],
                10
            ),
            (
                vec![
                    "11      ",
                    " 2S     ",
                    "11S     ",
                    "3       ",
                    "13      "],
                vec![
                    vec![0,3,16],
                    vec![2,2,15],
                    vec![2,1,16],
                    vec![4,4,30],
                    vec![4,2,12],
                    vec![5,0,14],
                    vec![7,3,16],
                    vec![7,0,13]],
                12
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         "],
                vec![
                    vec![0,0,18],
                    vec![2,3,12],
                    vec![2,5,25],
                    vec![4,2,21],
                    vec![6,1,35],
                    vec![6,4,9],
                    vec![8,0,22],
                    vec![8,1,8],
                    vec![8,2,17],
                    vec![10,3,18],
                    vec![11,0,15],
                    vec![12,4,21]],
                20
            ),
            (
                vec![
                    "12      ",
                    "2S      ",
                    "1S      ",
                    "2S      ",
                    "3       "],
                vec![
                    vec![0,0,15],
                    vec![1,1,18],
                    vec![2,2,14],
                    vec![3,3,15],
                    vec![4,4,13],
                    vec![5,0,12],
                    vec![6,1,19],
                    vec![7,2,11],
                    vec![8,3,17],
                    vec![9,4,18],
                    vec![10,0,15],
                    vec![11,4,14]],
                19
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         "],
                vec![
                    vec![0,2,16],
                    vec![1,3,19],
                    vec![2,0,18],
                    vec![4,2,21],
                    vec![6,3,20],
                    vec![7,5,17],
                    vec![8,1,21],
                    vec![8,2,11],
                    vec![9,0,10],
                    vec![11,4,23],
                    vec![12,1,15],
                    vec![13,3,22]],
                0
            )
        ];

        example_tests.into_iter().for_each(|(grid,zqueue,sol)| assert_eq!(pnz::plants_and_zombies(&grid,&zqueue),sol));
    }
}
