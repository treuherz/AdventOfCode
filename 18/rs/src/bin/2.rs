#![feature(slice_partition_dedup)]

use std::collections::BTreeMap;
use std::ops::Add;

use aoc18::util::{parse, print_ans};

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = parse("inputs/2")?;
    print_ans(&inputs, f1, f2);
    Ok(())
}

#[derive(Debug)]
struct TwosAndThrees {
    pub twos: i32,
    pub threes: i32,
}

impl From<&BTreeMap<char, i32>> for TwosAndThrees {
    fn from(map: &BTreeMap<char, i32>) -> TwosAndThrees {
        let (twos, threes) = map.values().fold((0, 0), |(twos, threes), count| {
            match count {
                2 => (1, threes),
                3 => (twos, 1),
                _ => (twos, threes),
            }
        });
        TwosAndThrees { twos, threes }
    }
}

impl From<&String> for TwosAndThrees {
    fn from(string: &String) -> TwosAndThrees {
        TwosAndThrees::from(&char_count(&string))
    }
}

impl Add for TwosAndThrees {
    type Output = TwosAndThrees;

    fn add(self, other: TwosAndThrees) -> TwosAndThrees {
        TwosAndThrees {
            twos: self.twos + other.twos,
            threes: self.threes + other.threes,
        }
    }
}

fn char_count(input: &str) -> BTreeMap<char, i32> {
    input.chars().fold(BTreeMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn f1(inputs: &[String]) -> i32 {
    let twos_and_threes = inputs
        .iter()
        .fold(TwosAndThrees { twos: 0, threes: 0 }, |acc, string| {
            acc + TwosAndThrees::from(string)
        });
    twos_and_threes.twos * twos_and_threes.threes
}

fn f2(inputs: &[String]) -> String {
    for i in 0..inputs.first().unwrap().len() {
        let mut working = inputs
            .iter()
            .map(|s| {
                let mut ss = s.clone();
                ss.remove(i);
                ss
            })
            .collect::<Vec<String>>();
        working.sort_unstable();
        let (_, dupes) = working.partition_dedup();
        if !dupes.is_empty() {
            assert_eq!(dupes.len(), 1);
            return dupes.first().unwrap().to_string();
        }
    }
    String::new()
}
