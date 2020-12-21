use std::{collections::HashMap, str::FromStr, time::Instant};

use aoc20::util::{parse, print_answers};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alpha1, complete::digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult, Parser,
};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/19")?;
    print_answers(19, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

type Idx = usize;

#[derive(Debug)]
enum Rule {
    Char(char),
    Concat(Vec<Idx>),
    AltConcat(Vec<Idx>, Vec<Idx>),
}

struct Ruleset {
    rules: HashMap<Idx, Rule>,
}

impl Ruleset {
    fn new(inputs: &[String]) -> Ruleset {
        let mut rules = HashMap::with_capacity(inputs.len());
        for s in inputs {
            let (key, rule) = parse_line(&s).finish().expect("parse failure").1;
            rules.insert(key, rule);
        }
        Ruleset { rules }
    }

    fn attempt<'a>(&self, input: &'a str, idx: &Idx) -> Option<&'a str> {
        let rule = &self.rules[idx];
        let res = match rule {
            Rule::Char(c) => input.strip_prefix(*c),
            Rule::Concat(v) => v.iter().fold(Some(input), |o, idx| {
                o.and_then(|rem| self.attempt(rem, idx))
            }),
            Rule::AltConcat(u, v) => u
                .iter()
                .fold(Some(input), |o, idx| {
                    o.and_then(|rem| self.attempt(rem, idx))
                })
                .or_else(|| {
                    v.iter().fold(Some(input), |o, idx| {
                        o.and_then(|rem| self.attempt(rem, idx))
                    })
                }),
        };
        res
    }

    fn matches(&self, input: &str, idx: &Idx) -> bool {
        self.attempt(input, idx)
            .filter(|rem| rem.is_empty())
            .is_some()
    }
}

/// Index integer e.g. '123'
fn idx(input: &str) -> IResult<&str, Idx> {
    map_res(digit1, Idx::from_str)(input)
}

/// Two numbers, e.g. '1 2'
fn concat(input: &str) -> IResult<&str, Vec<Idx>> {
    separated_list1(tag(" "), idx)(input)
}

/// Separated paris of numbers,  e.g. '1 2 | 3 4'
fn alt_concat(input: &str) -> IResult<&str, (Vec<Idx>, Vec<Idx>)> {
    separated_pair(concat, tag(" | "), concat)(input)
}

fn literal(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), alpha1, tag("\""))(input)
}

fn parse_line(input: &str) -> IResult<&str, (Idx, Rule)> {
    separated_pair(
        // Key index
        idx,
        tag(": "),
        // Rule proper
        alt((
            literal.map(|s| Rule::Char(s.chars().next().unwrap())),
            alt_concat.map(|(u, v)| Rule::AltConcat(u, v)),
            concat.map(Rule::Concat),
        )),
    )(input)
}

fn part1(inputs: &[String]) -> usize {
    let split = inputs.iter().position(|s| s.is_empty()).unwrap();
    let rules = Ruleset::new(&inputs[..split]);

    inputs[split + 1..]
        .iter()
        .map(|l| rules.matches(l, &0))
        .filter(|b| *b)
        .count()
}

fn part2(inputs: &[String]) -> usize {
    let split = inputs.iter().position(|s| s.is_empty()).unwrap();
    let rules = Ruleset::new(&inputs[..split]);

    let mut count = 0;
    'lines: for line in &inputs[split + 1..] {
        let mut input = line.as_str();

        let mut count42 = 0;
        while let Some(rem) = rules.attempt(&input, &42) {
            input = rem;
            count42 += 1;
        }

        // Early continue if it didn't match at all
        if count42 == 0 {
            continue;
        }

        // Try parsing the last part of the string with 31s
        for n in (2..=count42).rev() {
            let mut input = line.as_str();
            // First consume as many 42s as possible
            for _ in 1..=n {
                input = rules.attempt(input, &42).unwrap()
            }

            // Then see if we can parse the rest of the line with 31s, so long
            // as there are fewer 31s than 42s
            let mut count31 = 0;
            for _ in 1..n {
                if let Some(rem) = rules.attempt(&input, &31) {
                    count31 += 1;
                    input = rem;
                } else {
                    break;
                }
            }

            if input.is_empty() && count31 != 0 {
                count += 1;
                println!("PASSES with {} x 42, {} x 31: {}", n, count31, line);
                continue 'lines;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let inputs: Vec<String> = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(super::part1(&inputs), 2)
    }

    #[test]
    fn part2() {
        let input: Vec<String> = vec![
            "0: 8 11",
            "1: \"a\"",
            "2: 1 24 | 14 4",
            "3: 5 14 | 16 1",
            "4: 1 1",
            "5: 1 14 | 15 1",
            "6: 14 14 | 1 14",
            "7: 14 5 | 1 21",
            "8: 42",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "11: 42 31",
            "12: 24 14 | 19 1",
            "13: 14 3 | 1 12",
            "14: \"b\"",
            "15: 1 | 14",
            "16: 15 1 | 14 14",
            "17: 14 2 | 1 7",
            "18: 15 15",
            "19: 14 1 | 14 14",
            "20: 14 14 | 1 15",
            "21: 14 1 | 1 14",
            "22: 14 14",
            "23: 25 1 | 22 14",
            "24: 14 1",
            "25: 1 1 | 1 14",
            "26: 14 22 | 1 20",
            "27: 1 6 | 14 18",
            "28: 16 1",
            "31: 14 17 | 1 13",
            "42: 9 14 | 10 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        // assert_eq!(super::part1(&input), 3);
        assert_eq!(super::part2(&input), 12);
    }
}
