#![feature(iterator_fold_self)]

use std::time::Instant;

use aoc20::util::{parse, print_answers};
use itertools::Itertools;
use num::Integer;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/13")?;
    assert_eq!(inputs.len(), 2);
    print_answers(13, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

fn part1(inputs: &[String]) -> u64 {
    let go_time: u64 = inputs[0].parse().unwrap();
    let buses: Vec<u64> = inputs[1]
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let to_wait = |id: u64| id - (go_time % id);

    let bus = *buses.iter().min_by_key(|&&id| to_wait(id)).unwrap();

    bus * to_wait(bus)
}

fn part2(inputs: &[String]) -> usize {
    let buses: Vec<(usize, usize)> = inputs[1]
        .split(',')
        .map(|s| s.parse::<usize>().ok())
        .enumerate()
        .filter(|(_, o)| o.is_some())
        .map(|(idx, o)| (idx, o.unwrap()))
        .sorted_by_key(|(_, id)| *id)
        .rev()
        .collect();

    let (max_id_idx, max_id) = *buses.first().unwrap();

    let mut t = max_id - max_id_idx;
    let mut step;
    loop {
        let matched: Vec<(usize, usize)> = buses
            .iter()
            .take_while(|(idx, id)| (t + idx) % id == 0)
            .copied()
            .collect();
        if matched.len() == buses.len() {
            break t;
        }

        step = matched
            .iter()
            .map(|(_, id)| *id)
            .fold_first(|a, b| a.lcm(&b))
            .unwrap();

        t += step;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let input = vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()];
        assert_eq!(super::part2(&input), 1068781)
    }
}
