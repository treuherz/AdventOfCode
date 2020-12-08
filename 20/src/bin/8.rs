#![feature(str_split_once)]

use aoc20::util::{parse, print_answers};
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/8")?;
    let ops: Vec<Op> = inputs.iter().map(|s| parse_op(s)).collect();
    print_answers(8, &ops, part1, part2);
    Ok(())
}

enum Op {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn parse_op(s: &str) -> Op {
    let (code, numstr) = s.split_once(' ').unwrap();
    let num: i32 = numstr.parse().unwrap();
    match code {
        "jmp" => Op::Jmp(num),
        "acc" => Op::Acc(num),
        "nop" => Op::Nop(num),
        _ => panic!("unknown code {}", code),
    }
}

fn part1(inputs: &[Op]) -> i32 {
    let mut cur: usize = 0;
    let mut acc: i32 = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    while !seen.contains(&cur) {
        seen.insert(cur);
        match inputs[cur] {
            Op::Jmp(n) => {
                cur = ((cur as i32) + n) as usize;
            }
            Op::Acc(n) => {
                acc += n;
                cur += 1;
            }
            Op::Nop(_) => {
                cur += 1;
            }
        }
    }
    acc
}

fn part2(inputs: &[Op]) -> i32 {
    // Simulate computer to build up the seen list.
    let mut cur: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    while !seen.contains(&cur) {
        seen.insert(cur);
        match inputs[cur] {
            Op::Jmp(n) => {
                cur = ((cur as i32) + n) as usize;
            }
            Op::Acc(_) => {
                cur += 1;
            }
            Op::Nop(_) => {
                cur += 1;
            }
        }
    }

    let target = inputs.len();

    for ix in seen {
        let mut cur = 0;
        let mut acc: i32 = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        while !seen.contains(&cur) && cur != target{
            seen.insert(cur);
            match inputs[cur] {
                Op::Jmp(n) => {
                    if cur == ix {
                        cur += 1
                    } else {
                        cur = ((cur as i32) + n) as usize;
                    }
                }
                Op::Acc(n) => {
                    acc += n;
                    cur += 1;
                }
                Op::Nop(n) => {
                    if cur == ix && n != 0 {
                        cur = ((cur as i32) + n) as usize;
                    } else {
                        cur += 1;
                    }
                }
            }
        }
        if cur == target {
            return acc
        }
    }
    panic!("never found answer")
}
