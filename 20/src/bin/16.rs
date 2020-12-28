#![feature(iterator_fold_self)]

use aoc20::util::print_answers;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::{digit1, line_ending},
    combinator::map,
    combinator::map_res,
    multi::many1,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};
use std::ops::Not;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    ops::RangeInclusive,
    str::FromStr,
};

fn main() -> anyhow::Result<()> {
    let lines: String = std::fs::read_to_string("inputs/16")?;
    let input: Input = parse_input(&lines).finish().unwrap().1;
    print_answers(16, &input, part1, part2);
    Ok(())
}

#[derive(Debug)]
struct Input {
    ranges: HashMap<String, [RangeInclusive<u64>; 2]>,
    mine: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, ranges) = map(
        many1(map(
            tuple((
                take_until(":"),
                tag(": "),
                map_res(digit1, u64::from_str),
                tag("-"),
                map_res(digit1, u64::from_str),
                tag(" or "),
                map_res(digit1, u64::from_str),
                tag("-"),
                map_res(digit1, u64::from_str),
                many1(line_ending),
            )),
            |(name, _, a_from, _, a_to, _, b_from, _, b_to, _)| {
                (name.to_string(), [a_from..=a_to, b_from..=b_to])
            },
        )),
        HashMap::from_iter,
    )(input)?;

    let (input, mine) = map(
        tuple((
            tag("your ticket:"),
            line_ending,
            separated_list1(tag(","), map_res(digit1, u64::from_str)),
            many1(line_ending),
        )),
        |(_, _, mine, _)| mine.iter().copied().collect(),
    )(input)?;

    let (input, nearby) = map(
        tuple((
            tag("nearby tickets:"),
            line_ending,
            separated_list1(
                line_ending,
                separated_list1(tag(","), map_res(digit1, u64::from_str)),
            ),
        )),
        |(_, _, v)| v.iter().map(|v| v.iter().copied().collect()).collect(),
    )(input)?;

    Ok((
        input,
        Input {
            ranges,
            mine,
            nearby,
        },
    ))
}

fn part1(input: &Input) -> u64 {
    input
        .nearby
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&n| {
            input
                .ranges
                .values()
                .any(|[a, b]| a.contains(n) || b.contains(n))
                .not()
        })
        .sum()
}

fn part2(input: &Input) -> u64 {
    let valid_nearby: Vec<Vec<u64>> = input
        .nearby
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|n| {
                input
                    .ranges
                    .values()
                    .any(|[a, b]| a.contains(n) || b.contains(n))
            })
        })
        .cloned()
        .collect();

    let mut possible_fields: Vec<HashSet<String>> = (0..input.mine.len())
        .map(|idx| {
            valid_nearby
                .iter()
                .map(|ticket| {
                    let n = ticket[idx];
                    input
                        .ranges
                        .iter()
                        .filter(|(name, [a, b])| a.contains(&n) || b.contains(&n))
                        .map(|(name, _)| name)
                        .cloned()
                        .collect::<HashSet<String>>()
                })
                .fold_first(|ref a, ref b| a & b)
                .unwrap()
        })
        .collect();

    let mut assigned: HashSet<String> = HashSet::new();
    while possible_fields.iter().any(|set| set.len() > 1) {
        possible_fields
            .iter_mut()
            .filter(|set| set.len() != 1)
            .for_each(|set| *set = set.difference(&assigned).cloned().collect());
        possible_fields
            .iter()
            .filter(|set| set.len() == 1)
            .for_each(|set| assigned = &assigned | set);
    }

    let departure_idxs: Vec<usize> = possible_fields
        .iter()
        .enumerate()
        .filter(|&(idx, name)| name.iter().next().unwrap().starts_with("departure"))
        .map(|(idx, _)| idx)
        .collect();

    departure_idxs.iter().map(|&idx| input.mine[idx]).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}

    #[test]
    fn part2() {}
}
