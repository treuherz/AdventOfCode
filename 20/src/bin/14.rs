use std::convert::TryInto;
use std::time::Instant;

use bitvec::prelude::*;
use itertools::Itertools;
use nom::lib::std::str::FromStr;
use regex::Regex;

use anyhow::anyhow;
use aoc20::util::{parse, print_answers};
use std::str::from_utf8;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/14")?;
    print_answers(14, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

type Bits = bitarr!(for 36, in Msb0, u8);

enum Instruction {
    Mask { or: Bits, and: Bits },
    Mem { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let mask_str: &str = from_utf8(&s.as_bytes()[7..43])?;
            let mut or_mask = bitarr![Msb0, u8; 0; 36];
            let mut and_mask = bitarr![Msb0, u8; 1; 36];

            for (i, c) in mask_str.char_indices() {
                match c {
                    'X' => continue,
                    '0' => and_mask.set(i, false),
                    '1' => or_mask.set(i, true),
                    _ => return Err(anyhow!("unrecognised mask value")),
                }
            }

            Ok(Instruction::Mask { or: or_mask, and: and_mask })
        } else if s.starts_with("mem") {
            let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            let caps = re.captures(s).unwrap();
            let address: u64 = caps[1].parse()?;
            let value: u64 = caps[2].parse()?;

            Ok(Instruction::Mem { address, value })
        } else {
            Err(anyhow!("unrecognised instruction"))
        }
    }
}

fn part1(inputs: &[String]) -> u64 {
    todo!()
}

fn part2(inputs: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    #[test]
    fn bit_assign() {
        let mut store = bitarr![Msb0, u8; 0; 36];
        dbg!(store);
    }

    #[test]
    fn bits() {
        println!("{}", (2u64.pow(36)).view_bits::<Msb0>());
    }
}
