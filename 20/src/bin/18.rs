use std::{ops::Not, time::Instant};

use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/18")?;
    print_answers(18, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

enum Op {
    Add,
    Multiply,
}

#[derive(Copy, Clone)]
enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Times,
    Num(u64),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' => Token::Plus,
            '*' => Token::Times,
            n if n.is_ascii_digit() => Token::Num(n.to_digit(10).unwrap() as u64),
            _ => panic!("unrecognised token"),
        }
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::OpenParen => '(',
            Token::CloseParen => ')',
            Token::Plus => '+',
            Token::Times => '*',
            Token::Num(n) => n.to_string().chars().next().unwrap(),
        }
    }
}

fn tokens(input: &str) -> Vec<Token> {
    input
        .chars()
        .filter(|c| c.is_whitespace().not())
        .map(|c| c.into())
        .collect()
}

struct Calculator {
    expr: String,
    acc_stack: Vec<Option<u64>>,
    op_stack: Vec<Option<Op>>,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            expr: String::new(),
            acc_stack: vec![None],
            op_stack: vec![None],
        }
    }

    fn consume(&mut self, tok: Token) {
        match tok {
            Token::OpenParen => {
                self.acc_stack.push(None);
                self.op_stack.push(None);
            }
            Token::CloseParen => {
                let n = self.acc_stack.pop().flatten().unwrap();
                self.apply(n)
            }
            Token::Plus => {
                self.op_stack.push(Some(Op::Add));
            }
            Token::Times => {
                self.op_stack.push(Some(Op::Multiply));
            }
            Token::Num(n) => self.apply(n),
        };
        self.expr.push(tok.into())
    }

    fn apply(&mut self, n: u64) {
        let acc = self.acc_stack.last_mut().unwrap();
        match self.op_stack.pop().unwrap() {
            None => {
                acc.replace(n);
            }
            Some(Op::Add) => *acc.as_mut().unwrap() += n,
            Some(Op::Multiply) => *acc.as_mut().unwrap() *= n,
        }
    }

    fn done(mut self) -> Option<u64> {
        self.acc_stack.pop().unwrap()
    }
}

fn part1(inputs: &[String]) -> u64 {
    let mut acc = 0;
    for s in inputs {
        let mut calc = Calculator::new();
        tokens(&s).iter().for_each(|&tok| {
            calc.consume(tok);
            dbg!(&calc.expr);
        });
        acc += calc.done().unwrap()
    }
    acc
}

fn part2(inputs: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let cases = vec![
            (vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()], 71),
            (vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()], 51),
        ];

        for (input, expected) in cases {
            assert_eq!(super::part1(&input), expected);
        }
    }
}
