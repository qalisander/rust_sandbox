use itertools::Itertools;

fn order_weight(s: &str) -> String {
    s.split_whitespace()
        .sorted_by_key(|&str| {
            (
                str.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>(),
                str,
            )
        })
        .join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
