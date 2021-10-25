// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750/train/rust

use std::collections::HashMap;
use itertools::{EitherOrBoth, Itertools, PeekingNext};
use std::iter;

#[derive(Clone, Debug)]
struct Token {
    index: usize,
    len: usize,
    t_type: TType,
}

// TODO: add end of string token
#[derive(Clone, Debug, PartialEq)]
enum TType {
    Op(char),
    Assignment,
    FnArrow,
    LeftParen,
    RightParen,
    Num(f32),
    Fn,
    Identifier(String), // TODO: maybe use &str
}

#[derive(Clone, Debug)]
enum Expr {
    Binary(Box<Expr>, char, Box<Expr>),
    Unary(char, Box<Expr>),
    Grouping(Box<Expr>),
    Num(f32),
    Var(String),
    Fn {
        args: Box<[Expr]>,
        identifier: String,
    },
}

#[derive(Clone, Debug)]
enum Stmt {
    FnDeclaration { identifier: String, body: FnBody },
    Assignment { identifier: String, value: Expr },
    Expr(Expr),
}

#[derive(Clone, Debug)]
struct FnBody {
    params: Box<[String]>,
    callee: Box<Expr>,
}

struct Interpreter {
    // TODO: compare hashset with vec
    // TODO: stage2, store functions and FnBody in single table
    var_stack: Vec<(String, f32)>,
    funcs: HashMap<String, FnBody>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Self{
            var_stack: vec![],
            funcs: HashMap::new(),
        }
    }

    // TODO: use cow<'static, &str>
    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        let tokens = scan(input);
        let statement = self.parse(tokens);
        self.interpret(statement)
    }

    // statement      → "fn" identifier "=>" expression | identifier "=" expression | expression ;
    // expression     → term ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → "-" unary | primary ;
    // primary        → "(" term ")" | identifier | number;
    fn parse(&mut self, tokens: impl Iterator<Item = Token>) -> Stmt {
        unimplemented!()
    }

    //TODO: implement separate variables resolver

    fn interpret(&mut self, stmt: Stmt) -> Result<Option<f32>, String>{
        match stmt {
            Stmt::FnDeclaration { identifier, body } => {
                self.funcs.insert(identifier, body);
                //TODO: respond with error when function has not valid args
                Ok(None)
            },
            Stmt::Assignment { identifier, value: val } => {
                let val = self.eval(&val)?;
                self.var_stack.push((identifier, val));
                Ok(Some(val))
            },
            Stmt::Expr(expr) => self.eval(&expr).map(Some),
        }
    }

    fn eval(&mut self, expr: &Expr) -> Result<f32, String> {
        let res = match expr {
            Expr::Binary(left, ch, right) => match ch {
                '+' => self.eval(left)? + self.eval(right)?,
                '-' => self.eval(left)? - self.eval(right)?,
                '/' => self.eval(left)? / self.eval(right)?,
                '*' => self.eval(left)? * self.eval(right)?,
                '%' => self.eval(left)? % self.eval(right)?,
                op => panic!("Invalid operation! op:{:?}", op),
            },
            Expr::Unary(ch, expr) => -self.eval(expr)?,
            Expr::Grouping(expr) => self.eval(expr)?,
            Expr::Num(num) => *num,
            Expr::Var(identifier) => *self.var_stack.iter()
                .rfind(|(str, _)| str == identifier)
                .map(|(_, val)| val)
                .ok_or(format!("ERROR: Invalid variable identifier '{0}'", identifier))?,
            Expr::Fn {args: expr_args, identifier} => {
                let args: Vec<_> = expr_args.iter().map(|arg| self.eval(arg)).try_collect()?;

                let func_body = self.funcs.get(identifier)
                    .ok_or(format!("ERROR: Invalid function identifier '{0}'", identifier))?;

                let vars: Vec<_> = func_body.params.iter().zip_longest(args).map(|arg| {
                    match arg {
                        EitherOrBoth::Both(param, arg) => Ok((param.clone(), arg)),
                        _ => Err(format!("ERROR: Invalid function '{0}' params", identifier)),
                    }
                }).try_collect()?;
                let vars_len = vars.len();

                self.var_stack.extend(vars);
                let expr = self.eval(&func_body.callee.clone())?;
                self.var_stack.truncate(self.var_stack.len() - vars_len);
                expr
            },
            _ => panic!("Invalid expression! expr:\n{:?}", expr),
        };
        Ok(res)
    }
}


fn scan(input: &str) -> impl Iterator<Item = Token> + '_{
    input.chars().enumerate().peekable().batching(|iter|{
        iter.peeking_next(|(_, ch)| ch.is_whitespace());
        match iter.next() {
            None => None,
            Some((index, ch)) => {
                let token = match ch {
                    '0'..='9' => {
                        let num_str: String = iter::once(ch)
                            .chain(iter
                                .peeking_take_while(|(_, ch)| ch.is_numeric() || *ch == '.')
                                .map(|(_, ch)| ch))
                            .collect();
                        let num = num_str.parse::<f32>()
                            .unwrap_or_else(|_| panic!("Invalid token! index:{0}", index));
                        Token { index, len: num_str.len(), t_type: TType::Num(num) }
                    },
                    '(' => Token { index, len: 1, t_type: TType::LeftParen },
                    ')' => Token { index, len: 1, t_type: TType::RightParen },
                    '+' | '-' | '/' | '*' | '%' => Token { index, len: 1, t_type: TType::Op(ch) },
                    '=' => match iter.peeking_next(|(_, ch)| *ch == '>') {
                        None => Token { index, len: 1, t_type: TType::Assignment },
                        Some(_) => Token { index, len: 2, t_type: TType::FnArrow }
                    },
                    'a'..='z' | 'A'..='Z' => {
                        let identifier: String = iter::once(ch)
                            .chain(iter
                                .peeking_take_while(|(_, ch)| ch.is_alphabetic() || ch.is_numeric())
                                .map(|(_, ch)| ch))
                            .collect();

                        let len = identifier.len();
                        let t_type = if identifier == "fn" { TType::Fn } else { TType::Identifier(identifier) };
                        Token {index, len, t_type}
                    },
                    _ => panic!("Invalid token! index:{0}", index),
                };
                Some(token)
            }
        }
    })
}

#[test]
fn scan_test(){
    let test = "fn avg x y => (x + y) / 2";
    let tokens = scan(test).map(|token| token.t_type);
    let expected_tokens = [
        TType::Fn,
        TType::Identifier("avg".to_string()),
        TType::Identifier("x".to_string()),
        TType::Identifier("y".to_string()),
        TType::FnArrow,
        TType::LeftParen,
        TType::Identifier("x".to_string()),
        TType::Op('+'),
        TType::Identifier("y".to_string()),
        TType::RightParen,
        TType::Op('/'),
        TType::Num(2.0),
    ];

    itertools::assert_equal(tokens, expected_tokens);
}

#[test]
fn basic_arithmetic() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("1 + 1"), Ok(Some(2.0)));
    assert_eq!(i.input("2 - 1"), Ok(Some(1.0)));
    assert_eq!(i.input("2 * 3"), Ok(Some(6.0)));
    assert_eq!(i.input("8 / 4"), Ok(Some(2.0)));
    assert_eq!(i.input("7 % 4"), Ok(Some(3.0)));
}

#[test]
fn variables() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("x"), Ok(Some(1.0)));
    assert_eq!(i.input("x + 3"), Ok(Some(4.0)));
    assert!(i.input("y").is_err());
}

#[test]
fn functions() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert_eq!(i.input("avg 4 2"), Ok(Some(3.0)));
    assert!(i.input("avg 7").is_err());
    assert!(i.input("avg 7 2 4").is_err());
}

#[test]
fn conflicts() {
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = 1"), Ok(Some(1.0)));
    assert_eq!(i.input("fn avg x y => (x + y) / 2"), Ok(None));
    assert!(i.input("fn x => 0").is_err());
    assert!(i.input("avg = 5").is_err());
}
