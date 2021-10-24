// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750/train/rust

// NOTE: here we have two types of statements: Function, and

#[derive(Clone, Debug)]
struct Token{
    index: usize,
    len: usize,
    t_type: TType,
}

#[derive(Clone, Debug)]
enum TType{
    Op(OpType),
    LeftParen,
    RightParen,
    Num(f32),
    Fn,
    Identifier(String), // TODO: maybe use &str
}

#[derive(Clone, Debug)]
enum OpType {
    Mult(char),
    Add(char),
    Assignment,
    Fn,
}

#[derive(Clone, Debug)]
enum Expr{
    Binary(Box<Expr>, OpType, Box<Expr>),
    Unary(OpType, Box<Expr>),
    Grouping,
    Num(f32),
    Fn {
        callee: Box<Expr>, // TODO: maybe put as separate type
        args: Box<[Expr]>,
    }
}

enum Stmt{
    FnDeclaration,
    Assignment,
    Output,
}

struct Interpreter {}

impl Interpreter {

    fn new() -> Interpreter {
        unimplemented!()
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        unimplemented!()
    }
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