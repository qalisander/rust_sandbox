// https://www.codewars.com/kata/5265b0885fda8eac5900093b/train/rust

use std::collections::HashMap;
use std::iter::Peekable;

const IMM: &str = "imm";
const ARG: &str = "arg";

#[derive(Debug, Clone, Eq, PartialEq)]
enum Ast {
    BinOp(String, Box<Ast>, Box<Ast>),
    UnOp(String, i32),
}

impl Ast {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

struct Parser<I>
where
    I: Iterator<Item = String> + Clone,
{
    arg_to_index: HashMap<String, i32>,
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

    /// Syntax
    /// - function   ::= '[' arg-list ']' expression
    /// - arg-list   ::= variable*
    /// - expression ::= term ( '+' | '-' term )*
    /// - term       ::= factor ( '*' | '/' factor )*
    /// - factor     ::= number | variable | '(' expression ')'
    pub fn parse(&mut self) -> Ast {
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

    fn term(&mut self) -> Ast {
        let mut left = self.factor();
        loop {
            match self.tokens.peek() {
                None => break left,
                Some(op) => match &**op {
                    ch @ ("+" | "-") => {
                        let op = ch.to_string();
                        self.tokens.next();
                        left = Ast::BinOp(op, left.boxed(), self.factor().boxed())
                    }
                    _ => break left,
                },
            }
        }
    }

    fn factor(&mut self) -> Ast {
        let mut left = self.primary();
        loop {
            match self.tokens.peek() {
                None => break left,
                Some(op) => match &**op {
                    ch @ ("*" | "/") => {
                        let op = ch.to_string();
                        self.tokens.next();
                        left = Ast::BinOp(op, left.boxed(), self.primary().boxed())
                    }
                    _ => break left,
                },
            }
        }
    }

    fn primary(&mut self) -> Ast {
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
            return Ast::UnOp(IMM.to_string(), num);
        }
        if let Some(index) = self.arg_to_index.get(&token) {
            return Ast::UnOp(ARG.to_string(), *index);
        }
        panic!("Invalid token! {token}")
    }
}

const WORDS: &'static str = "hello rust!";

struct Compiler {}

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }

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

    /// Compile to ast
    fn pass1(&mut self, program: &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut parser = Parser::new(tokens.into_iter());
        parser.parse()
    }

    /// Reduce consts
    fn pass2(&mut self, ast: &Ast) -> Ast {
        match ast {
            Ast::BinOp(op, left, right) => {
                let left = self.pass2(&**left);
                let right = self.pass2(&**right);
                match (left, right) {
                    (Ast::UnOp(l_code, l_num), Ast::UnOp(r_code, r_num))
                        if l_code == IMM && r_code == IMM =>
                    {
                        let num = match &**op {
                            "+" => l_num + r_num,
                            "-" => l_num - r_num,
                            "*" => l_num * r_num,
                            "/" => l_num / r_num,
                            _ => panic!("Invalid operation!"),
                        };
                        let string = IMM.to_string();
                        Ast::UnOp(string, num)
                    }
                    (left, right) => Ast::BinOp(op.clone(), left.boxed(), right.boxed()),
                }
            }
            ast => ast.clone(),
        }
    }

    /// Compile to asm
    /// - "IM n"     load the constant value n into R0
    /// - "AR n"     load the n-th input argument into R0
    /// - "SW"       swap R0 and R1
    ///
    ///
    /// - "PU"       push R0 onto the stack
    /// - "PO"       pop the top value off of the stack into R0
    ///
    ///
    /// - "AD"       add R1 to R0 and put the result in R0
    /// - "SU"       subtract R1 from R0 and put the result in R0
    /// - "MU"       multiply R0 by R1 and put the result in R0
    /// - "DI"       divide R0 by R1 and put the result in R0
    fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        fn compile_rec(asm: &mut Vec<String>, ast: &Ast) {
            match ast {
                Ast::BinOp(op, left, right) => {
                    compile_rec(asm, &**left);
                    asm.push("PU".to_string());
                    compile_rec(asm, &**right);
                    asm.push("SW".to_string());
                    asm.push("PO".to_string());
                    let asm_op = match &**op {
                        "+" => "AD",
                        "-" => "SU",
                        "*" => "MU",
                        "/" => "DI",
                        _ => unreachable!(),
                    };
                    asm.push(asm_op.to_string())
                }
                Ast::UnOp(code, num) => match &**code {
                    IMM => asm.push(format!("IM {num}")),
                    ARG => asm.push(format!("AR {num}")),
                    _ => unreachable!(),
                },
            }
        }

        let mut ans = vec![];
        compile_rec(&mut ans, ast);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreters::three_pass_compiler::Ast::{BinOp, UnOp};

    #[test]
    fn pass1_map_to_expression() {
        let program = "[ a b ] a*a + b*b";
        let expected_ast = BinOp(
            "+".to_string(),
            BinOp(
                "*".to_string(),
                UnOp(ARG.to_string(), 0).boxed(),
                UnOp(ARG.to_string(), 0).boxed(),
            )
            .boxed(),
            BinOp(
                "*".to_string(),
                UnOp(ARG.to_string(), 1).boxed(),
                UnOp(ARG.to_string(), 1).boxed(),
            )
            .boxed(),
        );
        let mut compiler = Compiler::new();
        let ast = compiler.pass1(program);
        assert_eq!(ast, expected_ast)
    }

    #[test]
    fn pass2_reduce_consts() {
        let program = "[ a b ] a*a + 3*3*3";
        let expected_ast = BinOp(
            "+".to_string(),
            BinOp(
                "*".to_string(),
                UnOp(ARG.to_string(), 0).boxed(),
                UnOp(ARG.to_string(), 0).boxed(),
            )
            .boxed(),
            UnOp(IMM.to_string(), 27).boxed(),
        );
        let mut compiler = Compiler::new();
        let ast = compiler.pass1(program);
        let ast = compiler.pass2(&ast);
        assert_eq!(ast, expected_ast)
    }

    #[test]
    fn pass3_reduce_consts() {
        let program = "[ a b ] (5 + b) * (10 + a)";
        let args = vec![1, 0];
        let expected_ans = 55;
        let expected_asm = [
            "IM 5", "PU", "AR 1", "SW", "PO", "AD", "PU", "IM 10", "PU", "AR 0", "SW", "PO", "AD",
            "SW", "PO", "MU",
        ]
        .map(|str| str.to_string())
        .to_vec();
        let mut compiler = Compiler::new();
        let asm = compiler.compile(program);
        assert_eq!(asm, expected_asm);
        assert_eq!(simulate(asm, args), expected_ans);
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
