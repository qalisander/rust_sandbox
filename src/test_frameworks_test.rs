fn add5<T: Into<u32>>(component: T) -> u32 {
    component.into() + 5
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

//    ide!();

    #[parameterized(input = {
    0, 1, 2
    }, expected = {
    5, 6, 7
    })]
    fn test_add5(input: u16, expected: u32) {
        assert_eq!(add5(input), expected);
    }
}

// The next two lines are not needed for 2018 edition or newer
#[cfg(test)]
extern crate test_case;

#[cfg(test)]
mod tests_2 {
    use test_case::test_case;

    // Not needed for this example, but useful in general
    use super::*;

    #[test_case(4,  2  ; "when operands are swapped")]
    #[test_case(-2, -4 ; "when both operands are negative")]
    #[test_case(2,  4  ; "when both operands are positive")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }

    // You can still use regular tests too
    #[test]
    fn addition_test() {
        let actual = -2 + 8;
        assert_eq!(6, actual)
    }
}
