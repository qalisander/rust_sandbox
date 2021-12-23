// https://www.codewars.com/kata/5265b0885fda8eac5900093b/train/rust

enum Ast {
    // your code
}

struct Compiler {
    // your code
}

impl Compiler {
    fn new() -> Compiler {
        Compiler { }
    }

    fn tokenize<'a>(&self, program : &'a str) -> Vec<String> {
        let mut tokens : Vec<String> = vec![];

        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a'..='z'|'A'..='Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    },
                    '0'..='9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    },
                    ' ' => { iter.next(); },
                    _ => {
                        tokens.push(iter.next().unwrap().to_string());
                    },
                },
                None => break
            }
        }

        tokens
    }

    fn compile(&mut self, program : &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn pass1(&mut self, program : &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut iter = tokens.iter().peekable();
        todo!()
    }

    fn pass2(&mut self, ast : &Ast) -> Ast {
        todo!()
    }

    fn pass3(&mut self, ast : &Ast) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulator() {
        assert_eq!(simulate(vec!["IM 7".to_string()], vec![3]), 7);
        assert_eq!(simulate(vec!["AR 1".to_string()], vec![1,2,3]), 2);
    }

    fn simulate(assembly : Vec<String>, argv : Vec<i32>) -> i32 {
        let mut r = (0, 0);
        let mut stack : Vec<i32> = vec![];

        for ins in assembly {
            let mut ws = ins.split_whitespace();
            match ws.next() {
                Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
                Some("AR") => r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize],
                Some("SW") => r = (r.1,r.0),
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
