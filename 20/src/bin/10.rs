#![feature(array_windows)]

use std::time::Instant;

use itertools::Itertools;

use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<u64> = parse("inputs/10")?;
    print_answers(10, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

fn part1(inputs: &[u64]) -> u64 {
    let (_, (jumps1, jumps3)) =
        inputs
            .iter()
            .sorted()
            .fold((0, (0, 0)), |(prev, (jumps1, jumps3)), &n| match n - prev {
                1 => (n, (jumps1 + 1, jumps3)),
                2 => (n, (jumps1, jumps3)),
                3 => (n, (jumps1, jumps3 + 1)),
                _ => panic!("no appropriate adapter"),
            });
    // jumps3 + 1 to account for jump to device
    jumps1 * (jumps3 + 1)
}

fn part2(inputs: &[u64]) -> u64 {
    let start = 0;
    let end = inputs.iter().max().unwrap() + 3;
    let mut inputs: Vec<u64> = [&[start], inputs, &[end]].concat();
    inputs.sort();

    let mut paths: Vec<u64> = vec![1, 1];
    if inputs[2] - inputs[0] <= 3 {
        paths.push(2)
    } else {
        paths.push(1)
    }

    for ((a_ix, a), (b_ix, b), (c_ix, _), (_, d)) in inputs.iter().enumerate().tuple_windows() {
        let mut acc = paths[c_ix];
        if d - b <= 3 {
            acc += paths[b_ix];
        }
        if d - a <= 3 {
            acc += paths[a_ix];
        }
        paths.push(acc)
    }

    *paths.last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let inputs = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(super::part2(&inputs), 19208);
    }
}
