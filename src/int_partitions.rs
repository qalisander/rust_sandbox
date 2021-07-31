use itertools::Itertools;
use std::collections::HashMap;

// https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b/train/rust
fn part(n: i64) -> String {
    let mut memory = HashMap::new();
    let ans = part_rec(n as usize, &mut memory);
    return format!(
        "Range: {} Average: {:.2} Median: {:.2}",
        ans.range(),
        ans.average(),
        ans.median()
    );

    fn part_rec(arg: usize, memory: &mut HashMap<usize, Vec<i32>>) -> Vec<i32> {
        if let Some(vct) = memory.get(&arg) {
            return vct.to_vec();
        }

        let vec = std::iter::once(arg as i32)
            .chain((1..=arg / 2)
                .flat_map(|i| {
                    part_rec(&arg - i, memory).iter().map(|x| (x * (i as i32))).collect_vec()
                }))
            .unique()
            .sorted()
            .collect_vec();

        memory.insert(arg, vec);
        return memory.get(&arg).unwrap().to_vec();
    }
}

trait VecExt {
    fn range(&self) -> f64;
    fn average(&self) -> f64;
    fn median(&self) -> f64;
}

impl VecExt for Vec<i32> {
    fn range(&self) -> f64 {
        *self.iter().max().unwrap() as f64 - *self.iter().min().unwrap() as f64
    }
    
    fn average(&self) -> f64 {
        self.iter().map(|f| *f as f64 / (self.len() as f64)).sum()
    }
    
    fn median(&self) -> f64 {
        if self.len() % 2 == 0 {
            (self[self.len() / 2 - 1] as f64 + self[self.len() / 2] as f64) / 2_f64
        } else {
            self[self.len() / 2] as f64
        }
    }
}

#[test]
fn returns_expected() {
    testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
    testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    testequal(&part(50), "Range: 86093441 Average: 1552316.81 Median: 120960.00");
}
fn testequal(ans: &str, sol: &str) {
    assert_eq!(ans, sol);
}

// Shortest solution:
// fn sub(n: i64, m: i64) -> Vec<i64> {
//     if m == 1 { return vec![1]; }
//     (0..=n/m).flat_map(|p| {
//         let pow = m.pow(p as u32);
//         sub(n - m * p, m - 1).iter().map(|s| s * pow).collect::<Vec<_>>()
//     }).collect()
// }
// 
// fn part(n: i64) -> String {
//     let mut v = sub(n, n);
//     v.sort_unstable();
//     v.dedup();
//     let len = v.len();
//     let rng = v[len - 1] - v[0];
//     let avg = v.iter().sum::<i64>() as f64 / len as f64;
//     let med = if len % 2 == 0 { (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0 } else { v[len / 2] as f64 };
//     format!("Range: {} Average: {:.02} Median: {:.02}", rng, avg, med)
// }
