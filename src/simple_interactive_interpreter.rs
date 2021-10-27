// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750/train/rust

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
// TODO: what happens when you clone &str
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
enum Identifier<'a>{
    Fn(FnBody<'a>),
    Var(Vec<&'a str>),
}

#[derive(Clone, Debug)]
struct FnBody<'a> {
    params: Box<[&'a str]>,
    callee: Box<Expr<'a>>,
}

const INVALID_END: &str = "ERROR: Invalid ending!";
const INVALID_TOKEN: &str = "ERROR: Invalid token!";
const INVALID_PAREN: &str = "ERROR: Invalid paren!";

struct Interpreter<'a> {
    // TODO: compare hashset with vec in benchmark
    // TODO: stage2, store functions and FnBody in single table
    // TODO: name conflicts of variables with function already exist and vice-versa
    vars: HashMap<&'a str, Vec<f32>>,
    funcs: HashMap<&'a str, FnBody<'a>>,
}

impl<'a> Interpreter<'a> {

    fn new() -> Interpreter<'a> {
        Self{
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    // TODO: use cow<'static, &str>
    // TODO: format error string with ERROR: prefix
    fn input(&mut self, input: &'a str) -> Result<Option<f32>, String> {
        dbg!(input);
        let tokens = scan(input);
        if tokens.clone().next().is_none() { // NOTE: crutch
            return Ok(None);
        }
        let mut parser = Parser::new(tokens, &self.funcs);
        let statement = parser.parse()?;
        self.interpret(statement)
    }

    //TODO: implement separate variables resolver

    fn interpret(&mut self, stmt: Stmt<'a>) -> Result<Option<f32>, String>{
        match stmt {
            Stmt::FnDeclaration { identifier, body } => {
                self.check_fn_params(&body)?;
                if self.vars.contains_key(identifier) {
                    Err(format!("ERROR: Name '{0}' already exist", {identifier}))
                }
                else {
                    self.funcs.insert(identifier, body);
                    Ok(None)
                }
            },
            Stmt::Assignment { identifier, value: val } => {
                let val = self.eval(&val)?;
                if self.funcs.contains_key(identifier) {
                    Err(format!("ERROR: Name '{0}' already exist", {identifier}))
                }
                else {
                    self.vars.insert(identifier, vec![val]);
                    Ok(Some(val))
                }
            },
            Stmt::Expr(expr) => self.eval(&expr).map(Some),
        }
    }

    // NOTE: no recursion implementation looks pretty:)
    fn check_fn_params(&mut self, fn_body: &FnBody) -> Result<(), String> {
        let param_not_exist = |identifier: &str| {
            !(fn_body.params.contains(&identifier)
                || self.vars.iter().any(|(var, _)| *var == identifier))
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
                op => panic!("ERROR: Invalid operation! op:{:?}", op),
            },
            Expr::Unary(ch, expr) => -self.eval(expr)?,
            Expr::Grouping(expr) => self.eval(expr)?,
            Expr::Num(num) => *num,
            Expr::Var(fn_identifier) => *self.vars.get(fn_identifier)
                .map(|val| val.last())
                .flatten()
                .ok_or(format!("ERROR: Invalid variable identifier '{0}'", fn_identifier))?,
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

                for (identifier, value) in vars {
                    self.vars.entry(identifier).or_default().push(value);
                };

                let func_body = func_body.deref().clone();
                let expr = self.eval(&func_body.callee)?;

                for identifier in func_body.params.iter() {
                    self.vars.get_mut(identifier).unwrap().pop();
                }
                expr
            },
        };
        Ok(res)
    }
}

struct Parser<'a,'b, T>
    where T: Iterator<Item=Token<'a>> + Clone
{
    tokens: Peekable<T>,
    funcs: &'b HashMap<&'a str, FnBody<'a>>,
}

impl<'a, 'b, T> Parser<'a,'b, T>
    where T: Iterator<Item=Token<'a>> + Clone
{
    fn new(tokens: T, funcs: &'b HashMap<&'a str, FnBody<'a>>) -> Self{
        Parser{ tokens: tokens.peekable(), funcs }
    }

    // statement      → "fn" fn_identifier "=>" expression | (identifier "=")? expression ;
    // expression     → term | fn_identifier (expression)*;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → "-" unary | primary ;
    // primary        → "(" expression ")" | identifier | number;
    // TODO: extract as separate method (maybe create struct parser and pass there iterator)
    // TODO: resolve functions here
    fn parse(&mut self) -> Result<Stmt<'a>, String> {
        let expr = self.stmt();
        return match self.tokens.next() {
            Some(token) => Err(format!("{0} {1:?}", INVALID_TOKEN, token)),
            None => expr,
        };
    }

    fn stmt(&mut self) -> Result<Stmt<'a>, String> {
        let token = self.tokens.peek().ok_or(INVALID_END.to_string())?;
        match token.t_type {
            TType::Fn => {
                self.tokens.next();
                match self.tokens.next().ok_or(INVALID_END.to_string())? {
                    Token { t_type: TType::Identifier(identifier), .. } => {
                        let params = iter::from_fn(||{
                            match self.tokens.peek()?.t_type {
                                TType::Identifier(param) => {
                                    self.tokens.next();
                                    Some(param)
                                },
                                _ => None,
                            }
                        }).collect_vec().into_boxed_slice();

                        match self.tokens.next().ok_or(INVALID_END.to_string())?.t_type {
                            TType::FnArrow => (),
                            token => return Err(format!("{0} {1:?}", INVALID_TOKEN, token)),
                        };

                        let callee = Box::new(self.expression()?);
                        let body = FnBody { params, callee };
                        Ok(Stmt::FnDeclaration { identifier, body })
                    }
                    token => Err(format!("{0} {1:?}", INVALID_TOKEN, token))
                }
            },
            TType::Identifier(var) => {
                match self.tokens.peeking_ahead(2){
                    Some(Token{t_type: TType::Assignment, ..}) => {
                        self.tokens.next_tuple::<(_,_)>();
                        Ok(Stmt::Assignment {identifier: var, value: self.expression()?})
                    },
                    _ => Ok(self.expression()?.into()),
                }
            },
            _ => Ok(self.expression()?.into()),
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>, String> {
        match self.tokens.peek() {
            Some(&Token { t_type: TType::Identifier(func_identifier), .. })
            if self.funcs.contains_key(func_identifier) => {
                self.tokens.next();
                let func = self.funcs.get(func_identifier).unwrap();
                let args: Vec<_> = func.params
                    .iter()
                    .map(|&param| self.term())
                    .try_collect()?;

                Ok(Expr::Fn {identifier: func_identifier, args: args.into_boxed_slice()})
            },
            _ => self.term(),
        }
    }

    // TODO: Create assignment as binary operation, and move common logic to separate method

    fn term(&mut self) -> Result<Expr<'a>, String> {
        let mut left_expr = self.factor()?;
        loop {
            match self.tokens.peek() {
                Some(&Token { t_type: TType::Op(ch @ ('+' | '-') ), .. }) => {
                    self.tokens.next();
                    let left = Box::new(left_expr);
                    let right = Box::new(self.factor()?);
                    left_expr = Expr::Binary(left, ch, right);
                },
                _ => break Ok(left_expr),
            }
        }
    }

    fn factor(&mut self) -> Result<Expr<'a>, String> {
        let mut left_expr = self.unary()?;
        loop {
            match self.tokens.peek() {
                Some(&Token { t_type: TType::Op(ch @ ('*' | '/' | '%')), .. }) => {
                    self.tokens.next();
                    let left = Box::new(left_expr);
                    let right = Box::new(self.unary()?);
                    left_expr = Expr::Binary(left, ch, right);
                },
                _ => break Ok(left_expr),
            }
        }
    }

    fn unary(&mut self) -> Result<Expr<'a>, String> {
        let token = self.tokens.peek().ok_or_else(|| INVALID_END.to_string())?;
        match token.t_type {
            TType::Op(ch @ '-') => {
                self.tokens.next();
                let expr = Box::new(self.unary()?);
                Ok(Expr::Unary(ch, expr))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>, String> {
        let token = self.tokens.next().ok_or_else(|| INVALID_END.to_string())?;
        match token.t_type {
            TType::Num(num) => Ok(Expr::Num(num)),
            TType::Identifier(var) => Ok(Expr::Var(var)),
            TType::LeftParen => {
                let expr = self.expression()?;
                match self.tokens.next() {
                    Some(token) => match token.t_type {
                        TType::RightParen => Ok(Expr::Grouping(Box::new(expr))),
                        _ => Err(format!("{0} index:{1}", INVALID_PAREN, token.index)),
                    },
                    None => Err(INVALID_PAREN.to_string()),
                }
            }
            _ => Err(format!("{0} {1:?}", INVALID_TOKEN, token)),
        }
    }
}

#[rustfmt::skip]
fn scan(input: &str) -> impl Iterator<Item=Token> + Clone + '_{
    input.char_indices().peekable().batching(move |iter|{
        iter.peeking_take_while(|(_, ch)| ch.is_whitespace()).for_each(|_|());
        match iter.next() {
            None => None,
            Some((index, ch)) => {
                let mut last_index_when = |accept: fn(char) -> bool| iter
                    .peeking_take_while(|(_, ch)| accept(*ch))
                    .map(|(index, _)| index)
                    .last()
                    .unwrap_or(index);

                let token = match ch {
                    '0'..='9' => {
                        let last_index = last_index_when(|ch|
                            matches!(ch, '0'..='9' | '.'));

                        let len = 1 + last_index - index;
                        let num = input[index..=last_index].parse::<f32>()
                            .unwrap_or_else(|_| panic!("{0} index:{1}", INVALID_TOKEN, index));

                        Token { index, len, t_type: TType::Num(num) }
                    },
                    '(' => Token { index, len: 1, t_type: TType::LeftParen },
                    ')' => Token { index, len: 1, t_type: TType::RightParen },
                    '+' | '-' | '/' | '*' | '%' => Token { index, len: 1, t_type: TType::Op(ch) },
                    '=' => match iter.next_if(|(_, ch)| *ch == '>') {
                        None => Token { index, len: 1, t_type: TType::Assignment },
                        Some(_) => Token { index, len: 2, t_type: TType::FnArrow }
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let last_index = last_index_when(|ch|
                            matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

                        let len = 1 + last_index - index;
                        let t_type =  match &input[index..=last_index] {
                            "fn" => TType::Fn,
                            ch => TType::Identifier(ch),
                        };

                        Token {index, len, t_type}
                    },
                    _ => panic!("{0} index:{1}, char:{2}",INVALID_TOKEN, index, ch), // TODO: Use errors
                };
                Some(token)
            }
        }
    })
}

trait PeekingAhead: Iterator + Clone { // TODO: add clone bound

    /// Peeking multiple items of simple iterator by exploiting clone
    fn peeking_ahead(&self, leap_size: usize) -> Option<Self::Item>{
        let mut peekable = self.clone();
        (0..leap_size).map(|_| peekable.next()).last().flatten()
    }
}

impl<I> PeekingAhead for Peekable<I>
    where I: Iterator + Clone, Self::Item: Clone {}

#[test]
fn scan_test(){
    let test = "fn avg x y_1 =>    (x - y_1 + 123423434) / 2.343";
    let tokens = scan(test).map(|token| token.t_type);
    let expected_tokens = [
        TType::Fn,
        TType::Identifier("avg"),
        TType::Identifier("x"),
        TType::Identifier("y_1"),
        TType::FnArrow,
        TType::LeftParen,
        TType::Identifier("x"),
        TType::Op('-'),
        TType::Identifier("y_1"),
        TType::Op('+'),
        TType::Num(123423434.0),
        TType::RightParen,
        TType::Op('/'),
        TType::Num(2.343),
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
