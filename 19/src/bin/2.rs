use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};
use std::io::SeekFrom::Start;
use std::ops::Deref;
use std::str::FromStr;

use aoc19::util::print_ans;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let inputs: Vec<usize> = parse_commas("inputs/2")?;
    print_ans(&inputs, f1, f2);
    Ok(())
}

fn parse_commas(path: &str) -> Result<Vec<usize>> {
    let s = std::fs::read_to_string(path)?;
    let mut out = Vec::new();
    for i in s.trim_end().split(',') {
        out.push(i.parse()?);
    }
    Ok(out)
}

fn f1(inputs: &Vec<usize>) -> usize {
    let mut mem = inputs.clone();
    mem[1] = 12;
    mem[2] = 2;
    let mut c = Computer {
        mem: mem,
    };
    let out = c.run();
    out[0]
}

//    let mut mem = inputs.to_owned();
//    mem[1] = 12;
//    mem[2] = 2;
//
//    let mut cur = 0;
//    loop {
//        match mem[cur] {
//            99 => break,
//            i @ 1 | i @ 2 => {
//                cur += 1;
//                let in1 = mem[cur];
//                cur += 1;
//                let in2 = mem[cur];
//                cur += 1;
//                let out = mem[cur];
//                match i {
//                    1 => mem[out] = mem[in1] + mem[in2],
//                    2 => mem[out] = mem[in1] * mem[in2],
//                    _ => unreachable!(),
//                };
//                cur += 1;
//            }
//            _ => panic!(),
//        }
//    }
//
//    mem[0]

fn f2(inputs: &Vec<usize>) -> usize {
    unimplemented!()
}

struct Computer {
    mem: Vec<usize>,
}

impl Computer {
    fn run(&self) -> Vec<usize> {
        let mut mem = self.mem.clone();
        let mut cur = 0;
        loop {
            match mem[cur] {
                99 => break,
                n @ _ => match Operation::from_code(&n) {
                    None => panic!(n),
                    Some(op) => {
                        let (idx, val) = op.run(&cur, &mem);
                        mem[idx] = val;
                        cur += op.params() + 1;
                    }
                },
            }
        }
        return mem;
    }
}

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn from_code(code: &usize) -> Option<Operation> {
        use Operation::*;
        match code {
            1 => Some(Add),
            2 => Some(Mul),
            _ => None,
        }
    }

    fn params(&self) -> usize {
        use Operation::*;
        match self {
            Add => 3,
            Mul => 3,
        }
    }

    fn apply(&self, params: Vec<usize>, mem: &Vec<usize>) -> (usize, usize) {
        match self {
            Operation::Add => {
                let (in1, in2, out) = (params[0], params[1], params[2]);
                (out, mem[in1] + mem[in2])
            }
            Operation::Mul => {
                let (in1, in2, out) = (params[0], params[1], params[2]);
                (out, mem[in1] * mem[in2])
            }
        }
    }

    fn read_params(&self, cur: &usize, mem: &Vec<usize>) -> Vec<usize> {
        let count = self.params();
        let mut v = Vec::with_capacity(count);
        for i in 1..=count {
            v.push(mem[cur + i]);
        }
        v
    }

    fn run(&self, cur: &usize, mem: &Vec<usize>) -> (usize, usize) {
        let params = self.read_params(cur, mem);
        self.apply(params, mem)
    }
}
