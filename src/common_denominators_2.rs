// NOTE: can be used gcg and lcd of integer types
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let d = l
        .iter()
        .fold(1, |acc, &(num, den)| lcm(acc, den / gcd(num, den)));
    l.iter().map(|&(num, den)| (num * d / den, d)).collect()
}

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
