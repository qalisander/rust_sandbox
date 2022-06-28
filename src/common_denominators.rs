//https://www.codewars.com/kata/54d7660d2daf68c619000d95
fn convert_fracts(input: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let input_nrml = input
        .into_iter()
        .map(|tpl| {
            let gcf = gcf(tpl.0, tpl.1) as i64;
            (tpl.0 / gcf, tpl.1 / gcf)
        })
        .collect::<Vec<_>>();

    let common_denominator = input_nrml.iter().map(|tpl| tpl.1).fold(1, |x, y| lcm(x, y));

    input_nrml
        .into_iter()
        .map(|tpl| (common_denominator / tpl.1 * tpl.0, common_denominator))
        .collect()
}

// NOTE nuber lcm and gcf can be used
/// lowest common multiple
fn lcm(x: i64, y: i64) -> i64 {
    ((x as u64) * (y as u64) / gcf(x, y) as u64) as i64
}

/// greatest_common_factor
fn gcf(x: i64, y: i64) -> i64 {
    let (mut first, mut second) = (x.max(y), x.min(y));

    loop {
        if first % second == 0 {
            return second;
        } else {
            let tmp = first % second;
            first = second;
            second = tmp;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics_convert_fracts() {
        testing(
            vec![(69, 130), (87, 1310), (3, 4)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );
        testing(
            vec![(690, 1300), (87, 1310), (30, 40)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );

        fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
            assert_eq!(convert_fracts(l), exp)
        }
    }

    #[test]
    fn lowest_common_mult_test() {
        assert_eq!(gcf(17, 13), 1);
        assert_eq!(gcf(8, 18), 2);
        assert_eq!(gcf(1300, 1310), 10);
    }

    #[test]
    fn greatest_common_factor_test() {
        assert_eq!(lcm(1300, 1310), 170300)
    }
}

// NOTE: different solution
// NOTE: can be used gcg and lcd of integer types
//fn gcd(a: i64, b: i64) -> i64 {
//    if b == 0 {
//        a
//    } else {
//        gcd(b, a % b)
//    }
//}
//fn lcm(a: i64, b: i64) -> i64 {
//    a / gcd(a, b) * b
//}
//fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
//    let d = l
//        .iter()
//        .fold(1, |acc, &(num, den)| lcm(acc, den / gcd(num, den)));
//    l.iter().map(|&(num, den)| (num * d / den, d)).collect()
//}