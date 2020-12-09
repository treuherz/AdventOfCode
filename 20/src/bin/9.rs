use itertools::Itertools;

use aoc20::util::{parse, print_answers};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<u64> = parse("inputs/9")?;
    print_answers(9, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

fn part1(inputs: &[u64]) -> u64 {
    *inputs.windows(26).find(|&w| {
        let target = *w.last().unwrap();

        for (i, &n) in w.iter().enumerate() {
            let res = w[i..].iter().find(|&&m| n + m == target);
            if res.is_some() {
                return false;
            }
        }
        true
    })
        .and_then(|w| w.last())
        .unwrap()
}

fn part2(inputs: &[u64]) -> u64 {
    let target = part1(inputs);

    'run_from: for mut i in (0..inputs.len() - 1) {
        'run_to: for j in (i + 2..inputs.len()) {
            if inputs[j] >= target {
                i = j +1;
                continue 'run_from;
            }
            let sum: u64 = inputs[i..j].iter().sum();
            if sum == target {
                let (min, max) = inputs[i..j].iter().minmax().into_option().unwrap();
                return min + max
            }
            if sum > target {
                // Overshot, skip to the next starting point
                continue 'run_from;
            }
        }
    }
    panic!("didn't find answer")
}
