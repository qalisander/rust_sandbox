// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750/train/rust

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use itertools::{EitherOrBoth, Itertools, MultiPeek, PeekingNext};
use std::iter;
use std::iter::Peekable;
use std::ops::Deref;


#[derive(Copy, Clone, Debug)]
struct Token<'a> {
    index: usize,
    len: usize,
    t_type: TType<'a>,
}

// TODO: add end of string token and make programs bit more difficult
// TODO: add errors highlighting
// TODO: add parsing tests
// TODO: add benchmarks
#[derive(Copy, Clone, Debug, PartialEq)]
enum TType<'a> {
    Op(char),
    Assignment,
    FnArrow,
    LeftParen,
    RightParen,
    Num(f32),
    Fn,
    Identifier(&'a str),
}

#[derive(Clone, Debug)]
enum Expr<'a> {
    Binary(Box<Expr<'a>>, char, Box<Expr<'a>>),
    Unary(char, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Num(f32),
    Var(&'a str),
    Fn {
        args: Box<[Expr<'a>]>,
        identifier: &'a str,
    },
}

#[derive(Clone, Debug)]
enum Stmt<'a> {
    FnDeclaration { identifier: &'a str, body: FnBody<'a> },
    Assignment { identifier: &'a str, value: Expr<'a> },
    Expr(Expr<'a>),
}

impl<'a> From<Expr<'a>> for Stmt<'a> {
    fn from(expr: Expr<'a>) -> Stmt<'a> { Self::Expr(expr) }
}

#[derive(Clone, Debug)]
struct FnBody<'a> {
    params: Box<[&'a str]>,
    callee: Box<Expr<'a>>,
}

struct Interpreter<'a> {
    // TODO: compare hashset with vec in benchmark
    // TODO: stage2, store functions and FnBody in single table
    var_stack: Vec<(&'a str, f32)>,
    funcs: HashMap<&'a str, FnBody<'a>>,
}

const INVALID_END: &str = "Invalid ending!";

impl<'a> Interpreter<'a> {

    fn new() -> Interpreter<'a> {
        Self{
            var_stack: vec![],
            funcs: HashMap::new(),
        }
    }

    // TODO: use cow<'static, &str>
    // TODO: format error string with ERROR: prefix
    fn input(&mut self, input: &'a str) -> Result<Option<f32>, String> {
        let tokens = scan(input);
        let statement = Self::parse(tokens)?;
        self.interpret(statement)
    }

    // statement      → "fn" identifier "=>" expression | (identifier "=")? expression ;
    // expression     → term ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → "-" unary | primary ;
    // primary        → "(" term ")" | identifier | number;
    // TODO: extract as separate method (maybe create struct parser and pass there iterator)
    fn parse(tokens: impl Iterator<Item = Token<'a>> + 'a) -> Result<Stmt<'a>, String> {
        let mut peekable = tokens.multipeek();
        let expr = stmt(&mut peekable);
        return match peekable.next() {
            Some(token) => Err(format!("Invalid token! index:{0}", token.index)),
            None => expr,
        };

        fn stmt<'a>(tokens: &mut MultiPeek<impl Iterator<Item=Token<'a>>>) -> Result<Stmt<'a>, String> {
            let token = tokens.peek().ok_or(INVALID_END.to_string())?;
            match token.t_type {
                TType::Fn => {
                    tokens.next();
                    match tokens.next().ok_or(INVALID_END.to_string())? {
                        Token { t_type: TType::Identifier(identifier), .. } => {
                            let params = iter::from_fn(||{
                                match tokens.peek()?.t_type {
                                    TType::Identifier(param) => {
                                        tokens.next();
                                        Some(param)
                                    },
                                    _ => None,
                                }
                            }).collect_vec().into_boxed_slice();

//                            let params = tokens
//                                .peeking_take_while(|token| matches!(token.t_type, TType::Identifier(_)))
//                                .map(|token| match token.t_type {
//                                    TType::Identifier(param) => param,
//                                    _ => unreachable!(),
//                                })
//                                .collect_vec()
//                                .into_boxed_slice();

                            let callee = Box::new(term(&mut tokens.peekable())?);
                            let body = FnBody { params, callee };
                            Ok(Stmt::FnDeclaration { identifier, body })
                        }
                        token => Err(format!("Invalid token! index:{0}", token.index))
                    }
                },
                TType::Identifier(var) => {
                    match tokens.peek() {
                        Some(&Token{t_type: TType::Assignment, ..}) => {
                            tokens.next_tuple::<(_,_)>();
                            Ok(Stmt::Assignment {identifier: var, value: term(&mut tokens.peekable())?})
                        },
                        _ => Ok(term(&mut tokens.peekable())?.into()),
                    }
                },
                _ => Ok(term(&mut tokens.peekable())?.into()),
            }
        }


        fn term<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Result<Expr<'a>, String> {
            let mut left_expr = factor(tokens)?;
            loop {
                match tokens.peek() {
                    Some(&Token { t_type: TType::Op(ch @ ('+' | '-') ), .. }) => {
                        tokens.next();
                        let left = Box::new(left_expr);
                        let right = Box::new(factor(tokens)?);
                        left_expr = Expr::Binary(left, ch, right);
                    },
                    _ => break Ok(left_expr),
                }
            }
        }

        fn factor<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Result<Expr<'a>, String> {
            let mut left_expr = unary(tokens)?;
            loop {
                match tokens.peek() {
                    Some(&Token { t_type: TType::Op(ch @ ('*' | '/' | '%')), .. }) => {
                        tokens.next();
                        let left = Box::new(left_expr);
                        let right = Box::new(unary(tokens)?);
                        left_expr = Expr::Binary(left, ch, right);
                    },
                    _ => break Ok(left_expr),
                }
            }
        }

        fn unary<'a>(tokens: &mut Peekable<impl Iterator<Item = Token<'a>>>) -> Result<Expr<'a>, String> {
            match tokens.peek() {
                None => Err(INVALID_END.to_string()),
                Some(&token) => match token.t_type {
                    TType::Op(ch @ '-') => {
                        tokens.next();
                        let expr = Box::new(unary(tokens)?);
                        Ok(Expr::Unary(ch, expr))
                    }
                    _ => primary(tokens),
                },
            }
        }

        fn primary<'a>(tokens: &mut Peekable<impl Iterator<Item=Token<'a>>>) -> Result<Expr<'a>, String> {
            let token = tokens.next().ok_or(INVALID_END.to_string())?;
            match token.t_type {
                TType::Num(num) => Ok(Expr::Num(num)),
                TType::Identifier(var) => Ok(Expr::Var(var)),
                TType::LeftParen => {
                    let expr = term(tokens)?;
                    match tokens.next() {
                        Some(token) => match token.t_type {
                            TType::RightParen => Ok(Expr::Grouping(Box::new(expr))),
                            _ => Err(format!("Invalid paren! index:{0}", token.index)),
                        },
                        None => Err("Invalid paren!".to_string()), // TODO: move to const
                    }
                }
                _ => Err(format!("Invalid token! index:{0}", token.index)),
            }
        }
    }

    //TODO: implement separate variables resolver

    fn interpret(&mut self, stmt: Stmt<'a>) -> Result<Option<f32>, String>{
        match stmt {
            Stmt::FnDeclaration { identifier, body } => {
                self.check_fn_params(&body)?;
                self.funcs.insert(identifier, body);
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

    // NOTE: no recursion implementation looks pretty:)
    fn check_fn_params(&mut self, fn_body: &FnBody) -> Result<(), String> {
        let param_not_exist = |identifier: &str| {
            !(fn_body.params.contains(&identifier)
                || self.var_stack.iter().any(|(var, _)| *var == identifier))
        };
        let mut stack = vec![fn_body.callee.deref()];
        loop {
            match stack.pop() {
                Some(expr) => match expr{
                    Expr::Binary(left, _, right) => stack.extend([left.deref(), right]),
                    Expr::Unary(_, expr) => stack.push(expr),
                    Expr::Grouping(expr) => stack.push(expr),
                    Expr::Fn {args, ..} => stack.extend(args.iter()),
                    Expr::Var(identifier) if param_not_exist(identifier) =>
                        break Err(format!("ERROR: Invalid identifier '{0}' in function body", {identifier})),
                    _ => (),
                },
                None => break Ok(()),
            }
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
            Expr::Var(identifier) => *self.var_stack.iter() // BUG: evaluate function
                .rfind(|(str, _)| str == identifier)
                .map(|(_, val)| val)
                .ok_or(format!("ERROR: Invalid variable identifier '{0}'", identifier))?,
            Expr::Fn {args: expr_args, identifier} => {
                let args: Vec<_> = expr_args.iter()
                    .map(|arg| self.eval(arg))
                    .try_collect()?;

                let func_body = self.funcs.get(identifier)
                    .ok_or(format!("ERROR: Invalid function identifier '{0}'", identifier))?;

                let vars: Vec<_> = func_body.params.iter()
                    .zip_longest(args)
                    .map(|arg| {
                        match arg {
                            EitherOrBoth::Both(&param, arg) => Ok((param, arg)),
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


#[rustfmt::skip]
fn scan(input: &str) -> impl Iterator<Item=Token> + '_{
    input.char_indices().peekable().batching(move |iter|{
        iter.peeking_take_while(|(_, ch)| ch.is_whitespace()).for_each(|_|());
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
                    '=' => match iter.next_if(|(_, ch)| *ch == '>') {
                        None => Token { index, len: 1, t_type: TType::Assignment },
                        Some(_) => Token { index, len: 2, t_type: TType::FnArrow }
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let identifier: String = iter::once(ch)
                            .chain(iter
                                .peeking_take_while(|(_, ch)|
                                    ch.is_alphabetic() || ch.is_numeric() || *ch == '_')
                                .map(|(_, ch)| ch))
                            .collect();
                        let len = identifier.len(); //NOTE: bytes
                        let t_type = if identifier == "fn" { TType::Fn } else { TType::Identifier(&input[index..index + len]) }; //TODO: index of last one
                        Token {index, len, t_type}
                    },
                    _ => panic!("Invalid token! index:{0}, char:{1}", index, ch),
                };
                Some(token)
            }
        }
    })
}

#[test]
fn scan_test(){
    let test = "fn avg x y =>  (x + y) / 2";
    let tokens = scan(test).map(|token| token.t_type);
    let expected_tokens = [
        TType::Fn,
        TType::Identifier("avg"),
        TType::Identifier("x"),
        TType::Identifier("y"),
        TType::FnArrow,
        TType::LeftParen,
        TType::Identifier("x"),
        TType::Op('+'),
        TType::Identifier("y"),
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
