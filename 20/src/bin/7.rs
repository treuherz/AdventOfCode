use std::{collections::hash_map::DefaultHasher, collections::HashSet, hash::Hasher, str::FromStr};

use aoc20::util::{parse, print_answers};
use bimap::BiMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::separated_pair,
    sequence::{terminated, tuple},
};
use petgraph::{
    graph::NodeIndex,
    visit::EdgeRef,
    Directed,
    Direction::{Incoming, Outgoing},
    Graph,
};

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/7")?;
    print_answers(7, &inputs, part1, part2);
    Ok(())
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Colour(String, String);

impl Colour {
    fn new(descriptor: &str, name: &str) -> Colour {
        Colour(descriptor.to_string(), name.to_string())
    }
}

type BagGraph = Graph<Colour, u64, Directed>;

fn parse_graph(inputs: &[String]) -> (BagGraph, BiMap<Colour, NodeIndex>) {
    let mut g: BagGraph = Graph::with_capacity(inputs.len(), inputs.len());
    let mut lookup = BiMap::with_capacity(inputs.len());

    let lines: Vec<Line> = inputs
        .iter()
        .map(|s| line(s).unwrap())
        .map(|(_, r)| r)
        .collect();

    lines
        .iter()
        .flat_map(|(container, links)| {
            links
                .iter()
                .map(move |(weight, contained)| (container.clone(), contained.clone(), weight))
        })
        .for_each(|(container, contained, &weight)| {
            let container_ix = lookup.get_by_left(&container).cloned().unwrap_or_else(|| {
                let ix = g.add_node(container.clone());
                lookup.insert(container, ix);
                ix
            });
            let contained_ix = lookup.get_by_left(&contained).cloned().unwrap_or_else(|| {
                let ix = g.add_node(contained.clone());
                lookup.insert(contained, ix);
                ix
            });
            g.add_edge(container_ix, contained_ix, weight);
        });

    (g, lookup)
}

fn part1(inputs: &[String]) -> usize {
    let (g, lookup) = parse_graph(inputs);

    let ix = lookup
        .get_by_left(&Colour::new("shiny", "gold"))
        .cloned()
        .unwrap();

    let mut stack: Vec<NodeIndex> = vec![ix];
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    while !stack.is_empty() {
        let cur = *stack.last().unwrap();
        visited.insert(cur);
        let next = g
            .neighbors_directed(cur, Incoming)
            .find(|ix| !visited.contains(ix));
        match next {
            None => {
                stack.pop();
            }
            Some(n) => {
                stack.push(n);
            }
        };
    }

    visited.len() - 1
}

fn part2(inputs: &[String]) -> u64 {
    let (g, lookup) = parse_graph(inputs);

    let ix = lookup
        .get_by_left(&Colour::new("shiny", "gold"))
        .cloned()
        .unwrap();

    let mut stack: Vec<(u64, u64, NodeIndex)> = vec![(1, 0, ix)];
    let mut traversed: HashSet<u64> = HashSet::new();
    let mut acc = 0;
    while !stack.is_empty() {
        let (mul, hash, cur) = *stack.last().unwrap();
        let next = g
            .edges_directed(cur, Outgoing)
            .find(|e| !traversed.contains(&combine_hash(hash, e.id().index())));
        match next {
            None => {
                stack.pop();
                acc += mul;
                println!("ascending from {:?}", lookup.get_by_right(&cur));
            }
            Some(e) => {
                let hash = combine_hash(hash, e.id().index());
                traversed.insert(hash);
                let w = *e.weight();
                stack.push((mul * w, hash, e.target()));
                println!(
                    "descending from {:?} to {:?} x {}",
                    lookup.get_by_right(&cur),
                    lookup.get_by_right(&e.target()),
                    w
                );
            }
        };
    }

    acc - 1
}

fn combine_hash(cur: u64, input: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(cur);
    hasher.write_usize(input);
    hasher.finish()
}

type Line = (Colour, Vec<(u64, Colour)>);

fn line(input: &str) -> nom::IResult<&str, Line> {
    let (input, parent) = colour(input)?;
    let (input, _) = tag(" bags contain ")(input)?;

    let (input, links) = alt((
        map(tag("no other bags."), |_| Vec::new()),
        terminated(separated_list1(tag(", "), link), tag(".")),
    ))(input)?;

    Ok((input, (parent, links)))
}

fn link(input: &str) -> nom::IResult<&str, (u64, Colour)> {
    terminated(
        separated_pair(map_res(digit1, u64::from_str), tag(" "), colour),
        tuple((tag(" bag"), opt(tag("s")))),
    )(input)
}

fn colour(input: &str) -> nom::IResult<&str, Colour> {
    let (input, (descriptor, name)) = separated_pair(alpha1, space1, alpha1)(input)?;
    Ok((input, Colour::new(descriptor, name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_colours() {
        let tests = vec![
            ("deep blue", Colour::new("deep", "blue")),
            ("matt black", Colour::new("matt", "black")),
            ("john green", Colour::new("john", "green")),
        ];
        for (input, want) in tests {
            assert_eq!(colour(input), Ok(("", want)), "{}", input)
        }
    }

    #[test]
    fn parse_lines() {
        let tests = vec![
            (
                "bright white bags contain 1 shiny gold bag.",
                (
                    Colour::new("bright", "white"),
                    vec![(1, Colour::new("shiny", "gold"))],
                ),
            ),
            (
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                (
                    Colour::new("vibrant", "plum"),
                    vec![
                        (5, Colour::new("faded", "blue")),
                        (6, Colour::new("dotted", "black")),
                    ],
                ),
            ),
            (
                "faded blue bags contain no other bags.",
                (Colour::new("faded", "blue"), vec![]),
            ),
        ];

        for (input, want) in tests {
            assert_eq!(line(input), Ok(("", want)), "{}", input)
        }
    }

    #[test]
    fn part1() {
        let inputs: Vec<String> =
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#
                .lines()
                .map(String::from)
                .collect();

        assert_eq!(super::part1(&inputs), 4);
    }

    #[test]
    fn part2() {
        let inputs: Vec<String> = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#
            .lines()
            .map(String::from)
            .collect();

        assert_eq!(super::part2(&inputs), 126);
    }
}
