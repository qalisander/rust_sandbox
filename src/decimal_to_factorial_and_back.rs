fn dec2_fact_string(nb: u64) -> String {
    let mut num: u64 = nb;

    (0..=50)
        .rfold(String::new(), |mut ans, pow| {
            let fact = saturating_fact(pow);
            if num >= fact {
                let i = (1..=pow).filter(|i| i * fact <= num).last().unwrap_or(0);
                ans.push(char::from_digit(i as u32, 36).unwrap().to_ascii_uppercase());
                num -= i * fact;
            } else {
                ans.push_str("0")
            }
            ans
        })
        .trim_start_matches('0')
        .into()
}

fn fact_string_2dec(s: String) -> u64 {
    s.chars().rev().enumerate().fold(0, |acc, (index, ch)| {
        acc + ch.to_digit(36).unwrap() as u64 * saturating_fact(index as u64)
    })
}

fn saturating_fact(num: u64) -> u64 {
    (1..=num).reduce(|x, y| x.saturating_mul(y)).unwrap_or(1)
}

#[test]
fn factorial_test() {
    assert_eq!(3_628_800, saturating_fact(10));
}

#[test]
fn basics_dec2_fact_string() {
    testing1(2982, "4041000");
    testing1(463, "341010");
    testing1(36288000, "A0000000000");

    fn testing1(nb: u64, exp: &str) -> () {
        assert_eq!(&dec2_fact_string(nb), exp)
    }
}

#[test]
fn basics_fact_string_2dec() {
    testing2("4041000", 2982);
    testing2("341010", 463);

    fn testing2(s: &str, exp: u64) -> () {
        assert_eq!(fact_string_2dec(s.to_string()), exp)
    }
}
