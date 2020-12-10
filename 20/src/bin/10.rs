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
    let (_, (jumps1, jumps3)) = inputs.iter().sorted().fold((0, (0, 0)), |(prev, (jumps1, jumps3)), &n| {
        match n - prev {
            1 => (n, (jumps1 + 1, jumps3)),
            2 => (n, (jumps1, jumps3)),
            3 => (n, (jumps1, jumps3 + 1)),
            _ => panic!("no appropriate adapter")
        }
    });
    // jumps3 + 1 to account for jump to device
    jumps1 * (jumps3 + 1)
}

fn part2(inputs: &[u64]) -> u128 {
    let start = 0;
    let end = inputs.iter().max().unwrap() + 3;
    let mut inputs: Vec<u64> = [&[start], inputs, &[end]].concat();
    inputs.sort();

    inputs.iter().spl
    let mut acc = 1u128;
    for [a, b, c, d] in inputs.array_windows().step_by(2) {
        // We don't count the option of skipping just c, as that will be picked up on the next pass.
        // The only one that wouldn't be is the penultimate entry, but as we know the end is
        // 3 higher than the penultimate, we wouldn't be able to skip it anyway.
        acc *= match (b - a, c - a, d - a) {
            (1, 2, 3) => 8, // could skip (b) or (b and c) or (c)
            (1, 2, 4) => 4, // could skip b or c
            (1, 2, 5) => 2, // could skip b,
            (1, 3, 4) => 4, // could skip b or c,
            (1, 3, 5) => 2, // could skip b
            (1, 3, 6) => 1,
            (1, 4, _) => 1,
            (2, 3, 4) => 4, // could skip b or c,
            (2, 3, 5) => 4, // could skip b or c
            (2, 3, 6) => 2, // could skip b,
            (2, 4, 5) => 2, // could skip c,
            (2, 4, 6) => 1,
            (2, 4, 7) => 1,
            (2, 5, _) => 1,
            (3, 4, 5) => 2, // could skip c
            (3, 4, 6) => 2, // could skip c
            (3, 4, 7) => 1,
            (3, 5, 6) => 2, // could skip c
            (3, 5, 7) => 1,
            (3, 5, 8) => 1,
            (3, 6, _) => 1,
            _ => panic!("unexpected window ({}, {}, {})", b-a, c-a, d-a),
        }
    }

    inputs.iter().batching(|it| {it.})

    let gaps = inputs.iter()
        .tuple_windows()
        .enumerate()
        .filter(|(_, (&a, &b))| a + 3 == b)
        .map(|(i, _)| i)
        .collect();




    acc
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let inputs = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        assert_eq!(super::part2(&inputs), 19208);
    }
}