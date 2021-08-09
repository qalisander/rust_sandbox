// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254/train/rust

use std::collections::{HashMap};

// TODO: compare with criterion: BTreeMap, HashMap and BinaryHeap
fn partitions(num: isize) -> isize {
    let mut memory = HashMap::new();
    return part_rec(num, num, &mut memory);
    
    fn part_rec(num: isize, max_num: isize, memory: &mut HashMap<(isize, isize), isize>) -> isize {
        if let Some(&ans) = memory.get(&(num, max_num)) {
            return ans;
        }
        
        let ans = if num == 0 { 1 } else {
            (1..=num.min(max_num)).map(|delta| {
                part_rec(num - delta, delta.min(num - delta), memory)
            }).sum()
        };
        
        memory.insert((num, max_num), ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

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
    
    #[test]
    fn basic_test_70() {
        assert_eq!(partitions(70), 4087968);
    }

    #[bench]
    fn bench_test_70(bencher: &mut Bencher) {
        bencher.iter(||{
            partitions(70)
        });
    }

    #[bench]
    fn bench_test_100(bencher: &mut Bencher) {
        bencher.iter(||{
            partitions(100)
        });
    }
}

    