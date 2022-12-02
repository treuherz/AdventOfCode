#![feature(box_syntax)]
#![feature(box_patterns)]

use std::{
    ops::Not,
    time::Instant,
    fmt::{Debug,},
    convert::TryInto,
};

use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/18")?;
    print_answers(18, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Multiply,
    Add,
}

#[derive(Debug)]
enum Node {
    Op {
        op: Op,
        children: (Box<Node>, Box<Node>),
    },
    Num {
        n: u64,
    },
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("unexpected end of input")]
    EndOfInput,
    #[error("token {0:?} in unexpected position")]
    UnexpectedToken(Token),
}
type Result<'a> = std::result::Result<(&'a [Token], Node), ParseError>;

struct LeftAssoc {}

impl LeftAssoc {
    fn parse_atom(toks: &[Token]) -> Result {
        match toks.first() {
            Some(Token::OpenParen) => {
                let (rem, inner) = Self::parse(&toks[1..])?;
                Ok((&rem[1..], inner))
            }
            Some(Token::Num(n)) => Ok((&toks[1..], Node::Num { n: *n })),
            Some(tok) => Err(ParseError::UnexpectedToken(*tok)),
            None => Err(ParseError::EndOfInput),
        }
    }

    fn parse_right(left: Node, toks: &[Token]) -> Result {
        match toks.first() {
            Some(tok @ Token::Plus | tok @ Token::Times) => {
                let (rem, right) = Self::parse_atom(&toks[1..])?;
                Ok((
                    rem,
                    Node::Op {
                        op: match tok {
                            Token::Plus => Op::Add,
                            Token::Times => Op::Multiply,
                            _ => unreachable!(),
                        },
                        children: (Box::new(left), Box::new(right)),
                    },
                ))
            }
            Some(tok) => Err(ParseError::UnexpectedToken(*tok)),
            None => Err(ParseError::EndOfInput),
        }
    }

    fn parse(toks: &[Token]) -> Result {
        let (mut rem, mut left) = Self::parse_atom(toks)?;
        loop {
            let (new_rem, node) = Self::parse_right(left, rem)?;

            rem = new_rem;
            left = node;

            if rem.is_empty() || *rem.first().unwrap() == Token::CloseParen {
                break;
            }
        }
        Ok((rem, left))
    }
}

struct AddMult {}
impl AddMult {
    fn prec(op: Op) -> u8 {
        match op {
            Op::Multiply => 0,
            Op::Add => 1,
        }
    }

    fn parse_atom(toks: &[Token]) -> Result {
        match toks.first() {
            Some(Token::OpenParen) => {
                let (rem, inner) = Self::parse(&toks[1..])?;
                Ok((&rem[1..], inner))
            }
            Some(Token::Num(n)) => Ok((&toks[1..], Node::Num { n: *n })),
            Some(tok) => Err(ParseError::UnexpectedToken(*tok)),
            None => Err(ParseError::EndOfInput),
        }
    }

    fn parse_expr(mut left: Node, mut toks: &[Token], prec: u8) -> Result {
        while let Some(op) = toks.first().and_then(|tok| tok.try_into().ok()) {
            if Self::prec(op) < prec {
                break;
            }
            let (rem, mut right) = Self::parse_atom(&toks[1..])?;
            toks = rem;

            while let Some(op2) = toks.first().and_then(|tok| tok.try_into().ok()) {
                if Self::prec(op2) <= Self::prec(op) {
                    break;
                }
                let (rem, right2) = Self::parse_expr(right, &toks, Self::prec(op2))?;
                toks = rem;
                right = right2;
            }

            left = Node::Op {
                op,
                children: (Box::new(left), Box::new(right)),
            }
        }

        Ok((toks, left))
    }

    fn parse(toks: &[Token]) -> Result {
        let (mut rem, mut left) = Self::parse_atom(toks)?;
        loop {
            let (new_rem, node) = Self::parse_expr(left, rem, 0)?;

            rem = new_rem;
            left = node;

            if rem.is_empty() || *rem.first().unwrap() == Token::CloseParen {
                break;
            }
        }
        Ok((rem, left))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

impl TryInto<Op> for &Token {
    type Error = Token;

    fn try_into(self) -> std::result::Result<Op, Token> {
        match self {
            Token::Plus => Ok(Op::Add),
            Token::Times => Ok(Op::Multiply),
            _ => Err(*self),
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

impl Node {
    fn evaluate(mut self: Node) -> Node {
        loop {
            self = match self {
                Node::Num { .. } => break self,
                Node::Op {
                    op,
                    children: (x, y),
                } => match (x, y) {
                    (box Node::Num { n: x }, box Node::Num { n: y }) => Node::Num {
                        n: match op {
                            Op::Add => x + y,
                            Op::Multiply => x * y,
                        },
                    },
                    (x @ box Node::Op { .. }, y) => Node::Op {
                        op,
                        children: (Box::new(x.evaluate()), y),
                    },
                    (x, y @ box Node::Op { .. }) => Node::Op {
                        op,
                        children: (x, Box::new(y.evaluate())),
                    },
                },
            }
        }
    }
}

fn part1(inputs: &[String]) -> u64 {
    let mut acc = 0;
    for s in inputs {
        let toks = tokens(&s);
        let root = LeftAssoc::parse(&toks).unwrap().1;
        if let Node::Num { n } = root.evaluate() {
            acc += n;
        } else {
            panic!("tree didn't evaluate fully")
        }
    }
    acc
}

fn part2(inputs: &[String]) -> u64 {
    let mut acc = 0;
    for s in inputs {
        let toks = tokens(&s);
        let root = AddMult::parse(&toks).unwrap().1;
        if let Node::Num { n } = root.evaluate() {
            acc += n;
        } else {
            panic!("tree didn't evaluate fully")
        }
    }
    acc
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

    #[test]
    fn part2() {
        let cases = vec![
            (vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()], 231),
            (vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()], 51),
        ];

        for (input, expected) in cases {
            assert_eq!(super::part2(&input), expected);
        }
    }
}
