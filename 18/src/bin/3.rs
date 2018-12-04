use regex::Regex;
use std::convert::TryFrom;
use aoc18::util::{parse, print_ans};

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = parse("inputs/3")?;
    print_ans(&inputs, f1, f2);
    Ok(())
}

fn f1(inputs: &Vec<String>) -> i32 {
    0
}

fn f2(inputs: &Vec<String>) -> i32 {
    0
}

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl TryFrom<&String> for Claim {
    fn try_from(string: &String) {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): \d+x\d+$").unwrap();
        re.captures(string)
    }
}