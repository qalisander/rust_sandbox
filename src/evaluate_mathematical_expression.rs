//NOTE: https://www.codewars.com/kata/52a78825cdfc2cfc87000005/train/rust
//TODO: evaluate expression
// TODO: use itertools tree_fold1() for printing tree structure

#[macro_use]
use std::borrow::{Borrow, Cow};
use itertools::Itertools;
use std::iter;

#[derive(Debug, PartialEq, Clone)]
struct Token {
    index: usize,
    len: usize,
    t_type: TType,
}

#[derive(Debug, PartialEq, Clone)]
enum TType {
    Space,
    Op(char),
    LeftParen,
    RightParen,
    Num(f64),
}

fn calc(input_expr: &str) -> f64 {
    let scan1 = scan(input_expr);
    create_expr(scan1);
    todo!("evaluate expr")
}

fn create_expr(tokens: impl Iterator<Item = Token>){
    todo!("create expr");
}

// NOTE: str.chars().coalesce()
// TODO: use dedup for spaces
// NOTE: while_some
fn scan(str: &str) -> impl Iterator<Item = Token> + '_{
    str.chars().enumerate().batching(|iter| {
        match iter.next() {
            None => None,
            Some((index, ch)) => {
                let token = match ch {
                    '0'..='9' => {
                        let num_str: String = iter::once(ch)
                            .chain(iter.take_while_ref(|(_, ch)| ch.is_numeric() || *ch == '.').map(|(_, ch)| ch))
                            .collect();
                        let num = num_str.parse::<f64>().expect(format!("Invalid token! index:{index}").as_str());
                        Token { index, len: num_str.len(), t_type: TType::Num(num) }
                    },
                    '(' => Token { index, len: 1, t_type: TType::LeftParen },
                    ')' => Token { index, len: 1, t_type: TType::RightParen },
                    '+' | '-' | '/' | '*' => Token { index, len: 1, t_type: TType::Op(ch) },
                    ' ' => {
                        let last_index = iter
                            .take_while_ref(|(_, ch)| ch.is_whitespace())
                            .map(|(index, _)| index)
                            .last()
                            .unwrap_or(index);
                        Token { index, len: 1 + last_index - index, t_type: TType::Space }
                    }
                    _ => panic!("Invalid token! index:{0}", index), // TODO: check exception, format errors
                };
                Some(token)
            }
        }
    })
}

#[deny(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use super::calc;
    use crate::evaluate_mathematical_expression::{scan, TType, Token};
    use itertools::Itertools;

    #[test]
    fn scan_test() {
        let test = "1 + -2.24   (   -12323";
        let tokens = scan(test).map(|token| token.t_type).collect_vec();
        let expected_tokens = vec![ //itertools::assert_equal(
            TType::Num(1.0),
            TType::Space,
            TType::Op('+'),
            TType::Space,
            TType::Op('-'),
            TType::Num(2.24),
            TType::Space,
            TType::LeftParen,
            TType::Space,
            TType::Op('-'),
            TType::Num(12323.0),
        ];
        assert_eq!(tokens, expected_tokens);

        let test = "1.0  ";
        let tokens = scan(test).collect_vec();
        let expected_tokens = vec![
            Token{index: 0, len: 3, t_type: TType::Num(1.0)},
            Token{index: 3, len: 2, t_type: TType::Space}];
        assert_eq!(tokens, expected_tokens);
    }

    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert!(
                calc($expr).eq(&$expect),
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }

}

