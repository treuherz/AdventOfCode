use std::collections::HashSet;

use aoc18::util::{parse, print_ans};

fn main() -> std::io::Result<()> {
    let inputs: Vec<i32> = parse("inputs/1")?;
    print_ans(&inputs, f1, f2);
    Ok(())
}

fn f1(inputs: &Vec<i32>) -> i32 {
    inputs.iter().sum()
}

fn f2(inputs: &Vec<i32>) -> i32 {
    let l = inputs.iter().cycle();
    let mut seen = HashSet::new();
    let mut cur: i32 = 0;
    for i in l {
        cur += i;
        if seen.contains(&cur) {
            break;
        } else {
            seen.insert(cur);
        }
    }
    cur
}
