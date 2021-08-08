// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254/train/rust

use std::collections::HashMap;
use itertools::Itertools;
// #![feature(test)]

fn partitions(num: isize) -> isize {
    // let mut memory = HashMap::new();
    return part_rec(num, num/*, &mut memory*/);
    
    fn part_rec(num: isize, max_num: isize/*, memory: &mut HashMap<isize, isize>*/) -> isize {
/*        if let Some(&ans) = memory.get(&num) {
            return ans;
        }*/
        
        if num == 0 { 1 } else {
            (1..=num.min(max_num)).map(|delta| {
                part_rec(num - delta, delta.min(num - delta))
            }).sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

    // #[bench]
    // fn basic_test_100() {
    //     assert_eq!(partitions(50), 1958);
    // }
}
