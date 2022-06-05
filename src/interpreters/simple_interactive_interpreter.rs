// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750/train/rust
use std::collections::{HashMap};
use itertools::{EitherOrBoth, Itertools,};
use std::iter;
use std::iter::Peekable;
use std::ops::Deref;
use rand::rngs::mock::StepRng;

// TODO: end of string token and make programs bit more difficult
// TODO: errors highlighting
// TODO: add benchmarks

#[derive(Clone, Debug)]
struct Token {
    index: usize,
    len: usize,
    t_type: TType,
}

#[derive(Clone, Debug, PartialEq)]
enum TType {
    Op(char),
    FnArrow,
    LeftParen,
    RightParen,
    Num(f32),
    Fn,
    Identifier(String),
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
    Expr(Expr),
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Stmt { Self::Expr(expr) }
}

#[derive(Clone, Debug)]
enum Identifier{
    Fn(FnBody),
    Var(Vec<StepRng>),
}

#[derive(Clone, Debug)]
struct FnBody {
    params: Box<[String]>,
    callee: Box<Expr>,
}

const INVALID_END: &str = "ERROR: Invalid ending!";
const INVALID_TOKEN: &str = "ERROR: Invalid token!";
const INVALID_PAREN: &str = "ERROR: Invalid paren!";

struct Interpreter {
    vars: HashMap<String, Vec<f32>>,
    funcs: HashMap<String, FnBody>,
}

impl Interpreter {

    fn new() -> Interpreter {
        Self{
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    fn input(& mut self, input: & str) -> Result<Option<f32>, String> {
        let tokens = scan(input);
        if tokens.clone().next().is_none() { // crutch
            return Ok(None);
        }
        let mut parser = Parser::new(tokens, &self.funcs);
        let statement = parser.parse()?;
        self.interpret(statement)
    }

    fn interpret(&mut self, stmt: Stmt) -> Result<Option<f32>, String>{
        match stmt {
            Stmt::FnDeclaration { identifier, body } => {
                self.check_fn_body(&body)?;
                if self.vars.contains_key(&identifier) {
                    Err(format!("ERROR: Name '{0}' already exist", {identifier}))
                }
                else {
                    self.funcs.insert(identifier, body);
                    Ok(None)
                }
            },
            Stmt::Expr(expr) => self.eval(&expr).map(Some),
        }
    }

    // NOTE: no recursion implementation looks pretty:)
    fn check_fn_body(&mut self, fn_body: &FnBody) -> Result<(), String> {
        if fn_body.params.iter().duplicates().count() > 0 {
            return Err(format!("ERROR: Duplicates in function:\n{fn_body:?}"));
        }

        let param_not_exist = |identifier: &String| {
            !fn_body.params.contains(identifier)
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
                        break Err(format!("ERROR: Invalid identifier '{identifier}' in function body")),
                    _ => (),
                },
                None => break Ok(()),
            }
        }
    }

    fn eval(&mut self, expr: &Expr) -> Result<f32, String> {
        match expr {
            Expr::Binary(left, '=', right) =>{
                match left.deref() {
                    Expr::Var(identifier) => {
                        let val = self.eval(right)?;
                        if self.funcs.contains_key(identifier) {
                            Err(format!("ERROR: Name '{identifier}' already exist"))
                        }
                        else {
                            self.vars.insert(identifier.clone(), vec![val]);
                            Ok(val)
                        }
                    },
                    _ => Err(format!("ERROR: Invalid assigment!\n {expr:?}"))
                }
            },
            Expr::Binary(left, ch, right) => match ch {
                '+' => Ok(self.eval(left)? + self.eval(right)?),
                '-' => Ok(self.eval(left)? - self.eval(right)?),
                '/' => Ok(self.eval(left)? / self.eval(right)?),
                '*' => Ok(self.eval(left)? * self.eval(right)?),
                '%' => Ok(self.eval(left)? % self.eval(right)?),
                op => Err(format!("ERROR: Invalid operation! op:{op:?}")),
            },
            Expr::Unary(ch, expr) => Ok(-self.eval(expr)?),
            Expr::Grouping(expr) => self.eval(expr),
            Expr::Num(num) => Ok(*num),
            Expr::Var(fn_identifier) => self.vars.get(fn_identifier)
                .map(|val| val.last())
                .flatten()
                .cloned()
                .ok_or(format!("ERROR: Invalid variable identifier '{fn_identifier:?}'")),
            Expr::Fn {args: expr_args, identifier} => {
                let args: Vec<_> = expr_args.iter()
                    .map(|arg| self.eval(arg))
                    .try_collect()?;

                let func_body = self.funcs.get(identifier)
                    .ok_or(format!("ERROR: Invalid function identifier '{identifier}'"))?;

                let vars: Vec<_> = func_body.params.iter()
                    .zip_longest(args)
                    .map(|arg| {
                        match &arg {
                            &EitherOrBoth::Both(param, arg) => Ok((param, arg)),
                            _ => Err(format!("ERROR: Invalid function '{identifier}' params")),
                        }
                }).try_collect()?;

                for (identifier, value) in vars {
                    self.vars.entry(identifier.into()).or_default().push(value);
                };

                let func_body = func_body.deref().clone();
                let expr = self.eval(&func_body.callee)?;

                for identifier in func_body.params.iter() {
                    self.vars.get_mut(identifier).unwrap().pop();
                }
                Ok(expr)
            },
        }
    }
}

struct Parser<'a, T>
    where T: Iterator<Item=Token> + Clone
{
    tokens: Peekable<T>,
    funcs: &'a HashMap<String, FnBody>,
}

impl<'a,T> Parser<'a, T>
    where T: Iterator<Item=Token> + Clone
{
    fn new(tokens: T, funcs: &'a HashMap<String, FnBody>) -> Self{
        Parser{ tokens: tokens.peekable(), funcs }
    }

    // statement      → "fn" fn_identifier "=>" expression | (identifier "=")? expression ;
    // expression     → term | fn_identifier (expression)*;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → "-" unary | primary ;
    // primary        → "(" expression ")" | identifier | number;
    fn parse(&mut self) -> Result<Stmt, String> {
        let expr = self.stmt();
        return match self.tokens.next() {
            Some(token) => Err(format!("{0} {1:?}", INVALID_TOKEN, token)),
            None => expr,
        };
    }

    fn stmt(&mut self) -> Result<Stmt, String> {
        let token = self.tokens.peek().ok_or(INVALID_END.to_string())?.clone();
        match &token.t_type {
            TType::Fn => {
                self.tokens.next();
                match self.tokens.next().ok_or(INVALID_END.to_string())? {
                    Token { t_type: TType::Identifier(identifier), .. } => {
                        let params = iter::from_fn(||{
                            match self.tokens.next_if(|tkn| matches!(tkn.t_type, TType::Identifier(_)))?.t_type {
                                TType::Identifier(param) => Some(param),
                                _ => None,
                            }
                        }).collect_vec().into_boxed_slice();

                        match self.tokens.next().ok_or(INVALID_END.to_string())?.t_type {
                            TType::FnArrow => (),
                            token => return Err(format!("{INVALID_TOKEN} {token:?}")),
                        };

                        let callee = Box::new(self.expression()?);
                        let body = FnBody { params, callee };
                        Ok(Stmt::FnDeclaration { identifier, body })
                    }
                    token => Err(format!("{INVALID_TOKEN} {token:?}"))
                }
            },
            _ => Ok(self.expression()?.into()),
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        match self.tokens.peek() {
            Some(Token { t_type: TType::Identifier(func_identifier), .. })
            if self.funcs.contains_key(func_identifier) => {
                let func_identifier = func_identifier.clone();
                self.tokens.next();
                let func = self.funcs.get(&func_identifier).unwrap();
                let args: Vec<_> = func.params
                    .iter()
                    .map(|param| self.expression())
                    .try_collect()?;

                Ok(Expr::Fn {identifier: func_identifier, args: args.into_boxed_slice()})
            },
            _ => self.assignment(),
        }
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let mut left_expr = self.term()?;
        let mut expr_stack = vec![left_expr];
        loop {
            match self.tokens.peek() {
                Some(&Token { t_type: TType::Op(ch @ '='), .. }) => {
                    self.tokens.next();
                    expr_stack.push(self.term()?);
                },
                _ => break expr_stack.into_iter().rev().reduce(|right, left| {
                    Expr::Binary(
                        Box::new(left),
                        '=',
                        Box::new(right))
                    }).ok_or_else(|| "".to_string()),
            }
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
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

    fn factor(&mut self) -> Result<Expr, String> {
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

    fn unary(&mut self) -> Result<Expr, String> {
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

    fn primary(&mut self) -> Result<Expr, String> {
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
            _ => Err(format!("{INVALID_TOKEN} {token:?}")),
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
                        None => Token { index, len: 1, t_type: TType::Op('=') },
                        Some(_) => Token { index, len: 2, t_type: TType::FnArrow }
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let last_index = last_index_when(|ch|
                            matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

                        let len = 1 + last_index - index;
                        let t_type =  match &input[index..=last_index] {
                            "fn" => TType::Fn,
                            ch => TType::Identifier(ch.to_string()),
                        };

                        Token {index, len, t_type}
                    },
                    _ => panic!("{INVALID_TOKEN} index:{index}, char:{ch}"), // TODO: Use errors
                };
                Some(token)
            }
        }
    })
}

trait PeekingAhead: Iterator + Clone {

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
    let test = "fn avg x y_1 =>    (x = y_1 + 123423434) / 2.343";
    let tokens = scan(test).map(|token| token.t_type);
    let expected_tokens = [
        TType::Fn,
        TType::Identifier("avg".to_string()),
        TType::Identifier("x".to_string()),
        TType::Identifier("y_1".to_string()),
        TType::FnArrow,
        TType::LeftParen,
        TType::Identifier("x".to_string()),
        TType::Op('='),
        TType::Identifier("y_1".to_string()),
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

#[test]
fn chained_nested_assignments(){
    let mut i = Interpreter::new();
    assert_eq!(i.input("x = y = 713"), Ok(Some(713.0)));
    assert_eq!(i.input("x = 29 + (y = 11)"), Ok(Some(40.0)));
}

#[test]
fn nested_functions(){
    let mut i = Interpreter::new();
    assert_eq!(i.input("fn f a b => a * b"), Ok(None));
    assert_eq!(i.input("fn g a b c => a * b * c"), Ok(None));
    assert_eq!(i.input("g g 1 2 3 f 4 5 f 6 7"), Ok(Some(5040.0)));
}

#[test]
fn invalid_function_args(){
    let mut i = Interpreter::new();
    assert!(i.input("fn add x x => x + x").is_err());

    let mut i = Interpreter::new();
    i.input("x = 23");
    i.input("x = 25");
    i.input("z = 0");
    assert!(i.input("fn add x y => x + z").is_err());
}
