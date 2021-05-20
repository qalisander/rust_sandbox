// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254/train/rust

fn partitions(n: isize) -> isize {
    1
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
}
