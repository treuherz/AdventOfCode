use aoc19::intcode::{parse_memory, Computer};
use aoc19::util::print_answers;

fn main() -> anyhow::Result<()> {
    let inputs = parse_memory("inputs/5")?;
    print_answers(5, &inputs, part1, part2);
    Ok(())
}

fn part1(input: &[i64]) -> i64 {
    let mut mem = Computer::new(input);
    let output = mem.run_on(Some(1));
    assert!(output.iter().take(output.len() - 2).all(|&n| n == 0));
    *output.last().unwrap()
}

fn part2(input: &[i64]) -> i64 {
    let mut mem = Computer::new(input);
    let output = mem.run_on(Some(5));
    output[0]
}
