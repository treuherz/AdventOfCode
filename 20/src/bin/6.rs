#![feature(iterator_fold_self)]

use aoc20::util::{parse, print_answers};
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/6")?;
    print_answers(6, &inputs, part1, part2);
    Ok(())
}

/// Count number of characters appearing in any line for each group
fn part1(inputs: &[String]) -> usize {
    inputs
        // Group the input on empty lines
        .split(|s| s.is_empty())
        // For each group...
        .map(|g| {
            g.iter()
                // Collect all the characters
                .flat_map(|s| s.chars())
                // Count the distinct ones
                .unique()
                .count()
        })
        // Add up the counts
        .sum()
}

/// Count characters appearing in all lines for each group
fn part2(inputs: &[String]) -> usize {
    inputs
        // Group the input on empty lines
        .split(|s| s.is_empty())
        // For each group...
        .map(|g| {
            g.iter()
                .map(|s| s.chars().collect::<HashSet<char>>())
                // ...find intersection of characters in all lines,
                .fold_first(|prev, cur| prev.intersection(&cur).copied().collect())
                .unwrap()
                // get the size of the remaining set
                .len()
        })
        // Add up the counts
        .sum()
}
