// https://www.codewars.com/kata/5265b0885fda8eac5900093b/train/rust

use std::borrow::Cow;
use std::collections::btree_map::IntoKeys;
use std::collections::HashMap;
use std::iter::Peekable;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Binary(Box<Expr>, char, Box<Expr>),
    Var(usize),
    Num(i32),
}

impl Expr {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

struct Parser<I>
where
    I: Iterator<Item = String> + Clone,
{
    arg_to_index: HashMap<String, usize>,
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = String> + Clone,
{
    pub fn new(tokens: I) -> Self {
        Self {
            arg_to_index: HashMap::new(),
            tokens: tokens.into_iter().peekable(),
        }
    }

    // Syntax
    //  function   ::= '[' arg-list ']' expression
    //  arg-list   ::= variable*
    //  expression ::= term ( '+' | '-' term )*
    //  term       ::= factor ( '*' | '/' factor )*
    //  factor     ::= number | variable | '(' expression ')'
    fn parse(&mut self) -> Expr {
        self.tokens.next_if_eq("[").expect("Invalid arg-list!");
        for index_of_arg in 0.. {
            let arg = self.tokens.next().expect("Invalid arg-list!");
            if arg == "]" {
                break;
            }
            self.arg_to_index.insert(arg, index_of_arg);
        }
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        loop {
            match self.tokens.peek() {
                None => break left,
                Some(op) => match &**op {
                    ch @ ("+" | "-") => {
                        let ch: char = ch.parse().unwrap();
                        self.tokens.next();
                        left = Expr::Binary(left.boxed(), ch, self.factor().boxed())
                    }
                    _ => break left,
                },
            }
        }
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.primary();
        loop {
            match self.tokens.peek() {
                None => break left,
                Some(op) => match &**op {
                    ch @ ("*" | "/") => {
                        let ch: char = ch.parse().unwrap();
                        self.tokens.next();
                        left = Expr::Binary(left.boxed(), ch, self.primary().boxed())
                    }
                    _ => break left,
                },
            }
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.tokens.next().expect("Invalid ending!");
        if "(" == token {
            let term = self.term();
            let token = self.tokens.next().expect("Invalid ending!");
            if token == ")" {
                return term;
            }
            panic!("Invalid paren!")
        }
        if let Ok(num) = token.parse::<i32>() {
            return Expr::Num(num);
        }
        if let Some(index) = self.arg_to_index.get(&token) {
            return Expr::Var(*index);
        }
        panic!("Invalid token! {token}")
    }
}

struct Compiler {}

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }

    // TODO: use &str
    // TODO: rename to scan and use batching method, return lazy iterator
    // TODO: use simple tokens
    fn tokenize(&self, program: &str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];

        let mut iter = program.chars().peekable();
        while let Some(&c) = iter.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(tmp);
                }
                '0'..='9' => {
                    let mut tmp = String::new();
                    while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                        tmp.push(iter.next().unwrap());
                    }
                    tokens.push(tmp);
                }
                ' ' => {
                    iter.next();
                }
                _ => {
                    tokens.push(iter.next().unwrap().to_string());
                }
            }
        }
        tokens
    }

    fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    // Compile to ast
    fn pass1(&mut self, program: &str) -> Expr {
        //        self.tokenize(program).into()
        todo!()
    }

    // Reduce constants
    fn pass2(&mut self, ast: &Expr) -> Expr {
        ast.clone()
    }

    // Compiling ast
    fn pass3(&mut self, ast: &Expr) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreters::three_pass_compiler::Expr::{Binary, Num, Var};
    use parameterized::ide;
    use test_case::test_case;

    #[test]
    fn pass2() {
        fn test(program: &str, expected_ast: Expr) {
            let mut compiler = Compiler::new();
            let ast = compiler.pass1(program);
            let ast = compiler.pass2(&ast);
            assert_eq!(ast, expected_ast)
        }

        let program = "[ a b ] a*a + b*b";
        let expected_expr = Binary(
            Binary(Var(0).boxed(), '*', Var(0).boxed()).boxed(),
            '+',
            Binary(Var(1).boxed(), '*', Var(1).boxed()).boxed(),
        );
        test(program, expected_expr)
    }

    #[test]
    fn simulator() {
        assert_eq!(simulate(vec!["IM 7".to_string()], vec![3]), 7);
        assert_eq!(simulate(vec!["AR 1".to_string()], vec![1, 2, 3]), 2);
    }

    fn simulate(assembly: Vec<String>, argv: Vec<i32>) -> i32 {
        let mut r = (0, 0);
        let mut stack: Vec<i32> = vec![];

        for ins in assembly {
            let mut ws = ins.split_whitespace();
            match ws.next() {
                Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
                Some("AR") => {
                    r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize]
                }
                Some("SW") => r = (r.1, r.0),
                Some("PU") => stack.push(r.0),
                Some("PO") => r.0 = stack.pop().unwrap(),
                Some("AD") => r.0 += r.1,
                Some("SU") => r.0 -= r.1,
                Some("MU") => r.0 *= r.1,
                Some("DI") => r.0 /= r.1,
                _ => panic!("Invalid instruction encountered"),
            }
        }
        r.0
    }
}
