use aoc19::computer;
use aoc19::computer::Memory;
use aoc19::util::print_answers;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let inputs: Vec<usize> = parse_commas("inputs/2")?;
    print_answers(&inputs, f1, f2);
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

fn f1(input: &Vec<usize>) -> usize {
    let mem = Memory::new(input);
    mem.set(1, 12);
    mem.set(2, 2);
    computer::run(mem).get(0)
}

fn f2(input: &Vec<usize>) -> usize {
    const GOAL: usize = 19690720;
    let max = input.len();
    for noun in 0..max {
        for verb in 0..max {
            let mem = Memory::new(input);
            mem.set(1, noun);
            mem.set(2, verb);
            let out = computer::run(mem).get(0);
            if out == GOAL {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}
