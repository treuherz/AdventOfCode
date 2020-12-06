use aoc19::intcode::{parse_memory, Computer};
use aoc19::util::print_answers;
use std::error::Error;

fn main() -> anyhow::Result<()> {
    let inputs = parse_memory("inputs/2")?;
    print_answers(2, &inputs, f1, f2);
    Ok(())
}

fn f1(input: &[i64]) -> i64 {
    let mut mem = Computer::new(input);
    mem.set(1, 12);
    mem.set(2, 2);
    mem.run();
    mem.get(0)
}

fn f2(input: &[i64]) -> i64 {
    const GOAL: i64 = 19690720;
    let max = input.len() as i64;
    for noun in 0..max {
        for verb in 0..max {
            let mut mem = Computer::new(input);
            mem.set(1, noun);
            mem.set(2, verb);
            mem.run();
            if mem.get(0) == GOAL {
                return 100 * noun + verb;
            }
        }
    }
    panic!("never found an answer")
}
