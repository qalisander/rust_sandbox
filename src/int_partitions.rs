// use itertools::Itertools;

// // https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b/train/rust

// fn part(n: i64) -> String {
//     // TODO: use hash map instead of that kind of array, Option<vec> - brrrrr
//     let ans = part_rec(n as usize, vec![None; n as usize + 1]).collect_vec();
//     return format!(
//         "Range: {:.2} Average: {:.2} Median: {:.2}",
//         ans.range(),
//         ans.average(),
//         ans.median()
//     );

//     fn part_rec<'a>(arg: usize, memory: Vec<Option<Vec<i32>>>) -> impl Iterator<Item = i32> + 'a {
//         match memory[arg] {
//             Some(vec) => vec,
//             None => todo!()
//         }
//     }
// }

// trait VecExt {
//     fn range(self) -> f64;
//     fn average(self) -> f64;
//     fn median(self) -> f64;
// }

// impl VecExt for Vec<i32> {
//     fn range(self) -> f64 {
//         *self.iter().max().unwrap() as f64 - *self.iter().min().unwrap() as f64
//     }
//     fn average(self) -> f64 {
//         self.iter().map(|f| *f as f64 / (self.len() as f64)).sum()
//     }
//     fn median(self) -> f64 {
//         if self.len() % 2 == 0 {
//             (self[self.len() / 2 - 1] as f64 + self[self.len() / 2] as f64) / 2_f64
//         } else {
//             self[self.len() / 2] as f64
//         }
//     }
// }

// #[test]
// fn returns_expected() {
//     testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
//     testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
//     testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
//     testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
//     testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
// }
// fn testequal(ans: &str, sol: &str) {
//     assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
// }
